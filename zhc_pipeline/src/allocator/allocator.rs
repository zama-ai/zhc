use crate::allocator::{
    batch_map::BatchMap,
    heap::{Heap, HeapSlot},
    live_range::{LiveRangeMap, TimePoint},
    register_file::{RegFile, RegId, RegRangeId},
    register_state::RegState,
    value_state::ValState,
};
use zhc_ir::{IR, OpMap, OpRef, ValId, ValMap};
use zhc_langs::hpulang::{HpuInstructionSet, HpuLang};
use zhc_utils::{
    iter::{CollectInSmallVec, Intermediate, MultiZip, ReconcilerOf3},
    small::SmallVec,
    svec,
};

static TRACE_EXECUTION: bool = false;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Spill {
    pub from: RegId,
    pub to: HeapSlot,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Unspill {
    pub from: HeapSlot,
    pub to: RegId,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Alloc {
    pub spills: SmallVec<Spill>,
    pub unspills: SmallVec<Unspill>,
    pub srcs: SmallVec<RegId>,
    pub dsts: SmallVec<RegId>,
    pub slots: SmallVec<HeapSlot>
}

pub struct Allocator<'ir> {
    input: &'ir IR<HpuLang>,
    live_ranges: LiveRangeMap,
    map: ValMap<ValState>,
    register_file: RegFile,
    heap: Heap,
    current_point: TimePoint,
    end_point: TimePoint,
}

impl<'ir> Allocator<'ir> {
    pub fn init(ir: &IR<HpuLang>, nregs: usize) -> Allocator<'_> {
        let live_ranges = LiveRangeMap::from_scheduled_ir(ir);
        let register_file = RegFile::empty(nregs);
        let map = ir.filled_valmap(ValState::Unseen);
        let input = ir;
        let current_point = 0;
        let end_point = ir.n_ops();
        let heap = Heap::empty();
        Allocator {
            input,
            live_ranges,
            register_file,
            map,
            current_point,
            end_point,
            heap,
        }
    }

    fn pick_reg_for_src_eviction(&mut self) -> RegId {
        let (rid, _) = self
            .register_file
            .iter_registers()
            .filter(|(_, rs)| match rs {
                RegState::Storing(valid) => {
                    !self.live_ranges[*valid].is_used_at(self.current_point)
                }
                _ => false,
            })
            .max_by_key(|(_, rs)| {
                let RegState::Storing(valid) = rs else {
                    unreachable!()
                };
                self.live_ranges[*valid]
                    .next_use(self.current_point)
                    .unwrap_or(self.end_point) as usize
            })
            .expect("Failed to encounter a compatible register.");
        rid
    }

    fn pick_regs_for_dst_eviction(&mut self, range_size: u8, is_batch: bool) -> RegRangeId {
        let (rrid, _) = self
            .register_file
            .iter_register_ranges(range_size)
            .filter(|(_, range)| {
                range.iter().all(|r| match r {
                    RegState::Empty => true,
                    RegState::Storing(valid) => {
                        !self.live_ranges[*valid].is_used_at(self.current_point)
                    }
                    RegState::Retiring(_) => !is_batch,
                    _ => false,
                })
            })
            .max_by_key(|(_, range)| {
                range
                    .iter()
                    .filter_map(|a| match a {
                        RegState::Storing(valid) | RegState::Retiring(valid) => Some(valid),
                        _ => None,
                    })
                    .map(|valid| {
                        self.live_ranges[*valid]
                            .next_use(self.current_point)
                            .unwrap_or(self.end_point) as usize
                    })
                    .sum::<usize>()
            })
            .expect("Failed to encounter a compatible block.");
        rrid
    }

    fn retire_registers(&mut self) {
        let retiring = self
            .live_ranges
            .retiring_iter(self.current_point)
            .map(|valid| match self.map[valid] {
                ValState::Registered { reg } => reg,
                _ => panic!(
                    "Error while stepping. A retiring valid is not in register file: {:?}",
                    valid
                ),
            });
        for to_retire in retiring {
            self.register_file[to_retire].retire();
        }
    }

    fn acquire_heap_slots(
        &mut self,
        op: OpRef<'ir, HpuLang>,
    ) -> SmallVec<HeapSlot> {

        use HpuInstructionSet::*;
        match op.get_instruction() {
            TransferIn { tid } | TransferOut { tid }=> svec![self.heap.get_unmapped()],
            _ => svec![]
        }
    }


    fn acquire_src_registers(
        &mut self,
        op: OpRef<'ir, HpuLang>,
    ) -> (SmallVec<Spill>, SmallVec<Unspill>) {
        let mut spills = svec![];
        let mut unspills = svec![];

        let to_unspill = op
            .get_arg_valids()
            .iter()
            .cloned()
            .filter(|v| self.map[*v].is_spilled())
            .intermediate();

        for valid_to_unspill in to_unspill {
            let maybe_avail = self
                .register_file
                .iter_registers()
                .find(|(_, rs)| rs.may_receive_unspill());
            let available = match maybe_avail {
                Some((ri, _)) => ri,
                None => {
                    let rid = self.pick_reg_for_src_eviction();
                    let evicted = self.register_file[rid].evict();
                    let slot = self.heap.get(&evicted);
                    self.map[evicted].spill(slot);
                    spills.push(Spill {
                        from: rid,
                        to: slot,
                    });
                    rid
                }
            };

            self.register_file[available].acquire_unspill(valid_to_unspill);
            let slot = self.map[valid_to_unspill].unspill(available);
            unspills.push(Unspill {
                from: slot,
                to: available,
            });
        }

        (spills, unspills)
    }

    fn acquire_dst_registers(&mut self, op: OpRef<'ir, HpuLang>) -> SmallVec<Spill> {
        let mut spills = svec![];
        let is_batch = op.get_instruction().is_batch();

        for val_range in get_ranges(op) {
            let range_size = val_range.len() as u8;
            let maybe_avail = self
                .register_file
                .iter_register_ranges(range_size)
                .find(|reg_range| reg_range.1.iter().all(|r| r.may_receive_dst(is_batch)))
                .map(|rr| rr.0);
            let available = match maybe_avail {
                Some(reg_range) => reg_range,
                None => {
                    let rrid = self.pick_regs_for_dst_eviction(range_size, is_batch);
                    for rid in rrid.rids_iter() {
                        if !self.register_file[rid].is_empty() {
                            let evicted = self.register_file[rid].evict();
                            let slot = self.heap.get(&evicted);
                            self.map[evicted].spill(slot);
                            spills.push(Spill {
                                from: rid,
                                to: slot,
                            });
                        }
                    }
                    rrid
                }
            };

            for (rid, valid) in (available.rids_iter(), val_range.iter()).mzip() {
                self.register_file[rid].acquire_dst(*valid);
                self.map[*valid].register(rid);
            }
        }
        spills
    }

    pub fn allocate_registers(mut self) -> OpMap<Alloc> {
        let mut output = self.input.empty_opmap();

        for op in self.input.walk_ops_linear().into_iter() {
            if TRACE_EXECUTION {
                eprintln!("{}", op.format().show_opid(true));
            }

            let (mut spills, unspills) = self.acquire_src_registers(op.clone());
            let slots = self.acquire_heap_slots(op.clone());
            self.retire_registers();
            let more_spills = self.acquire_dst_registers(op.clone());

            spills.extend(more_spills.into_iter());
            output.insert(
                *op,
                Alloc {
                    spills,
                    unspills,
                    srcs: op
                        .get_arg_valids()
                        .iter()
                        .map(|v| self.map[*v].rid())
                        .collect(),
                    dsts: op
                        .get_return_valids()
                        .iter()
                        .map(|v| self.map[*v].rid())
                        .collect(),
                    slots
                },
            );

            if TRACE_EXECUTION {
                eprintln!("Reg file: {}", self.register_file);
                eprintln!("  : Heap size: {}", self.heap.size());
            }

            self.register_file
                .iter_registers_mut()
                .for_each(|(_, rs)| match rs.stabilize() {
                    Some(valid) => self.map[valid].retire(),
                    None => {}
                });

            self.current_point += 1;
        }
        output
    }
}

