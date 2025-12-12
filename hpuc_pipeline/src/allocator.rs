//! Register allocation for HPU operations.
//!
//! This module implements register allocation algorithms that assign physical
//! registers to virtual values in the scheduled intermediate representation.
//! The allocator handles register pressure, spilling to memory when necessary,
//! and produces device operation code with concrete register assignments.

use std::{
    fmt::{Debug, Display},
    ops::{Div, Rem},
};

use hpuc_ir::{IR, OpId, OpIdRaw, ValId, ValMap};
use hpuc_langs::{
    doplang::{Argument, Doplang, Operations as DopOp},
    hpulang::{Hpulang, Operations as HpuOp},
};
use hpuc_sim::hpu::HpuConfig;
use hpuc_utils::{CollectInSmallVec, CollectInVec, SmallMap, SmallVec, StoreIndex, svec};

/// A register identifier used in the allocation process.
#[derive(Clone, Debug, Copy)]
pub struct Register(OpIdRaw);

#[derive(Clone, Copy, Debug)]
enum RegisterState {
    /// The register does not hold any value
    Empty,
    /// The register holds a freshly added value.
    Fresh(ValId),
    /// The register holds a value
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
                        src: Argument::ct_var(from.src_pos, from.block_pos),
                    });
                }
                HpuOp::DstSt { to } => {
                    let r_src = self.get_src_register(args[0]);
                    self.add_dop(DopOp::ST {
                        src: Argument::ct_reg(r_src.0),
                        dst: Argument::ct_var(to.dst_pos, to.block_pos),
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
                        cst: Argument::pt_var(from.imm_pos, from.block_pos),
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
                        cst: Argument::pt_var(from.imm_pos, from.block_pos),
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
                        cst: Argument::pt_var(from.imm_pos, from.block_pos),
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
                        cst: Argument::pt_var(from.imm_pos, from.block_pos),
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
                HpuOp::Pbs { lut } => {
                    let [r_dst] = self.get_dst_registers([rets[0]]);
                    let r_src = self.get_src_register(args[0]);
                    self.add_dop(DopOp::PBS {
                        dst: Argument::ct_reg(r_dst.0),
                        src: Argument::ct_reg(r_src.0),
                        lut: Argument::lut_id(lut),
                    });
                }
                HpuOp::PbsF { lut } => {
                    let [r_dst] = self.get_dst_registers([rets[0]]);
                    let r_src = self.get_src_register(args[0]);
                    self.add_dop(DopOp::PBS_F {
                        dst: Argument::ct_reg(r_dst.0),
                        src: Argument::ct_reg(r_src.0),
                        lut: Argument::lut_id(lut),
                    });
                }
                HpuOp::Pbs2 { lut } => {
                    let [r_dst, ..] = self.get_dst_registers([rets[0], rets[1]]);
                    let r_src = self.get_src_register(args[0]);
                    self.add_dop(DopOp::PBS_ML2 {
                        dst: Argument::ct_reg2(r_dst.0),
                        src: Argument::ct_reg(r_src.0),
                        lut: Argument::lut_id(lut),
                    });
                }
                HpuOp::Pbs2F { lut } => {
                    let [r_dst, ..] = self.get_dst_registers([rets[0], rets[1]]);
                    let r_src = self.get_src_register(args[0]);
                    self.add_dop(DopOp::PBS_ML2_F {
                        dst: Argument::ct_reg2(r_dst.0),
                        src: Argument::ct_reg(r_src.0),
                        lut: Argument::lut_id(lut),
                    });
                }
                HpuOp::Pbs4 { lut } => {
                    let [r_dst, ..] = self.get_dst_registers([rets[0], rets[1], rets[2], rets[3]]);
                    let r_src = self.get_src_register(args[0]);
                    self.add_dop(DopOp::PBS_ML4 {
                        dst: Argument::ct_reg4(r_dst.0),
                        src: Argument::ct_reg(r_src.0),
                        lut: Argument::lut_id(lut),
                    });
                }
                HpuOp::Pbs4F { lut } => {
                    let [r_dst, ..] = self.get_dst_registers([rets[0], rets[1], rets[2], rets[3]]);
                    let r_src = self.get_src_register(args[0]);
                    self.add_dop(DopOp::PBS_ML4_F {
                        dst: Argument::ct_reg4(r_dst.0),
                        src: Argument::ct_reg(r_src.0),
                        lut: Argument::lut_id(lut),
                    });
                }
                HpuOp::Pbs8 { lut } => {
                    let [r_dst, ..] = self.get_dst_registers([
                        rets[0], rets[1], rets[2], rets[3], rets[4], rets[5], rets[6], rets[7],
                    ]);
                    let r_src = self.get_src_register(args[0]);
                    self.add_dop(DopOp::PBS_ML8 {
                        dst: Argument::ct_reg8(r_dst.0),
                        src: Argument::ct_reg(r_src.0),
                        lut: Argument::lut_id(lut),
                    });
                }
                HpuOp::Pbs8F { lut } => {
                    let [r_dst, ..] = self.get_dst_registers([
                        rets[0], rets[1], rets[2], rets[3], rets[4], rets[5], rets[6], rets[7],
                    ]);
                    let r_src = self.get_src_register(args[0]);
                    self.add_dop(DopOp::PBS_ML8_F {
                        dst: Argument::ct_reg8(r_dst.0),
                        src: Argument::ct_reg(r_src.0),
                        lut: Argument::lut_id(lut),
                    });
                }
            }

            // println!("{}", op);
            // println!("{}: {}", self.point, self.register_file);

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
        scheduler::schedule,
        test::{get_add_ir, get_cmp_ir, get_sub_ir},
        translation::IoplangToHpulang,
    };

    use super::allocate_registers;

    fn pipeline(ir: &IR<Ioplang>) -> IR<Doplang> {
        let ir = IoplangToHpulang.translate(&ir);
        let mut config = HpuConfig::from(PhysicalConfig::gaussian_64b());
        config.regf_size = 10;
        let scheduled = schedule(&ir, &config);
        let allocated = allocate_registers(&scheduled, &config);
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
            %13 : Ctx = ST<CT_H(0), R(6)>(%12);
            %14 : Ctx = LD<R(6), TC(1, 4)>(%13);
            %15 : Ctx = ST<CT_H(1), R(5)>(%14);
            %16 : Ctx = ADD<R(5), R(1), R(0)>(%15);
            %17 : Ctx = PBS2<R(0, 2), R(8), LUT(26)>(%16);
            %18 : Ctx = LD<R(8), TC(1, 5)>(%17);
            %19 : Ctx = ST<CT_H(2), R(0)>(%18);
            %20 : Ctx = LD<R(0), TC(1, 6)>(%19);
            %21 : Ctx = ST<CT_H(3), R(1)>(%20);
            %22 : Ctx = ADD<R(1), R(2), R(7)>(%21);
            %23 : Ctx = PBS<R(2), R(5), LUT(47)>(%22);
            %24 : Ctx = ADD<R(7), R(3), R(9)>(%23);
            %25 : Ctx = PBS<R(3), R(1), LUT(48)>(%24);
            %26 : Ctx = ADD<R(9), R(4), R(6)>(%25);
            %27 : Ctx = PBS<R(4), R(7), LUT(49)>(%26);
            %28 : Ctx = ST<CT_H(4), R(7)>(%27);
            %29 : Ctx = LD<R(7), CT_H(1)>(%28);
            %30 : Ctx = ADD<R(6), R(7), R(8)>(%29);
            %31 : Ctx = PBS<R(7), R(9), LUT(47)>(%30);
            %32 : Ctx = ST<CT_H(5), R(9)>(%31);
            %33 : Ctx = LD<R(9), CT_H(0)>(%32);
            %34 : Ctx = ADD<R(8), R(9), R(0)>(%33);
            %35 : Ctx = PBS<R(0), R(6), LUT(48)>(%34);
            %36 : Ctx = PBSF<R(9), R(8), LUT(49)>(%35);
            %37 : Ctx = ST<CT_H(6), R(8)>(%36);
            %38 : Ctx = ST<CT_H(7), R(6)>(%37);
            %39 : Ctx = LD<R(6), CT_H(3)>(%38);
            %40 : Ctx = ADD<R(8), R(5), R(6)>(%39);
            %41 : Ctx = LD<R(5), CT_H(2)>(%40);
            %42 : Ctx = ST<TC(0, 0), R(5)>(%41);
            %43 : Ctx = ADD<R(5), R(2), R(6)>(%42);
            %44 : Ctx = ST<TC(0, 1), R(8)>(%43);
            %45 : Ctx = ADD<R(2), R(0), R(7)>(%44);
            %46 : Ctx = PBSF<R(0), R(5), LUT(44)>(%45);
            %47 : Ctx = ADD<R(6), R(3), R(5)>(%46);
            %48 : Ctx = ADD<R(3), R(9), R(2)>(%47);
            %49 : Ctx = PBSF<R(5), R(6), LUT(45)>(%48);
            %50 : Ctx = ADD<R(8), R(4), R(6)>(%49);
            %51 : Ctx = ADD<R(4), R(1), R(0)>(%50);
            %52 : Ctx = PBSF<R(0), R(8), LUT(46)>(%51);
            %53 : Ctx = LD<R(6), CT_H(4)>(%52);
            %54 : Ctx = ADD<R(1), R(6), R(5)>(%53);
            %55 : Ctx = ST<TC(0, 2), R(4)>(%54);
            %56 : Ctx = ST<TC(0, 3), R(1)>(%55);
            %57 : Ctx = ADD<R(1), R(7), R(0)>(%56);
            %58 : Ctx = ADD<R(4), R(2), R(0)>(%57);
            %59 : Ctx = PBS<R(2), R(1), LUT(46)>(%58);
            %60 : Ctx = ADD<R(1), R(3), R(0)>(%59);
            %61 : Ctx = PBS<R(0), R(4), LUT(44)>(%60);
            %62 : Ctx = PBSF<R(3), R(1), LUT(45)>(%61);
            %63 : Ctx = LD<R(4), CT_H(5)>(%62);
            %64 : Ctx = ADD<R(1), R(4), R(2)>(%63);
            %65 : Ctx = LD<R(4), CT_H(7)>(%64);
            %66 : Ctx = ADD<R(2), R(4), R(0)>(%65);
            %67 : Ctx = ST<TC(0, 4), R(1)>(%66);
            %68 : Ctx = LD<R(1), CT_H(6)>(%67);
            %69 : Ctx = ADD<R(0), R(1), R(3)>(%68);
            %70 : Ctx = ST<TC(0, 5), R(2)>(%69);
            %71 : Ctx = ST<TC(0, 6), R(0)>(%70);
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
            %12 : Ctx = ST<CT_H(0), R(6)>(%11);
            %13 : Ctx = LD<R(6), TC(1, 3)>(%12);
            %14 : Ctx = ST<CT_H(1), R(5)>(%13);
            %15 : Ctx = LD<R(5), TC(1, 4)>(%14);
            %16 : Ctx = ST<CT_H(2), R(4)>(%15);
            %17 : Ctx = SSUB<R(4), R(7), PT_I(3)>(%16);
            %18 : Ctx = LD<R(7), TC(1, 5)>(%17);
            %19 : Ctx = ST<CT_H(3), R(3)>(%18);
            %20 : Ctx = LD<R(3), TC(1, 6)>(%19);
            %21 : Ctx = ST<CT_H(4), R(2)>(%20);
            %22 : Ctx = SSUB<R(2), R(9), PT_I(3)>(%21);
            %23 : Ctx = SSUB<R(9), R(6), PT_I(3)>(%22);
            %24 : Ctx = SSUB<R(6), R(5), PT_I(3)>(%23);
            %25 : Ctx = SSUB<R(5), R(7), PT_I(3)>(%24);
            %26 : Ctx = SSUB<R(7), R(3), PT_I(3)>(%25);
            %27 : Ctx = ADD<R(3), R(0), R(8)>(%26);
            %28 : Ctx = ADD<R(0), R(1), R(4)>(%27);
            %29 : Ctx = ST<CT_H(5), R(5)>(%28);
            %30 : Ctx = PBS2<R(4, 2), R(3), LUT(26)>(%29);
            %31 : Ctx = LD<R(3), CT_H(4)>(%30);
            %32 : Ctx = ADD<R(1), R(3), R(2)>(%31);
            %33 : Ctx = PBS<R(2), R(0), LUT(47)>(%32);
            %34 : Ctx = LD<R(8), CT_H(3)>(%33);
            %35 : Ctx = ADD<R(3), R(8), R(9)>(%34);
            %36 : Ctx = PBS<R(8), R(1), LUT(48)>(%35);
            %37 : Ctx = ST<CT_H(6), R(1)>(%36);
            %38 : Ctx = LD<R(1), CT_H(2)>(%37);
            %39 : Ctx = ADD<R(9), R(1), R(6)>(%38);
            %40 : Ctx = PBS<R(1), R(3), LUT(49)>(%39);
            %41 : Ctx = ST<CT_H(7), R(3)>(%40);
            %42 : Ctx = LD<R(3), CT_H(1)>(%41);
            %43 : Ctx = ST<CT_H(8), R(1)>(%42);
            %44 : Ctx = LD<R(1), CT_H(5)>(%43);
            %45 : Ctx = ADD<R(6), R(3), R(1)>(%44);
            %46 : Ctx = PBS<R(1), R(9), LUT(47)>(%45);
            %47 : Ctx = ST<CT_H(9), R(9)>(%46);
            %48 : Ctx = LD<R(9), CT_H(0)>(%47);
            %49 : Ctx = ADD<R(3), R(9), R(7)>(%48);
            %50 : Ctx = PBS<R(7), R(6), LUT(48)>(%49);
            %51 : Ctx = PBSF<R(9), R(3), LUT(49)>(%50);
            %52 : Ctx = ST<CT_H(10), R(3)>(%51);
            %53 : Ctx = ADD<R(3), R(0), R(5)>(%52);
            %54 : Ctx = PBS<R(0), R(4), LUT(1)>(%53);
            %55 : Ctx = ADD<R(4), R(2), R(5)>(%54);
            %56 : Ctx = PBS<R(2), R(3), LUT(1)>(%55);
            %57 : Ctx = ADD<R(3), R(7), R(1)>(%56);
            %58 : Ctx = PBSF<R(5), R(4), LUT(44)>(%57);
            %59 : Ctx = ADD<R(7), R(8), R(4)>(%58);
            %60 : Ctx = ADD<R(4), R(9), R(3)>(%59);
            %61 : Ctx = ST<TC(0, 0), R(0)>(%60);
            %62 : Ctx = ST<TC(0, 1), R(2)>(%61);
            %63 : Ctx = LD<R(2), CT_H(8)>(%62);
            %64 : Ctx = ADD<R(0), R(2), R(7)>(%63);
            %65 : Ctx = PBS<R(2), R(7), LUT(45)>(%64);
            %66 : Ctx = LD<R(8), CT_H(6)>(%65);
            %67 : Ctx = ADD<R(7), R(8), R(5)>(%66);
            %68 : Ctx = PBS<R(5), R(0), LUT(46)>(%67);
            %69 : Ctx = PBSF<R(0), R(7), LUT(1)>(%68);
            %70 : Ctx = LD<R(8), CT_H(7)>(%69);
            %71 : Ctx = ADD<R(7), R(8), R(2)>(%70);
            %72 : Ctx = ST<TC(0, 2), R(0)>(%71);
            %73 : Ctx = ADD<R(0), R(1), R(5)>(%72);
            %74 : Ctx = PBS<R(1), R(7), LUT(1)>(%73);
            %75 : Ctx = ADD<R(2), R(3), R(5)>(%74);
            %76 : Ctx = PBS<R(3), R(0), LUT(46)>(%75);
            %77 : Ctx = ADD<R(0), R(4), R(5)>(%76);
            %78 : Ctx = PBS<R(4), R(2), LUT(44)>(%77);
            %79 : Ctx = PBSF<R(2), R(0), LUT(45)>(%78);
            %80 : Ctx = ST<TC(0, 3), R(1)>(%79);
            %81 : Ctx = LD<R(1), CT_H(9)>(%80);
            %82 : Ctx = ADD<R(0), R(1), R(3)>(%81);
            %83 : Ctx = ADD<R(1), R(6), R(4)>(%82);
            %84 : Ctx = PBS<R(3), R(0), LUT(1)>(%83);
            %85 : Ctx = LD<R(4), CT_H(10)>(%84);
            %86 : Ctx = ADD<R(0), R(4), R(2)>(%85);
            %87 : Ctx = PBS<R(2), R(1), LUT(1)>(%86);
            %88 : Ctx = PBSF<R(1), R(0), LUT(1)>(%87);
            %89 : Ctx = ST<TC(0, 4), R(3)>(%88);
            %90 : Ctx = ST<TC(0, 5), R(2)>(%89);
            %91 : Ctx = ST<TC(0, 6), R(1)>(%90);
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
            %9 : Ctx = PBS<R(0), R(2), LUT(0)>(%8);
            %10 : Ctx = LD<R(1), TC(0, 6)>(%9);
            %11 : Ctx = LD<R(2), TC(0, 7)>(%10);
            %12 : Ctx = LD<R(6), TC(1, 0)>(%11);
            %13 : Ctx = LD<R(7), TC(1, 1)>(%12);
            %14 : Ctx = MAC<R(8), R(4), R(3), PT_I(4)>(%13);
            %15 : Ctx = PBS<R(3), R(5), LUT(0)>(%14);
            %16 : Ctx = LD<R(4), TC(1, 2)>(%15);
            %17 : Ctx = LD<R(5), TC(1, 3)>(%16);
            %18 : Ctx = LD<R(9), TC(1, 4)>(%17);
            %19 : Ctx = ST<CT_H(0), R(3)>(%18);
            %20 : Ctx = LD<R(3), TC(1, 5)>(%19);
            %21 : Ctx = ST<CT_H(1), R(0)>(%20);
            %22 : Ctx = MAC<R(0), R(2), R(1), PT_I(4)>(%21);
            %23 : Ctx = PBS<R(1), R(8), LUT(0)>(%22);
            %24 : Ctx = LD<R(2), TC(1, 6)>(%23);
            %25 : Ctx = LD<R(8), TC(1, 7)>(%24);
            %26 : Ctx = ST<CT_H(2), R(1)>(%25);
            %27 : Ctx = MAC<R(1), R(7), R(6), PT_I(4)>(%26);
            %28 : Ctx = PBS<R(6), R(0), LUT(0)>(%27);
            %29 : Ctx = MAC<R(0), R(5), R(4), PT_I(4)>(%28);
            %30 : Ctx = PBS<R(4), R(1), LUT(0)>(%29);
            %31 : Ctx = MAC<R(1), R(3), R(9), PT_I(4)>(%30);
            %32 : Ctx = PBS<R(3), R(0), LUT(0)>(%31);
            %33 : Ctx = MAC<R(0), R(8), R(2), PT_I(4)>(%32);
            %34 : Ctx = PBS<R(2), R(1), LUT(0)>(%33);
            %35 : Ctx = PBSF<R(1), R(0), LUT(0)>(%34);
            %36 : Ctx = LD<R(5), CT_H(1)>(%35);
            %37 : Ctx = SUB<R(0), R(5), R(4)>(%36);
            %38 : Ctx = LD<R(5), CT_H(0)>(%37);
            %39 : Ctx = SUB<R(4), R(5), R(3)>(%38);
            %40 : Ctx = PBS<R(3), R(0), LUT(10)>(%39);
            %41 : Ctx = LD<R(5), CT_H(2)>(%40);
            %42 : Ctx = SUB<R(0), R(5), R(2)>(%41);
            %43 : Ctx = PBS<R(2), R(4), LUT(10)>(%42);
            %44 : Ctx = SUB<R(4), R(6), R(1)>(%43);
            %45 : Ctx = PBS<R(1), R(0), LUT(10)>(%44);
            %46 : Ctx = PBSF<R(0), R(4), LUT(10)>(%45);
            %47 : Ctx = ADDS<R(4), R(3), PT_I(1)>(%46);
            %48 : Ctx = ADDS<R(3), R(2), PT_I(1)>(%47);
            %49 : Ctx = ADDS<R(2), R(1), PT_I(1)>(%48);
            %50 : Ctx = ADDS<R(1), R(0), PT_I(1)>(%49);
            %51 : Ctx = MAC<R(0), R(3), R(4), PT_I(4)>(%50);
            %52 : Ctx = MAC<R(3), R(1), R(2), PT_I(4)>(%51);
            %53 : Ctx = PBS<R(1), R(0), LUT(0)>(%52);
            %54 : Ctx = PBSF<R(0), R(3), LUT(0)>(%53);
            %55 : Ctx = PBSF<R(2), R(1), LUT(11)>(%54);
            %56 : Ctx = PBSF<R(1), R(0), LUT(11)>(%55);
            %57 : Ctx = MAC<R(0), R(1), R(2), PT_I(4)>(%56);
            %58 : Ctx = PBSF<R(1), R(0), LUT(0)>(%57);
            %59 : Ctx = PBSF<R(0), R(1), LUT(27)>(%58);
            %60 : Ctx = ST<TC(0, 0), R(0)>(%59);
            ",
        );
    }
}
