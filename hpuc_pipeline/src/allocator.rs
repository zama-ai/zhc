//! Register allocation for HPU operations.
//!
//! This module implements register allocation algorithms that assign physical
//! registers to virtual values in the scheduled intermediate representation.
//! The allocator handles register pressure, spilling to memory when necessary,
//! and produces device operation code with concrete register assignments.

use std::{
    fmt::{Debug, Display},
    ops::{Div, Index, Rem},
};

use hpuc_ir::{IR, OpId, OpIdRaw, OpRef, ValId, ValMap};
use hpuc_langs::{
    doplang::{Argument, Doplang, Operations as DopOp},
    hpulang::{Hpulang, Operations as HpuOp},
};
use hpuc_sim::hpu::HpuConfig;
use hpuc_utils::{CollectInSmallVec, CollectInVec, MultiZip, SmallMap, SmallVec, StoreIndex, svec};

/// A register identifier used in the allocation process.
#[derive(Clone, Debug, Copy)]
pub struct Register(OpIdRaw);

/// Represents the state of a register.
#[derive(Clone, Copy, Debug)]
enum RegisterState {
    /// The register does not hold any value
    Empty,
    /// The register holds a freshly added scalar value.
    Fresh(ValId),
    /// The register holds a scalar value.
    Storing(ValId),
}

impl Display for RegisterState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RegisterState::Empty => write!(f, "   "),
            RegisterState::Fresh(val_id) => write!(f, "\x1b[1m{:3}\x1b[0m", val_id.as_usize()),
            RegisterState::Storing(val_id) => write!(f, "{:3}", val_id.as_usize()),
        }
    }
}

impl RegisterState {
    pub fn is_empty(&self) -> bool {
        matches!(self, RegisterState::Empty)
    }

    pub fn is_fresh(&self) -> bool {
        matches!(self, RegisterState::Fresh(_))
    }

    pub fn is_storing(&self) -> bool {
        matches!(self, RegisterState::Storing(_))
    }

    pub fn ok_storing(&self) -> Option<ValId> {
        match self {
            RegisterState::Storing(valid) => Some(*valid),
            _ => None,
        }
    }

    pub fn use_fresh(&mut self) {
        let RegisterState::Fresh(valid) = self else {
            unreachable!()
        };
        *self = RegisterState::Storing(*valid);
    }

    pub fn free(&mut self) -> ValId {
        let RegisterState::Storing(valid) = self else {
            unreachable!("{:?}", self)
        };
        let output = *valid;
        *self = RegisterState::Empty;
        output
    }

    pub fn acquire(&mut self, valid: ValId) {
        let RegisterState::Empty = self else {
            unreachable!("{:?}", self)
        };
        *self = RegisterState::Fresh(valid);
    }
}

/// A structure representing a register file.
///
/// In our case, a regfile is a map from registers identifiers to optional values stored at a
/// certain point in time.
///
/// # Notes:
/// For many-lut pbses, we need to be able to store in contiguous ranges of the regfile (with proper
/// alignment). For this reasons, some methods take a const `RANGE_SIZE`, which represents the size
/// of the expected range to be targetted.
#[derive(Debug)]
struct RegisterFile(Vec<RegisterState>);

fn translate<const RANGE_SIZE: usize>(v: usize) -> Register {
    Register((v * RANGE_SIZE) as OpIdRaw)
}

impl RegisterFile {
    pub fn empty(size: usize) -> Self {
        RegisterFile(vec![RegisterState::Empty; size])
    }

    fn iter_ranges<const RANGE_SIZE: usize>(
        &self,
    ) -> impl Iterator<Item = (Register, impl Iterator<Item = RegisterState> + Clone)> {
        self.0
            .as_slice()
            .chunks_exact(RANGE_SIZE)
            .enumerate()
            .map(|(i, a)| (translate::<RANGE_SIZE>(i), a.iter().copied()))
    }

    fn get_range_at<const RANGE_SIZE: usize>(
        &self,
        first_reg: Register,
    ) -> impl Iterator<Item = RegisterState> + Clone {
        assert_eq!(first_reg.0.rem(RANGE_SIZE as u16), 0);
        let nth = (first_reg.0 as usize).div(RANGE_SIZE);
        self.iter_ranges::<RANGE_SIZE>().nth(nth).unwrap().1
    }

    pub fn may_insert<const RANGE_SIZE: usize>(&self) -> bool {
        self.iter_ranges::<RANGE_SIZE>()
            .any(|(_, mut a)| a.all(|a| a.is_empty()))
    }

    pub fn insert<const RANGE_SIZE: usize>(
        &mut self,
        vs: [ValId; RANGE_SIZE],
    ) -> [Register; RANGE_SIZE] {
        let maybe_reg_range = self
            .iter_ranges::<RANGE_SIZE>()
            .find_map(|(r, c)| c.clone().all(|a| a.is_empty()).then_some(r));
        let Some(Register(reg_range)) = maybe_reg_range else {
            unreachable!()
        };
        let result = std::array::from_fn(|i| {
            self.0[reg_range as usize + i].acquire(vs[i]);
            Register(reg_range + i as u16)
        });
        result
    }

    pub fn purge(&mut self, regs: impl Iterator<Item = Register>) {
        for rid in regs {
            self.0[rid.0 as usize].free();
        }
    }

    pub fn use_fresh(&mut self) {
        self.0
            .iter_mut()
            .filter(|r| r.is_fresh())
            .for_each(|r| r.use_fresh());
    }