fn get_ranges<'ir>(op: OpRef<'ir, HpuLang>) -> impl Iterator<Item = SmallVec<ValId>> + 'ir {
    use HpuInstructionSet::*;
    match op.get_instruction() {
        AddCt
        | SubCt
        | Mac { .. }
        | AddPt
        | SubPt
        | PtSub
        | MulPt
        | AddCst { .. }
        | SubCst { .. }
        | CstSub { .. }
        | MulCst { .. }
        | CstCt { .. }
        | SrcLd { .. }
        | TransferIn { .. } => std::iter::once(svec![op.get_return_valids()[0]]).reconcile_1_of_3(),

        ImmLd { .. } | DstSt { .. } | TransferOut { .. } => std::iter::empty().reconcile_2_of_3(),

        Batch { block } => {
            let batch_map = BatchMap::from_op(&op);
            block
                .walk_ops_linear()
                .filter_map(move |batch_op| match batch_op.get_instruction() {
                    Pbs { .. }
                    | PbsF { .. }
                    | Pbs2 { .. }
                    | Pbs2F { .. }
                    | Pbs4 { .. }
                    | Pbs4F { .. }
                    | Pbs8 { .. }
                    | Pbs8F { .. } => Some(
                        batch_op
                            .get_return_valids()
                            .iter()
                            .map(|a| batch_map[*a])
                            .cosvec(),
                    ),
                    BatchArg { .. } | BatchRet { .. } => None,
                    _ => unreachable!(),
                })
                .collect::<Vec<_>>()
                .into_iter()
                .reconcile_3_of_3()
        }

        _ => unreachable!(),
    }
}

// Notes:
// ======
//
// [1]: The Register File of the HPU comprises 64 registers as of today. This is comfortable, but (obviously) not big
// enough to hold all the intermediate values one may need during a large computation. For this
// reason, the register allocator also relies on a Heap, to spill values for the time being. That
// said, a spilled value must always be unspilled to a register before being used in a computation.
// That is, the PEA and PEP can not work with heap adresses.
//
// [2]: Because of the inherently limited and shared nature of the Register File, the register allocation will
// invariably introduce some spurious dependencies during the allocation process. By that, we mean
// that runtime dependencies which do not derive from the computation depdencies will appear. This
// is due to the fact that the HPU Instruction Scheduler ensures that within its scheduling horizon,
// all reads of a register have been performed before it can be written a new value. It ensure that
// no race condition appears at runtime, and that the computation stays correct despite an erroneous
// register allocation.
//
// [3]: The HPU executes operations in two different ways depending on the PE:
//  + Scalar execution: The operations executed by the PEA and the PEM are execute one by one.
//  + Batched execution: The operations executed by the PEP are executed many at once.
// In the case of batched execution, particular care must be taken to ensure that no spurious
// dependencies as the one explained in [2] are created _within_ a batch. This would have a
// catastrophic impact on the performance profile since:
//  + The PBS batch would not be as big as expected.
//  + The PEP would wait for the (arguably long) timeout before launching the amputed batch.
// Of paramount importance is our ability to NOT introduce batch dependencies during the register
// allocation. For this reason we treat batches of PBSes as a single operation (reason why the Batch
// operation exist in the HpuLang dialect). This allows to allocate all the registers for the same
// time point, ensuring that no register is used for two separate operands within the batch.
//
// [4]: On register retiring and immediate reuse. When executing scalar operations, assuming a source operand is read
// for the last time (Retiring), its register can also be used as destinatino operand to store the
// result of the computation. Given the atommic nature of the scalar computation, this will not
// create any lock in the scheduling of the operation itself. When executing a batched operation
// though, this is should not be done. Indeed, it could lead to the introduction of the exact
// spurious dependencies that we try to avoid. Indeed, assuming a value is read for the last time at
// the level of the batch (a batch op arg), it may still be read by multiple operations inside the
// batch. If its register is reused to store the result of a first operation of the batch, it will
// get stalled before all the others have been executed. This would indeed destroy the batch we
// prepared earlier in the pipeline. For this reason, we differentiate between allocation policy for
// Scalar operations and for Batched operations.
//
// [5]: Retiring of unspilled. When a value is unspilled to be read a last time (a current situation), its regiter can
// legitimately be used as the destination operand of the op. For this reason, during the
// allocation, the retiring is perforned _after_ the source operands have been unspilled.
//
// [6]: Register ranges. Most operation operands represent a single register. This is certainly true for source operands
// which always target a single (ciphertext) register in the register file. The results of Many-LUT
// PBSes require a different treatment though. The register passed as dst operand represent a
// contiguous register ranges of (power-of-two-many) outputs of their computation. In this case, a
// special care must be taken to ensure that all registers of the range are free to use.