    pub fn evict<const RANGE_SIZE: usize>(
        &mut self,
        point: OpIdRaw,
        live_map: &LiveRangeMap,
    ) -> impl Iterator<Item = ValId> {
        // We search the ranges that have the smallest number of active registers.
        // And avoid to evict a range with a newly attributed value register 🙄.
        let min_n = self
            .iter_ranges::<RANGE_SIZE>()
            .filter(|(_, range)| range.clone().all(|a| !a.is_fresh()))
            .map(|(_, range)| range.filter(|a| a.is_storing()).count())
            .min()
            .unwrap();
        let compatible_blocks = self
            .iter_ranges::<RANGE_SIZE>()
            .filter(|(_, range)| range.clone().filter(|a| a.is_storing()).count() == min_n);

        // Now, for those ranges, we compute the sum of distance to next uses
        let block_uses_sum = compatible_blocks.map(|(i, b)| {
            (
                i,
                b.filter_map(|a| a.ok_storing())
                    .map(|r| live_map.next_use_of(point, r).unwrap_or(point) as usize)
                    .sum::<usize>(),
            )
        });

        // Now we search the arg of this max cumulated use.
        let arg_max = block_uses_sum
            .max_by_key(|(_, a)| *a)
            .map(|(i, _)| i)
            .unwrap();

        // We collect the evicted valids
        let output = self
            .get_range_at::<RANGE_SIZE>(arg_max)
            .filter_map(|a| a.ok_storing())
            .cosvec();

        // We evict
        let id = arg_max.0 as usize;
        for reg in id..(id + RANGE_SIZE) {
            if self.0[reg].is_empty() {
                continue;
            } else {
                self.0[reg].free();
            }
        }

        output.into_iter()
    }
}

impl Display for RegisterFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<[|")?;
        for i in self.0.iter() {
            write!(f, " {}|", i)?;
        }
        write!(f, "]>")
    }
}

#[derive(Clone, Debug)]
struct LiveRange {
    init: OpIdRaw,
    uses: SmallVec<OpIdRaw>,
}

impl LiveRange {
    pub fn to(&self) -> OpIdRaw {
        // The uses are ordered by construction, so the max is the last one.
        *self.uses.iter().last().unwrap_or(&self.init)
    }

    pub fn next_use(&self, point: OpIdRaw) -> Option<OpIdRaw> {
        // The uses are ordered by construction, so the first use that is greater or equal to now is
        // the next use.
        self.uses.iter().copied().find(|u| *u >= point)
    }
}

#[derive(Debug)]
struct LiveRangeMap(ValMap<LiveRange>);

impl LiveRangeMap {
    pub fn from_scheduled_ir(ir: &IR<Hpulang>) -> Self {
        let mut live_ranges: ValMap<LiveRange> = ir.empty_valmap();
        for (point, op) in ir.walk_ops_linear().enumerate() {
            for val in op.get_args_iter() {
                live_ranges
                    .get_mut(&val.get_id())
                    .unwrap()
                    .uses
                    .push(point as OpIdRaw);
            }
            for val in op.get_returns_iter() {
                live_ranges.insert(
                    val.get_id(),
                    LiveRange {
                        init: point as OpIdRaw,
                        uses: SmallVec::new(),
                    },
                );
            }
        }
        LiveRangeMap(live_ranges)
    }

    pub fn next_use_of(&self, point: OpIdRaw, valid: ValId) -> Option<OpIdRaw> {
        self.0[valid].next_use(point)
    }

    pub fn purgeable_iter(&self, point: OpIdRaw) -> impl Iterator<Item = ValId> {
        self.0
            .iter()
            .filter(move |(_, live_range)| live_range.to() == point)
            .map(|(valid, _)| valid)
    }
}

#[derive(Clone, Debug)]
enum ValState {
    Registered { reg: Register },
    Spilled { slot: HeapSlot },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct HeapSlot(u16);

#[derive(Clone, Debug)]
struct Heap {
    slots: SmallMap<ValId, HeapSlot>,
    last: HeapSlot,
}

impl Heap {
    pub fn empty() -> Self {
        Heap {
            slots: SmallMap::new(),
            last: HeapSlot(0),
        }
    }

    fn push(&mut self, valid: ValId) -> HeapSlot {
        self.slots.insert(valid, self.last);
        let next = HeapSlot(self.last.0 + 1);
        std::mem::replace(&mut self.last, next)
    }

    pub fn contains(&self, valid: &ValId) -> bool {
        self.slots.get(valid).is_some()
    }

    pub fn get(&mut self, valid: &ValId) -> Result<HeapSlot, HeapSlot> {
        if !self.contains(valid) {
            Err(self.push(*valid))
        } else {
            Ok(*self.slots.get(valid).unwrap())
        }
    }
}

impl Display for Heap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "((|")?;
        for (k, _) in self.slots.iter() {
            write!(f, " {}|", k.as_usize())?;
        }
        write!(f, "))")
    }
}

struct BatchMap(SmallMap<ValId, ValId>);

impl BatchMap {
    pub fn from_op(op: &OpRef<Hpulang>) -> Self {
        let args = op.get_arg_valids();
        let rets = op.get_return_valids();
        let mut map = SmallMap::<ValId, ValId>::new();
        let HpuOp::Batch { block } = op.get_operation() else {
            unreachable!()
        };
        let mut ordered_batch_arg_valids = block
            .walk_ops_linear()
            .filter(|op| matches!(op.get_operation(), HpuOp::BatchArg { .. }))
            .covec();
        ordered_batch_arg_valids.sort_unstable_by_key(|op| {
            let HpuOp::BatchArg { pos, .. } = op.get_operation() else {
                unreachable!()
            };
            pos
        });
        let mut ordered_batch_ret_valids = block
            .walk_ops_linear()
            .filter(|op| matches!(op.get_operation(), HpuOp::BatchRet { .. }))
            .covec();
        ordered_batch_ret_valids.sort_unstable_by_key(|op| {
            let HpuOp::BatchRet { pos, .. } = op.get_operation() else {
                unreachable!()
            };
            pos
        });
        for (outer_valid, inner_valid) in (
            args.iter(),
            ordered_batch_arg_valids
                .into_iter()
                .map(|a| a.get_return_valids()[0]),
        )
            .mzip()
        {
            map.insert(inner_valid, *outer_valid);
        }
        for (outer_valid, inner_valid) in (
            rets.iter(),
            ordered_batch_ret_valids
                .into_iter()
                .map(|a| a.get_arg_valids()[0]),
        )
            .mzip()
        {
            map.insert(inner_valid, *outer_valid);
        }
        BatchMap(map)
    }
}

impl Index<ValId> for BatchMap {
    type Output = ValId;

    fn index(&self, index: ValId) -> &Self::Output {
        self.0.get(&index).unwrap()
    }
}

/// # Note:
///
/// The allocator directly emits an ir in the doplang dialect. This means that no further scheduling
/// will be performed on the spilled instructions. If register pressure turns out to be a big
/// contender, it may make sense to try to better schedule the spills (to lift them slightly up in
/// the stream, ensuring that no time is spent waiting for them).
struct Allocator<'ir> {
    input: &'ir IR<Hpulang>,
    output: IR<Doplang>,
    live_ranges: LiveRangeMap,
    register_file: RegisterFile,
    heap: Heap,
    translation_map: ValMap<ValState>,
    current_ctx: ValId,
    point: OpIdRaw,
}

impl<'ir> Allocator<'ir> {
    pub fn init(ir: &IR<Hpulang>, nregs: usize) -> Allocator {
        let live_ranges = LiveRangeMap::from_scheduled_ir(ir);
        let register_file = RegisterFile::empty(nregs);
        let translation_map = ir.empty_valmap();
        let input = ir;
        let mut output = IR::empty();
        let (_, rets) = output.add_op(DopOp::_INIT, svec![]).unwrap();
        let current_ctx = rets[0];
        let point = 0;
        let heap = Heap::empty();
        Allocator {
            input,
            output,
            live_ranges,
            register_file,
            translation_map,
            current_ctx,
            point,
            heap,
        }
    }

    fn add_dop(&mut self, dop: DopOp) -> OpId {
        let (opid, rets) = self.output.add_op(dop, svec![self.current_ctx]).unwrap();
        self.current_ctx = rets[0];
        opid
    }

    fn add_spill(&mut self, valid: ValId) {
        let ValState::Registered { reg } = self.translation_map[valid] else {
            unreachable!()
        };
        let slot = match self.heap.get(&valid) {
            Ok(hs) => hs,
            Err(hs) => {
                self.add_dop(DopOp::ST {
                    dst: Argument::ct_heap(hs.0 as usize),
                    src: Argument::ct_reg(reg.0 as usize),
                });
                hs
            }
        };
        self.translation_map
            .insert(valid, ValState::Spilled { slot });
    }

    fn get_register<const BLOCK_SIZE: usize>(
        &mut self,
        vs: [ValId; BLOCK_SIZE],
    ) -> [Register; BLOCK_SIZE] {
        if !self.register_file.may_insert::<BLOCK_SIZE>() {
            for valid in self
                .register_file
                .evict::<BLOCK_SIZE>(self.point, &self.live_ranges)
                .cosvec()
                .into_iter()
            {
                self.add_spill(valid);
            }
        }
        let output = self.register_file.insert(vs.clone());
        for (valid, reg) in vs.iter().zip(output.iter()) {
            self.translation_map
                .insert(*valid, ValState::Registered { reg: *reg });
        }
        output
    }

    fn get_dst_registers<const BLOCK_SIZE: usize>(
        &mut self,
        vs: [ValId; BLOCK_SIZE],
    ) -> [Register; BLOCK_SIZE] {
        self.get_register(vs)
    }

    fn get_src_register(&mut self, vs: ValId) -> Register {
        match self.translation_map.get(&vs).cloned().unwrap() {
            ValState::Registered { reg } => reg,
            ValState::Spilled { slot } => {
                let [r] = self.get_register([vs]);
                self.add_dop(DopOp::LD {
                    dst: Argument::ct_reg(r.0),
                    src: Argument::ct_heap(slot.0 as usize),
                });
                r
            }
        }
    }

    pub fn allocate_registers(mut self) -> IR<Doplang> {
        for op in self.input.walk_ops_linear().covec().into_iter() {
            let args = op.get_arg_valids();
            let rets = op.get_return_valids();

            match op.get_operation() {
                HpuOp::SrcLd { from } => {
                    let [r_dst] = self.get_dst_registers([rets[0]]);
                    self.add_dop(DopOp::LD {
                        dst: Argument::ct_reg(r_dst.0),
                        src: Argument::ct_var(
                            from.src_pos.try_into().unwrap(),
                            from.block_pos.try_into().unwrap(),
                        ),
                    });
                }
                HpuOp::DstSt { to } => {
                    let r_src = self.get_src_register(args[0]);
                    self.add_dop(DopOp::ST {
                        src: Argument::ct_reg(r_src.0),
                        dst: Argument::ct_var(
                            to.dst_pos.try_into().unwrap(),
                            to.block_pos.try_into().unwrap(),
                        ),
                    });
                }
                HpuOp::ImmLd { .. } => {
                    // This is a no-op in the doplang dialect.
                    // Handled in Pt operations.
                }
                HpuOp::AddCt => {
                    let [r_dst] = self.get_dst_registers([rets[0]]);
                    let r_src1 = self.get_src_register(args[0]);
                    let r_src2 = self.get_src_register(args[1]);
                    self.add_dop(DopOp::ADD {
                        dst: Argument::ct_reg(r_dst.0),
                        src1: Argument::ct_reg(r_src1.0),
                        src2: Argument::ct_reg(r_src2.0),
                    });
                }
                HpuOp::SubCt => {
                    let [r_dst] = self.get_dst_registers([rets[0]]);
                    let r_src1 = self.get_src_register(args[0]);
                    let r_src2 = self.get_src_register(args[1]);
                    self.add_dop(DopOp::SUB {
                        dst: Argument::ct_reg(r_dst.0),
                        src1: Argument::ct_reg(r_src1.0),
                        src2: Argument::ct_reg(r_src2.0),
                    });
                }
                HpuOp::Mac { cst } => {
                    let [r_dst] = self.get_dst_registers([rets[0]]);
                    let r_src1 = self.get_src_register(args[0]);
                    let r_src2 = self.get_src_register(args[1]);
                    self.add_dop(DopOp::MAC {
                        dst: Argument::ct_reg(r_dst.0),
                        src1: Argument::ct_reg(r_src1.0),
                        src2: Argument::ct_reg(r_src2.0),
                        cst: Argument::pt_const(cst.0),
                    });
                }
                HpuOp::AddPt => {
                    let [r_dst] = self.get_dst_registers([rets[0]]);
                    let r_src = self.get_src_register(args[0]);
                    let imm_ld_op = self.input.get_val(args[1]).get_origin().get_operation();
                    let HpuOp::ImmLd { from } = imm_ld_op else {
                        unreachable!()
                    };
                    self.add_dop(DopOp::ADDS {
                        dst: Argument::ct_reg(r_dst.0),
                        src: Argument::ct_reg(r_src.0),
                        cst: Argument::pt_var(
                            from.imm_pos.try_into().unwrap(),
                            from.block_pos.try_into().unwrap(),
                        ),
                    });
                }
                HpuOp::SubPt => {
                    let [r_dst] = self.get_dst_registers([rets[0]]);
                    let r_src = self.get_src_register(args[0]);
                    let imm_ld_op = self.input.get_val(args[1]).get_origin().get_operation();
                    let HpuOp::ImmLd { from } = imm_ld_op else {
                        unreachable!()
                    };
                    self.add_dop(DopOp::SUBS {
                        dst: Argument::ct_reg(r_dst.0),
                        src: Argument::ct_reg(r_src.0),
                        cst: Argument::pt_var(
                            from.imm_pos.try_into().unwrap(),
                            from.block_pos.try_into().unwrap(),
                        ),
                    });
                }
                HpuOp::PtSub => {
                    let [r_dst] = self.get_dst_registers([rets[0]]);
                    let r_src = self.get_src_register(args[1]);
                    let imm_ld_op = self.input.get_val(args[0]).get_origin().get_operation();
                    let HpuOp::ImmLd { from } = imm_ld_op else {
                        unreachable!()
                    };
                    self.add_dop(DopOp::SSUB {
                        dst: Argument::ct_reg(r_dst.0),
                        src: Argument::ct_reg(r_src.0),
                        cst: Argument::pt_var(
                            from.imm_pos.try_into().unwrap(),
                            from.block_pos.try_into().unwrap(),
                        ),
                    });
                }
                HpuOp::MulPt => {
                    let [r_dst] = self.get_dst_registers([rets[0]]);
                    let r_src = self.get_src_register(args[0]);
                    let imm_ld_op = self.input.get_val(args[1]).get_origin().get_operation();
                    let HpuOp::ImmLd { from } = imm_ld_op else {
                        unreachable!()
                    };
                    self.add_dop(DopOp::MULS {
                        dst: Argument::ct_reg(r_dst.0),
                        src: Argument::ct_reg(r_src.0),
                        cst: Argument::pt_var(
                            from.imm_pos.try_into().unwrap(),
                            from.block_pos.try_into().unwrap(),
                        ),
                    });
                }
                HpuOp::AddCst { cst } => {
                    let [r_dst] = self.get_dst_registers([rets[0]]);
                    let r_src = self.get_src_register(args[0]);
                    self.add_dop(DopOp::ADDS {
                        dst: Argument::ct_reg(r_dst.0),
                        src: Argument::ct_reg(r_src.0),
                        cst: Argument::pt_const(cst.0),
                    });
                }
                HpuOp::SubCst { cst } => {
                    let [r_dst] = self.get_dst_registers([rets[0]]);
                    let r_src = self.get_src_register(args[0]);
                    self.add_dop(DopOp::SUBS {
                        dst: Argument::ct_reg(r_dst.0),
                        src: Argument::ct_reg(r_src.0),
                        cst: Argument::pt_const(cst.0),
                    });
                }
                HpuOp::CstSub { cst } => {
                    let [r_dst] = self.get_dst_registers([rets[0]]);
                    let r_src = self.get_src_register(args[0]);
                    self.add_dop(DopOp::SSUB {
                        dst: Argument::ct_reg(r_dst.0),
                        src: Argument::ct_reg(r_src.0),
                        cst: Argument::pt_const(cst.0),
                    });
                }
                HpuOp::MulCst { cst } => {
                    let [r_dst] = self.get_dst_registers([rets[0]]);
                    let r_src = self.get_src_register(args[0]);
                    self.add_dop(DopOp::MULS {
                        dst: Argument::ct_reg(r_dst.0),
                        src: Argument::ct_reg(r_src.0),
                        cst: Argument::pt_const(cst.0),
                    });
                }
                HpuOp::Batch { block } => {
                    let map = BatchMap::from_op(&op);
                    for op in block.walk_ops_linear() {
                        let rets = op.get_return_valids();
                        let args = op.get_arg_valids();
                        match op.get_operation() {
                            HpuOp::Pbs { lut } => {
                                let [r_dst] = self.get_dst_registers([map[rets[0]]]);
                                let r_src = self.get_src_register(map[args[0]]);
                                self.add_dop(DopOp::PBS {
                                    dst: Argument::ct_reg(r_dst.0),
                                    src: Argument::ct_reg(r_src.0),
                                    lut: Argument::lut_id(lut),
                                });
                            }
                            HpuOp::PbsF { lut } => {
                                let [r_dst] = self.get_dst_registers([map[rets[0]]]);
                                let r_src = self.get_src_register(map[args[0]]);
                                self.add_dop(DopOp::PBS_F {
                                    dst: Argument::ct_reg(r_dst.0),
                                    src: Argument::ct_reg(r_src.0),
                                    lut: Argument::lut_id(lut),
                                });
                            }
                            HpuOp::Pbs2 { lut } => {
                                let [r_dst, ..] =
                                    self.get_dst_registers([map[rets[0]], map[rets[1]]]);
                                let r_src = self.get_src_register(map[args[0]]);
                                self.add_dop(DopOp::PBS_ML2 {
                                    dst: Argument::ct_reg2(r_dst.0),
                                    src: Argument::ct_reg(r_src.0),
                                    lut: Argument::lut_id(lut),
                                });
                            }
                            HpuOp::Pbs2F { lut } => {
                                let [r_dst, ..] =
                                    self.get_dst_registers([map[rets[0]], map[rets[1]]]);
                                let r_src = self.get_src_register(map[args[0]]);
                                self.add_dop(DopOp::PBS_ML2_F {
                                    dst: Argument::ct_reg2(r_dst.0),
                                    src: Argument::ct_reg(r_src.0),
                                    lut: Argument::lut_id(lut),
                                });
                            }
                            HpuOp::Pbs4 { lut } => {
                                let [r_dst, ..] = self.get_dst_registers([
                                    map[rets[0]],
                                    map[rets[1]],
                                    map[rets[2]],
                                    map[rets[3]],
                                ]);
                                let r_src = self.get_src_register(map[args[0]]);
                                self.add_dop(DopOp::PBS_ML4 {
                                    dst: Argument::ct_reg4(r_dst.0),
                                    src: Argument::ct_reg(r_src.0),
                                    lut: Argument::lut_id(lut),
                                });
                            }
                            HpuOp::Pbs4F { lut } => {
                                let [r_dst, ..] = self.get_dst_registers([
                                    map[rets[0]],
                                    map[rets[1]],
                                    map[rets[2]],
                                    map[rets[3]],
                                ]);
                                let r_src = self.get_src_register(map[args[0]]);
                                self.add_dop(DopOp::PBS_ML4_F {
                                    dst: Argument::ct_reg4(r_dst.0),
                                    src: Argument::ct_reg(r_src.0),
                                    lut: Argument::lut_id(lut),
                                });
                            }
                            HpuOp::Pbs8 { lut } => {
                                let [r_dst, ..] = self.get_dst_registers([
                                    map[rets[0]],
                                    map[rets[1]],
                                    map[rets[2]],
                                    map[rets[3]],
                                    map[rets[4]],
                                    map[rets[5]],
                                    map[rets[6]],
                                    map[rets[7]],
                                ]);
                                let r_src = self.get_src_register(map[args[0]]);
                                self.add_dop(DopOp::PBS_ML8 {
                                    dst: Argument::ct_reg8(r_dst.0),
                                    src: Argument::ct_reg(r_src.0),
                                    lut: Argument::lut_id(lut),
                                });
                            }
                            HpuOp::Pbs8F { lut } => {
                                let [r_dst, ..] = self.get_dst_registers([
                                    map[rets[0]],
                                    map[rets[1]],
                                    map[rets[2]],
                                    map[rets[3]],
                                    map[rets[4]],
                                    map[rets[5]],
                                    map[rets[6]],
                                    map[rets[7]],
                                ]);
                                let r_src = self.get_src_register(map[args[0]]);
                                self.add_dop(DopOp::PBS_ML8_F {
                                    dst: Argument::ct_reg8(r_dst.0),
                                    src: Argument::ct_reg(r_src.0),
                                    lut: Argument::lut_id(lut),
                                });
                            }
                            HpuOp::BatchArg { .. } | HpuOp::BatchRet { .. } => {}
                            _ => unreachable!("Encountered unexpected operation while allocating: {}", op.get_operation())
                        }
                    }
                }
                _ => unreachable!(
                    "Encountered unexpected operation while allocating: {}",
                    op.get_operation()
                ),
            }

            #[cfg(debug_assertions)]
            {
                println!("{}", op);
                println!("{}: {}", self.point, self.register_file);
            }

            self.register_file.use_fresh();
            self.register_file
                .purge(self.live_ranges.purgeable_iter(self.point).map(|valid| {
                    match self.translation_map[valid] {
                        ValState::Registered { reg } => reg,
                        ValState::Spilled { .. } => panic!(
                            "Error while stepping. A purgeable valid is not in register file: {:?}",
                            valid
                        ),
                    }
                }));
            self.point += 1;
        }
        self.output
    }
}

/// Allocates physical registers to values in the scheduled IR.
///
/// Takes a scheduled intermediate representation `ir` containing HPU operations
/// and the hardware configuration `config` to produce a new IR in the device
/// operation language with physical register assignments for all values.
pub fn allocate_registers(ir: &IR<Hpulang>, config: &HpuConfig) -> IR<Doplang> {
    let allocator = Allocator::init(ir, config.regf_size);
    allocator.allocate_registers()
}

#[cfg(test)]
mod test {
    use hpuc_ir::{IR, translation::Translator};
    use hpuc_langs::{doplang::Doplang, ioplang::Ioplang};
    use hpuc_sim::hpu::{HpuConfig, PhysicalConfig};

    use crate::{
        batcher::batch, scheduler::schedule, test::{get_add_ir, get_cmp_ir, get_sub_ir}, translation::IoplangToHpulang
    };

    use super::allocate_registers;

    fn pipeline(ir: &IR<Ioplang>) -> IR<Doplang> {
        let ir = IoplangToHpulang.translate(&ir);
        let config = HpuConfig::from(PhysicalConfig::gaussian_64b());
        let scheduled = schedule(&ir, &config);
        let batched = batch(&scheduled);
        let allocated = allocate_registers(&batched, &config);
        allocated
    }

    #[test]
    fn test_allocate_add_ir() {
        let ir = pipeline(&get_add_ir(16, 2, 2));
        ir.check_ir(
            "
            %0 : Ctx = _INIT();
            %1 : Ctx = LD<R(0), TC(0, 0)>(%0);
            %2 : Ctx = LD<R(1), TC(0, 1)>(%1);
            %3 : Ctx = LD<R(2), TC(0, 2)>(%2);
            %4 : Ctx = LD<R(3), TC(0, 3)>(%3);
            %5 : Ctx = LD<R(4), TC(0, 4)>(%4);
            %6 : Ctx = LD<R(5), TC(0, 5)>(%5);
            %7 : Ctx = LD<R(6), TC(0, 6)>(%6);
            %8 : Ctx = LD<R(7), TC(1, 0)>(%7);
            %9 : Ctx = ADD<R(8), R(0), R(7)>(%8);
            %10 : Ctx = LD<R(0), TC(1, 1)>(%9);
            %11 : Ctx = LD<R(7), TC(1, 2)>(%10);
            %12 : Ctx = LD<R(9), TC(1, 3)>(%11);
            %13 : Ctx = LD<R(10), TC(1, 4)>(%12);
            %14 : Ctx = ADD<R(11), R(1), R(0)>(%13);
            %15 : Ctx = LD<R(0), TC(1, 5)>(%14);
            %16 : Ctx = LD<R(1), TC(1, 6)>(%15);
            %17 : Ctx = ADD<R(12), R(2), R(7)>(%16);
            %18 : Ctx = ADD<R(2), R(3), R(9)>(%17);
            %19 : Ctx = ADD<R(3), R(4), R(10)>(%18);
            %20 : Ctx = ADD<R(4), R(5), R(0)>(%19);
            %21 : Ctx = ADD<R(0), R(6), R(1)>(%20);
            %22 : Ctx = PBS2<R(6, 2), R(8), LUT(26)>(%21);
            %23 : Ctx = PBS<R(1), R(11), LUT(47)>(%22);
            %24 : Ctx = PBS<R(5), R(12), LUT(48)>(%23);
            %25 : Ctx = PBS<R(9), R(2), LUT(49)>(%24);
            %26 : Ctx = PBS<R(10), R(3), LUT(47)>(%25);
            %27 : Ctx = PBS<R(13), R(4), LUT(48)>(%26);
            %28 : Ctx = PBSF<R(14), R(0), LUT(49)>(%27);
            %29 : Ctx = ADD<R(8), R(11), R(7)>(%28);
            %30 : Ctx = ST<TC(0, 0), R(6)>(%29);
            %31 : Ctx = ADD<R(6), R(1), R(7)>(%30);
            %32 : Ctx = ST<TC(0, 1), R(8)>(%31);
            %33 : Ctx = ADD<R(1), R(13), R(10)>(%32);
            %34 : Ctx = PBSF<R(7), R(6), LUT(44)>(%33);
            %35 : Ctx = ADD<R(8), R(5), R(6)>(%34);
            %36 : Ctx = ADD<R(5), R(14), R(1)>(%35);
            %37 : Ctx = PBSF<R(6), R(8), LUT(45)>(%36);
            %38 : Ctx = ADD<R(11), R(9), R(8)>(%37);
            %39 : Ctx = ADD<R(8), R(12), R(7)>(%38);
            %40 : Ctx = PBSF<R(7), R(11), LUT(46)>(%39);
            %41 : Ctx = ADD<R(9), R(2), R(6)>(%40);
            %42 : Ctx = ST<TC(0, 2), R(8)>(%41);
            %43 : Ctx = ST<TC(0, 3), R(9)>(%42);
            %44 : Ctx = ADD<R(2), R(10), R(7)>(%43);
            %45 : Ctx = ADD<R(6), R(1), R(7)>(%44);
            %46 : Ctx = ADD<R(1), R(5), R(7)>(%45);
            %47 : Ctx = PBS<R(5), R(2), LUT(46)>(%46);
            %48 : Ctx = PBS<R(7), R(6), LUT(44)>(%47);
            %49 : Ctx = PBSF<R(8), R(1), LUT(45)>(%48);
            %50 : Ctx = ADD<R(1), R(3), R(5)>(%49);
            %51 : Ctx = ADD<R(2), R(4), R(7)>(%50);
            %52 : Ctx = ST<TC(0, 4), R(1)>(%51);
            %53 : Ctx = ADD<R(1), R(0), R(8)>(%52);
            %54 : Ctx = ST<TC(0, 5), R(2)>(%53);
            %55 : Ctx = ST<TC(0, 6), R(1)>(%54);
            ",
        );
    }

    #[test]
    fn test_allocate_sub_ir() {
        let ir = pipeline(&get_sub_ir(16, 2, 2));
        ir.check_ir(
            "
            %0 : Ctx = _INIT();
            %1 : Ctx = LD<R(0), TC(0, 0)>(%0);
            %2 : Ctx = LD<R(1), TC(0, 1)>(%1);
            %3 : Ctx = LD<R(2), TC(0, 2)>(%2);
            %4 : Ctx = LD<R(3), TC(0, 3)>(%3);
            %5 : Ctx = LD<R(4), TC(0, 4)>(%4);
            %6 : Ctx = LD<R(5), TC(0, 5)>(%5);
            %7 : Ctx = LD<R(6), TC(0, 6)>(%6);
            %8 : Ctx = LD<R(7), TC(1, 0)>(%7);
            %9 : Ctx = SSUB<R(8), R(7), PT_I(3)>(%8);
            %10 : Ctx = LD<R(7), TC(1, 1)>(%9);
            %11 : Ctx = LD<R(9), TC(1, 2)>(%10);
            %12 : Ctx = LD<R(10), TC(1, 3)>(%11);
            %13 : Ctx = LD<R(11), TC(1, 4)>(%12);
            %14 : Ctx = SSUB<R(12), R(7), PT_I(3)>(%13);
            %15 : Ctx = LD<R(7), TC(1, 5)>(%14);
            %16 : Ctx = LD<R(13), TC(1, 6)>(%15);
            %17 : Ctx = SSUB<R(14), R(9), PT_I(3)>(%16);
            %18 : Ctx = SSUB<R(9), R(10), PT_I(3)>(%17);
            %19 : Ctx = SSUB<R(10), R(11), PT_I(3)>(%18);
            %20 : Ctx = SSUB<R(11), R(7), PT_I(3)>(%19);
            %21 : Ctx = SSUB<R(7), R(13), PT_I(3)>(%20);
            %22 : Ctx = ADD<R(13), R(0), R(8)>(%21);
            %23 : Ctx = ADD<R(0), R(1), R(12)>(%22);
            %24 : Ctx = ADD<R(1), R(2), R(14)>(%23);
            %25 : Ctx = ADD<R(2), R(3), R(9)>(%24);
            %26 : Ctx = ADD<R(3), R(4), R(10)>(%25);
            %27 : Ctx = ADD<R(4), R(5), R(11)>(%26);
            %28 : Ctx = ADD<R(5), R(6), R(7)>(%27);
            %29 : Ctx = PBS2<R(6, 2), R(13), LUT(26)>(%28);
            %30 : Ctx = PBS<R(8), R(0), LUT(47)>(%29);
            %31 : Ctx = PBS<R(9), R(1), LUT(48)>(%30);
            %32 : Ctx = PBS<R(10), R(2), LUT(49)>(%31);
            %33 : Ctx = PBS<R(11), R(3), LUT(47)>(%32);
            %34 : Ctx = PBS<R(12), R(4), LUT(48)>(%33);
            %35 : Ctx = PBSF<R(14), R(5), LUT(49)>(%34);
            %36 : Ctx = ADD<R(13), R(0), R(7)>(%35);
            %37 : Ctx = ADD<R(0), R(8), R(7)>(%36);
            %38 : Ctx = ADD<R(7), R(12), R(11)>(%37);
            %39 : Ctx = PBS<R(8), R(6), LUT(1)>(%38);
            %40 : Ctx = PBS<R(12), R(13), LUT(1)>(%39);
            %41 : Ctx = PBSF<R(15), R(0), LUT(44)>(%40);
            %42 : Ctx = ADD<R(6), R(9), R(0)>(%41);
            %43 : Ctx = ADD<R(0), R(14), R(7)>(%42);
            %44 : Ctx = ST<TC(0, 0), R(8)>(%43);
            %45 : Ctx = ST<TC(0, 1), R(12)>(%44);
            %46 : Ctx = ADD<R(8), R(10), R(6)>(%45);
            %47 : Ctx = ADD<R(9), R(1), R(15)>(%46);
            %48 : Ctx = PBS<R(1), R(6), LUT(45)>(%47);
            %49 : Ctx = PBS<R(10), R(8), LUT(46)>(%48);
            %50 : Ctx = PBSF<R(12), R(9), LUT(1)>(%49);
            %51 : Ctx = ADD<R(6), R(2), R(1)>(%50);
            %52 : Ctx = ST<TC(0, 2), R(12)>(%51);
            %53 : Ctx = ADD<R(1), R(11), R(10)>(%52);
            %54 : Ctx = ADD<R(2), R(7), R(10)>(%53);
            %55 : Ctx = ADD<R(7), R(0), R(10)>(%54);
            %56 : Ctx = PBS<R(0), R(6), LUT(1)>(%55);
            %57 : Ctx = PBS<R(8), R(1), LUT(46)>(%56);
            %58 : Ctx = PBS<R(9), R(2), LUT(44)>(%57);
            %59 : Ctx = PBSF<R(10), R(7), LUT(45)>(%58);
            %60 : Ctx = ST<TC(0, 3), R(0)>(%59);
            %61 : Ctx = ADD<R(0), R(3), R(8)>(%60);
            %62 : Ctx = ADD<R(1), R(4), R(9)>(%61);
            %63 : Ctx = ADD<R(2), R(5), R(10)>(%62);
            %64 : Ctx = PBS<R(3), R(0), LUT(1)>(%63);
            %65 : Ctx = PBS<R(4), R(1), LUT(1)>(%64);
            %66 : Ctx = PBSF<R(5), R(2), LUT(1)>(%65);
            %67 : Ctx = ST<TC(0, 4), R(3)>(%66);
            %68 : Ctx = ST<TC(0, 5), R(4)>(%67);
            %69 : Ctx = ST<TC(0, 6), R(5)>(%68);
            ",
        );
    }

    #[test]
    fn test_allocate_cmp_ir() {
        let ir = pipeline(&get_cmp_ir(16, 2, 2));
        ir.check_ir_linear(
            "
            %0 : Ctx = _INIT();
            %1 : Ctx = LD<R(0), TC(0, 0)>(%0);
            %2 : Ctx = LD<R(1), TC(0, 1)>(%1);
            %3 : Ctx = MAC<R(2), R(1), R(0), PT_I(4)>(%2);
            %4 : Ctx = LD<R(0), TC(0, 2)>(%3);
            %5 : Ctx = LD<R(1), TC(0, 3)>(%4);
            %6 : Ctx = LD<R(3), TC(0, 4)>(%5);
            %7 : Ctx = LD<R(4), TC(0, 5)>(%6);
            %8 : Ctx = MAC<R(5), R(1), R(0), PT_I(4)>(%7);
            %9 : Ctx = LD<R(0), TC(0, 6)>(%8);
            %10 : Ctx = LD<R(1), TC(0, 7)>(%9);
            %11 : Ctx = LD<R(6), TC(1, 0)>(%10);
            %12 : Ctx = LD<R(7), TC(1, 1)>(%11);
            %13 : Ctx = MAC<R(8), R(4), R(3), PT_I(4)>(%12);
            %14 : Ctx = LD<R(3), TC(1, 2)>(%13);
            %15 : Ctx = LD<R(4), TC(1, 3)>(%14);
            %16 : Ctx = LD<R(9), TC(1, 4)>(%15);
            %17 : Ctx = LD<R(10), TC(1, 5)>(%16);
            %18 : Ctx = MAC<R(11), R(1), R(0), PT_I(4)>(%17);
            %19 : Ctx = LD<R(0), TC(1, 6)>(%18);
            %20 : Ctx = LD<R(1), TC(1, 7)>(%19);
            %21 : Ctx = MAC<R(12), R(7), R(6), PT_I(4)>(%20);
            %22 : Ctx = MAC<R(6), R(4), R(3), PT_I(4)>(%21);
            %23 : Ctx = MAC<R(3), R(10), R(9), PT_I(4)>(%22);
            %24 : Ctx = MAC<R(4), R(1), R(0), PT_I(4)>(%23);
            %25 : Ctx = PBS<R(0), R(2), LUT(0)>(%24);
            %26 : Ctx = PBS<R(1), R(5), LUT(0)>(%25);
            %27 : Ctx = PBS<R(7), R(8), LUT(0)>(%26);
            %28 : Ctx = PBS<R(9), R(11), LUT(0)>(%27);
            %29 : Ctx = PBS<R(10), R(12), LUT(0)>(%28);
            %30 : Ctx = PBS<R(13), R(6), LUT(0)>(%29);
            %31 : Ctx = PBS<R(14), R(3), LUT(0)>(%30);
            %32 : Ctx = PBSF<R(15), R(4), LUT(0)>(%31);
            %33 : Ctx = SUB<R(2), R(0), R(10)>(%32);
            %34 : Ctx = SUB<R(0), R(1), R(13)>(%33);
            %35 : Ctx = SUB<R(1), R(7), R(14)>(%34);
            %36 : Ctx = SUB<R(3), R(9), R(15)>(%35);
            %37 : Ctx = PBS<R(4), R(2), LUT(10)>(%36);
            %38 : Ctx = PBS<R(5), R(0), LUT(10)>(%37);
            %39 : Ctx = PBS<R(6), R(1), LUT(10)>(%38);
            %40 : Ctx = PBSF<R(7), R(3), LUT(10)>(%39);
            %41 : Ctx = ADDS<R(0), R(4), PT_I(1)>(%40);
            %42 : Ctx = ADDS<R(1), R(5), PT_I(1)>(%41);
            %43 : Ctx = ADDS<R(2), R(6), PT_I(1)>(%42);
            %44 : Ctx = ADDS<R(3), R(7), PT_I(1)>(%43);
            %45 : Ctx = MAC<R(4), R(1), R(0), PT_I(4)>(%44);
            %46 : Ctx = MAC<R(0), R(3), R(2), PT_I(4)>(%45);
            %47 : Ctx = PBS<R(1), R(4), LUT(11)>(%46);
            %48 : Ctx = PBSF<R(2), R(0), LUT(11)>(%47);
            %49 : Ctx = MAC<R(0), R(2), R(1), PT_I(4)>(%48);
            %50 : Ctx = PBSF<R(1), R(0), LUT(27)>(%49);
            %51 : Ctx = ST<TC(0, 0), R(1)>(%50);
            ",
        );
    }
}
