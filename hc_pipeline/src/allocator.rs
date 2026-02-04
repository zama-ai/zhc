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

use hc_ir::{IR, OpId, OpIdRaw, OpRef, ValId, ValMap};
use hc_langs::{
    doplang::{Argument, DopInstructionSet, DopLang},
    hpulang::{HpuInstructionSet, HpuLang},
};
use hc_sim::hpu::HpuConfig;
use hc_utils::{
    StoreIndex,
    iter::{CollectInSmallVec, CollectInVec, MultiZip},
    small::{SmallMap, SmallVec},
    svec,
};

/// A register identifier used in the allocation process.
#[derive(Clone, Debug, Copy, PartialEq, Eq)]
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

#[derive(Clone, Debug, PartialEq, Eq)]
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
    pub fn from_scheduled_ir(ir: &IR<HpuLang>) -> Self {
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

#[derive(Clone, Debug, PartialEq, Eq)]
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
    pub fn from_op(op: &OpRef<HpuLang>) -> Self {
        let args = op.get_arg_valids();
        let rets = op.get_return_valids();
        let mut map = SmallMap::<ValId, ValId>::new();
        let HpuInstructionSet::Batch { block } = op.get_operation() else {
            unreachable!()
        };
        let mut ordered_batch_arg_valids = block
            .walk_ops_linear()
            .filter(|op| matches!(op.get_operation(), HpuInstructionSet::BatchArg { .. }))
            .covec();
        ordered_batch_arg_valids.sort_unstable_by_key(|op| {
            let HpuInstructionSet::BatchArg { pos, .. } = op.get_operation() else {
                unreachable!()
            };
            pos
        });
        let mut ordered_batch_ret_valids = block
            .walk_ops_linear()
            .filter(|op| matches!(op.get_operation(), HpuInstructionSet::BatchRet { .. }))
            .covec();
        ordered_batch_ret_valids.sort_unstable_by_key(|op| {
            let HpuInstructionSet::BatchRet { pos, .. } = op.get_operation() else {
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
    input: &'ir IR<HpuLang>,
    output: IR<DopLang>,
    live_ranges: LiveRangeMap,
    register_file: RegisterFile,
    heap: Heap,
    translation_map: ValMap<ValState>,
    current_ctx: ValId,
    point: OpIdRaw,
}

impl<'ir> Allocator<'ir> {
    pub fn init(ir: &IR<HpuLang>, nregs: usize) -> Allocator<'_> {
        let live_ranges = LiveRangeMap::from_scheduled_ir(ir);
        let register_file = RegisterFile::empty(nregs);
        let translation_map = ir.empty_valmap();
        let input = ir;
        let mut output = IR::empty();
        let (_, rets) = output.add_op(DopInstructionSet::_INIT, svec![]).unwrap();
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

    fn add_dop(&mut self, dop: DopInstructionSet) -> OpId {
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
                self.add_dop(DopInstructionSet::ST {
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
                self.add_dop(DopInstructionSet::LD {
                    dst: Argument::ct_reg(r.0),
                    src: Argument::ct_heap(slot.0 as usize),
                });
                r
            }
        }
    }

    pub fn allocate_registers(mut self) -> IR<DopLang> {
        for op in self.input.walk_ops_linear().covec().into_iter() {
            let args = op.get_arg_valids();
            let rets = op.get_return_valids();

            match op.get_operation() {
                HpuInstructionSet::SrcLd { from } => {
                    let [r_dst] = self.get_dst_registers([rets[0]]);
                    self.add_dop(DopInstructionSet::LD {
                        dst: Argument::ct_reg(r_dst.0),
                        src: Argument::ct_var(
                            from.src_pos.try_into().unwrap(),
                            from.block_pos.try_into().unwrap(),
                        ),
                    });
                }
                HpuInstructionSet::DstSt { to } => {
                    let r_src = self.get_src_register(args[0]);
                    self.add_dop(DopInstructionSet::ST {
                        src: Argument::ct_reg(r_src.0),
                        dst: Argument::ct_var(
                            to.dst_pos.try_into().unwrap(),
                            to.block_pos.try_into().unwrap(),
                        ),
                    });
                }
                HpuInstructionSet::ImmLd { .. } => {
                    // This is a no-op in the doplang dialect.
                    // Handled in Pt operations.
                }
                HpuInstructionSet::AddCt => {
                    let [r_dst] = self.get_dst_registers([rets[0]]);
                    let r_src1 = self.get_src_register(args[0]);
                    let r_src2 = self.get_src_register(args[1]);
                    self.add_dop(DopInstructionSet::ADD {
                        dst: Argument::ct_reg(r_dst.0),
                        src1: Argument::ct_reg(r_src1.0),
                        src2: Argument::ct_reg(r_src2.0),
                    });
                }
                HpuInstructionSet::SubCt => {
                    let [r_dst] = self.get_dst_registers([rets[0]]);
                    let r_src1 = self.get_src_register(args[0]);
                    let r_src2 = self.get_src_register(args[1]);
                    self.add_dop(DopInstructionSet::SUB {
                        dst: Argument::ct_reg(r_dst.0),
                        src1: Argument::ct_reg(r_src1.0),
                        src2: Argument::ct_reg(r_src2.0),
                    });
                }
                HpuInstructionSet::Mac { cst } => {
                    let [r_dst] = self.get_dst_registers([rets[0]]);
                    let r_src1 = self.get_src_register(args[0]);
                    let r_src2 = self.get_src_register(args[1]);
                    self.add_dop(DopInstructionSet::MAC {
                        dst: Argument::ct_reg(r_dst.0),
                        src1: Argument::ct_reg(r_src1.0),
                        src2: Argument::ct_reg(r_src2.0),
                        cst: Argument::pt_const(cst.0),
                    });
                }
                HpuInstructionSet::AddPt => {
                    let [r_dst] = self.get_dst_registers([rets[0]]);
                    let r_src = self.get_src_register(args[0]);
                    let imm_ld_op = self
                        .input
                        .get_val(args[1])
                        .get_origin()
                        .opref
                        .get_operation();
                    let HpuInstructionSet::ImmLd { from } = imm_ld_op else {
                        unreachable!()
                    };
                    self.add_dop(DopInstructionSet::ADDS {
                        dst: Argument::ct_reg(r_dst.0),
                        src: Argument::ct_reg(r_src.0),
                        cst: Argument::pt_var(
                            from.imm_pos.try_into().unwrap(),
                            from.block_pos.try_into().unwrap(),
                        ),
                    });
                }
                HpuInstructionSet::SubPt => {
                    let [r_dst] = self.get_dst_registers([rets[0]]);
                    let r_src = self.get_src_register(args[0]);
                    let imm_ld_op = self
                        .input
                        .get_val(args[1])
                        .get_origin()
                        .opref
                        .get_operation();
                    let HpuInstructionSet::ImmLd { from } = imm_ld_op else {
                        unreachable!()
                    };
                    self.add_dop(DopInstructionSet::SUBS {
                        dst: Argument::ct_reg(r_dst.0),
                        src: Argument::ct_reg(r_src.0),
                        cst: Argument::pt_var(
                            from.imm_pos.try_into().unwrap(),
                            from.block_pos.try_into().unwrap(),
                        ),
                    });
                }
                HpuInstructionSet::PtSub => {
                    let [r_dst] = self.get_dst_registers([rets[0]]);
                    let r_src = self.get_src_register(args[1]);
                    let imm_ld_op = self
                        .input
                        .get_val(args[0])
                        .get_origin()
                        .opref
                        .get_operation();
                    let HpuInstructionSet::ImmLd { from } = imm_ld_op else {
                        unreachable!()
                    };
                    self.add_dop(DopInstructionSet::SSUB {
                        dst: Argument::ct_reg(r_dst.0),
                        src: Argument::ct_reg(r_src.0),
                        cst: Argument::pt_var(
                            from.imm_pos.try_into().unwrap(),
                            from.block_pos.try_into().unwrap(),
                        ),
                    });
                }
                HpuInstructionSet::MulPt => {
                    let [r_dst] = self.get_dst_registers([rets[0]]);
                    let r_src = self.get_src_register(args[0]);
                    let imm_ld_op = self
                        .input
                        .get_val(args[1])
                        .get_origin()
                        .opref
                        .get_operation();
                    let HpuInstructionSet::ImmLd { from } = imm_ld_op else {
                        unreachable!()
                    };
                    self.add_dop(DopInstructionSet::MULS {
                        dst: Argument::ct_reg(r_dst.0),
                        src: Argument::ct_reg(r_src.0),
                        cst: Argument::pt_var(
                            from.imm_pos.try_into().unwrap(),
                            from.block_pos.try_into().unwrap(),
                        ),
                    });
                }
                HpuInstructionSet::AddCst { cst } => {
                    let [r_dst] = self.get_dst_registers([rets[0]]);
                    let r_src = self.get_src_register(args[0]);
                    self.add_dop(DopInstructionSet::ADDS {
                        dst: Argument::ct_reg(r_dst.0),
                        src: Argument::ct_reg(r_src.0),
                        cst: Argument::pt_const(cst.0),
                    });
                }
                HpuInstructionSet::SubCst { cst } => {
                    let [r_dst] = self.get_dst_registers([rets[0]]);
                    let r_src = self.get_src_register(args[0]);
                    self.add_dop(DopInstructionSet::SUBS {
                        dst: Argument::ct_reg(r_dst.0),
                        src: Argument::ct_reg(r_src.0),
                        cst: Argument::pt_const(cst.0),
                    });
                }
                HpuInstructionSet::CstSub { cst } => {
                    let [r_dst] = self.get_dst_registers([rets[0]]);
                    let r_src = self.get_src_register(args[0]);
                    self.add_dop(DopInstructionSet::SSUB {
                        dst: Argument::ct_reg(r_dst.0),
                        src: Argument::ct_reg(r_src.0),
                        cst: Argument::pt_const(cst.0),
                    });
                }
                HpuInstructionSet::MulCst { cst } => {
                    let [r_dst] = self.get_dst_registers([rets[0]]);
                    let r_src = self.get_src_register(args[0]);
                    self.add_dop(DopInstructionSet::MULS {
                        dst: Argument::ct_reg(r_dst.0),
                        src: Argument::ct_reg(r_src.0),
                        cst: Argument::pt_const(cst.0),
                    });
                }
                HpuInstructionSet::Batch { block } => {
                    let map = BatchMap::from_op(&op);
                    for op in block.walk_ops_linear() {
                        let rets = op.get_return_valids();
                        let args = op.get_arg_valids();
                        match op.get_operation() {
                            HpuInstructionSet::Pbs { lut } => {
                                let [r_dst] = self.get_dst_registers([map[rets[0]]]);
                                let r_src = self.get_src_register(map[args[0]]);
                                self.add_dop(DopInstructionSet::PBS {
                                    dst: Argument::ct_reg(r_dst.0),
                                    src: Argument::ct_reg(r_src.0),
                                    lut: Argument::lut_id(lut),
                                });
                            }
                            HpuInstructionSet::PbsF { lut } => {
                                let [r_dst] = self.get_dst_registers([map[rets[0]]]);
                                let r_src = self.get_src_register(map[args[0]]);
                                self.add_dop(DopInstructionSet::PBS_F {
                                    dst: Argument::ct_reg(r_dst.0),
                                    src: Argument::ct_reg(r_src.0),
                                    lut: Argument::lut_id(lut),
                                });
                            }
                            HpuInstructionSet::Pbs2 { lut } => {
                                let [r_dst, ..] =
                                    self.get_dst_registers([map[rets[0]], map[rets[1]]]);
                                let r_src = self.get_src_register(map[args[0]]);
                                self.add_dop(DopInstructionSet::PBS_ML2 {
                                    dst: Argument::ct_reg2(r_dst.0),
                                    src: Argument::ct_reg(r_src.0),
                                    lut: Argument::lut_id(lut),
                                });
                            }
                            HpuInstructionSet::Pbs2F { lut } => {
                                let [r_dst, ..] =
                                    self.get_dst_registers([map[rets[0]], map[rets[1]]]);
                                let r_src = self.get_src_register(map[args[0]]);
                                self.add_dop(DopInstructionSet::PBS_ML2_F {
                                    dst: Argument::ct_reg2(r_dst.0),
                                    src: Argument::ct_reg(r_src.0),
                                    lut: Argument::lut_id(lut),
                                });
                            }
                            HpuInstructionSet::Pbs4 { lut } => {
                                let [r_dst, ..] = self.get_dst_registers([
                                    map[rets[0]],
                                    map[rets[1]],
                                    map[rets[2]],
                                    map[rets[3]],
                                ]);
                                let r_src = self.get_src_register(map[args[0]]);
                                self.add_dop(DopInstructionSet::PBS_ML4 {
                                    dst: Argument::ct_reg4(r_dst.0),
                                    src: Argument::ct_reg(r_src.0),
                                    lut: Argument::lut_id(lut),
                                });
                            }
                            HpuInstructionSet::Pbs4F { lut } => {
                                let [r_dst, ..] = self.get_dst_registers([
                                    map[rets[0]],
                                    map[rets[1]],
                                    map[rets[2]],
                                    map[rets[3]],
                                ]);
                                let r_src = self.get_src_register(map[args[0]]);
                                self.add_dop(DopInstructionSet::PBS_ML4_F {
                                    dst: Argument::ct_reg4(r_dst.0),
                                    src: Argument::ct_reg(r_src.0),
                                    lut: Argument::lut_id(lut),
                                });
                            }
                            HpuInstructionSet::Pbs8 { lut } => {
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
                                self.add_dop(DopInstructionSet::PBS_ML8 {
                                    dst: Argument::ct_reg8(r_dst.0),
                                    src: Argument::ct_reg(r_src.0),
                                    lut: Argument::lut_id(lut),
                                });
                            }
                            HpuInstructionSet::Pbs8F { lut } => {
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
                                self.add_dop(DopInstructionSet::PBS_ML8_F {
                                    dst: Argument::ct_reg8(r_dst.0),
                                    src: Argument::ct_reg(r_src.0),
                                    lut: Argument::lut_id(lut),
                                });
                            }
                            HpuInstructionSet::BatchArg { .. }
                            | HpuInstructionSet::BatchRet { .. } => {}
                            _ => unreachable!(
                                "Encountered unexpected operation while allocating: {}",
                                op.get_operation()
                            ),
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
                println!("{}", op.format());
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
pub fn allocate_registers(ir: &IR<HpuLang>, config: &HpuConfig) -> IR<DopLang> {
    let allocator = Allocator::init(ir, config.regf_size);
    allocator.allocate_registers()
}

#[cfg(test)]
mod test {
    use hc_ir::{IR, PrintWalker, translation::Translator};
    use hc_langs::{doplang::DopLang, ioplang::IopLang};
    use hc_sim::hpu::{HpuConfig, PhysicalConfig};
    use hc_utils::assert_display_is;

    use crate::{
        batcher::batch,
        scheduler::schedule,
        test::{get_add_ir, get_cmp_ir},
        translation::IoplangToHpulang,
    };

    use super::allocate_registers;

    fn pipeline(ir: &IR<IopLang>) -> IR<DopLang> {
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
        assert_display_is!(
            ir.format(),
            r#"
                %0 : Ctx = _INIT();
                %1 : Ctx = LD<R(0), TC(0, 0)>(%0 : Ctx);
                %2 : Ctx = LD<R(1), TC(0, 1)>(%1 : Ctx);
                %3 : Ctx = LD<R(2), TC(0, 2)>(%2 : Ctx);
                %4 : Ctx = LD<R(3), TC(0, 3)>(%3 : Ctx);
                %5 : Ctx = LD<R(4), TC(0, 4)>(%4 : Ctx);
                %6 : Ctx = LD<R(5), TC(0, 5)>(%5 : Ctx);
                %7 : Ctx = LD<R(6), TC(0, 6)>(%6 : Ctx);
                %8 : Ctx = LD<R(7), TC(0, 7)>(%7 : Ctx);
                %9 : Ctx = LD<R(8), TC(1, 0)>(%8 : Ctx);
                %10 : Ctx = ADD<R(9), R(0), R(8)>(%9 : Ctx);
                %11 : Ctx = LD<R(0), TC(1, 1)>(%10 : Ctx);
                %12 : Ctx = LD<R(8), TC(1, 2)>(%11 : Ctx);
                %13 : Ctx = LD<R(10), TC(1, 3)>(%12 : Ctx);
                %14 : Ctx = LD<R(11), TC(1, 4)>(%13 : Ctx);
                %15 : Ctx = ADD<R(12), R(1), R(0)>(%14 : Ctx);
                %16 : Ctx = LD<R(0), TC(1, 5)>(%15 : Ctx);
                %17 : Ctx = LD<R(1), TC(1, 6)>(%16 : Ctx);
                %18 : Ctx = LD<R(13), TC(1, 7)>(%17 : Ctx);
                %19 : Ctx = ADD<R(14), R(2), R(8)>(%18 : Ctx);
                %20 : Ctx = ADD<R(2), R(3), R(10)>(%19 : Ctx);
                %21 : Ctx = ADD<R(3), R(4), R(11)>(%20 : Ctx);
                %22 : Ctx = ADD<R(4), R(5), R(0)>(%21 : Ctx);
                %23 : Ctx = ADD<R(0), R(6), R(1)>(%22 : Ctx);
                %24 : Ctx = ADD<R(1), R(7), R(13)>(%23 : Ctx);
                %25 : Ctx = PBS2<R(6, 2), R(9), LUT(26)>(%24 : Ctx);
                %26 : Ctx = PBS<R(5), R(12), LUT(47)>(%25 : Ctx);
                %27 : Ctx = PBS<R(8), R(14), LUT(48)>(%26 : Ctx);
                %28 : Ctx = PBS<R(10), R(2), LUT(49)>(%27 : Ctx);
                %29 : Ctx = PBS<R(11), R(3), LUT(47)>(%28 : Ctx);
                %30 : Ctx = PBS<R(13), R(4), LUT(48)>(%29 : Ctx);
                %31 : Ctx = PBSF<R(15), R(0), LUT(49)>(%30 : Ctx);
                %32 : Ctx = ADD<R(9), R(12), R(7)>(%31 : Ctx);
                %33 : Ctx = ADD<R(12), R(7), R(5)>(%32 : Ctx);
                %34 : Ctx = ADD<R(5), R(11), R(13)>(%33 : Ctx);
                %35 : Ctx = PBS<R(7), R(6), LUT(1)>(%34 : Ctx);
                %36 : Ctx = PBS<R(13), R(9), LUT(1)>(%35 : Ctx);
                %37 : Ctx = PBSF<R(16), R(12), LUT(44)>(%36 : Ctx);
                %38 : Ctx = ADD<R(6), R(12), R(8)>(%37 : Ctx);
                %39 : Ctx = ADD<R(8), R(5), R(15)>(%38 : Ctx);
                %40 : Ctx = ST<TC(0, 0), R(7)>(%39 : Ctx);
                %41 : Ctx = ST<TC(0, 1), R(13)>(%40 : Ctx);
                %42 : Ctx = ADD<R(7), R(6), R(10)>(%41 : Ctx);
                %43 : Ctx = ADD<R(9), R(14), R(16)>(%42 : Ctx);
                %44 : Ctx = PBS<R(10), R(6), LUT(45)>(%43 : Ctx);
                %45 : Ctx = PBS<R(12), R(7), LUT(46)>(%44 : Ctx);
                %46 : Ctx = PBSF<R(13), R(9), LUT(1)>(%45 : Ctx);
                %47 : Ctx = ADD<R(6), R(2), R(10)>(%46 : Ctx);
                %48 : Ctx = ST<TC(0, 2), R(13)>(%47 : Ctx);
                %49 : Ctx = ADD<R(2), R(11), R(12)>(%48 : Ctx);
                %50 : Ctx = ADD<R(7), R(5), R(12)>(%49 : Ctx);
                %51 : Ctx = ADD<R(5), R(8), R(12)>(%50 : Ctx);
                %52 : Ctx = ADD<R(8), R(3), R(12)>(%51 : Ctx);
                %53 : Ctx = PBS<R(3), R(6), LUT(1)>(%52 : Ctx);
                %54 : Ctx = PBS<R(9), R(2), LUT(44)>(%53 : Ctx);
                %55 : Ctx = PBS<R(10), R(7), LUT(45)>(%54 : Ctx);
                %56 : Ctx = PBS<R(11), R(5), LUT(46)>(%55 : Ctx);
                %57 : Ctx = PBSF<R(12), R(8), LUT(1)>(%56 : Ctx);
                %58 : Ctx = ST<TC(0, 3), R(3)>(%57 : Ctx);
                %59 : Ctx = ADD<R(2), R(4), R(9)>(%58 : Ctx);
                %60 : Ctx = ST<TC(0, 4), R(12)>(%59 : Ctx);
                %61 : Ctx = ADD<R(3), R(0), R(10)>(%60 : Ctx);
                %62 : Ctx = ADD<R(0), R(1), R(11)>(%61 : Ctx);
                %63 : Ctx = PBS<R(1), R(2), LUT(1)>(%62 : Ctx);
                %64 : Ctx = PBS<R(4), R(3), LUT(1)>(%63 : Ctx);
                %65 : Ctx = PBSF<R(5), R(0), LUT(1)>(%64 : Ctx);
                %66 : Ctx = ST<TC(0, 5), R(1)>(%65 : Ctx);
                %67 : Ctx = ST<TC(0, 6), R(4)>(%66 : Ctx);
                %68 : Ctx = ST<TC(0, 7), R(5)>(%67 : Ctx);
            "#
        );
    }

    #[test]
    fn test_allocate_cmp_ir() {
        let ir = pipeline(&get_cmp_ir(16, 2, 2));
        assert_display_is!(
            ir.format().with_walker(PrintWalker::Linear),
            r#"
            %0 : Ctx = _INIT();
            %1 : Ctx = LD<R(0), TC(0, 0)>(%0 : Ctx);
            %2 : Ctx = LD<R(1), TC(0, 1)>(%1 : Ctx);
            %3 : Ctx = MAC<R(2), R(1), R(0), PT_I(4)>(%2 : Ctx);
            %4 : Ctx = LD<R(0), TC(0, 2)>(%3 : Ctx);
            %5 : Ctx = LD<R(1), TC(0, 3)>(%4 : Ctx);
            %6 : Ctx = LD<R(3), TC(0, 4)>(%5 : Ctx);
            %7 : Ctx = LD<R(4), TC(0, 5)>(%6 : Ctx);
            %8 : Ctx = MAC<R(5), R(1), R(0), PT_I(4)>(%7 : Ctx);
            %9 : Ctx = LD<R(0), TC(0, 6)>(%8 : Ctx);
            %10 : Ctx = LD<R(1), TC(0, 7)>(%9 : Ctx);
            %11 : Ctx = LD<R(6), TC(1, 0)>(%10 : Ctx);
            %12 : Ctx = LD<R(7), TC(1, 1)>(%11 : Ctx);
            %13 : Ctx = MAC<R(8), R(4), R(3), PT_I(4)>(%12 : Ctx);
            %14 : Ctx = LD<R(3), TC(1, 2)>(%13 : Ctx);
            %15 : Ctx = LD<R(4), TC(1, 3)>(%14 : Ctx);
            %16 : Ctx = LD<R(9), TC(1, 4)>(%15 : Ctx);
            %17 : Ctx = LD<R(10), TC(1, 5)>(%16 : Ctx);
            %18 : Ctx = MAC<R(11), R(1), R(0), PT_I(4)>(%17 : Ctx);
            %19 : Ctx = LD<R(0), TC(1, 6)>(%18 : Ctx);
            %20 : Ctx = LD<R(1), TC(1, 7)>(%19 : Ctx);
            %21 : Ctx = MAC<R(12), R(7), R(6), PT_I(4)>(%20 : Ctx);
            %22 : Ctx = MAC<R(6), R(4), R(3), PT_I(4)>(%21 : Ctx);
            %23 : Ctx = MAC<R(3), R(10), R(9), PT_I(4)>(%22 : Ctx);
            %24 : Ctx = MAC<R(4), R(1), R(0), PT_I(4)>(%23 : Ctx);
            %25 : Ctx = PBS<R(0), R(2), LUT(0)>(%24 : Ctx);
            %26 : Ctx = PBS<R(1), R(5), LUT(0)>(%25 : Ctx);
            %27 : Ctx = PBS<R(7), R(8), LUT(0)>(%26 : Ctx);
            %28 : Ctx = PBS<R(9), R(11), LUT(0)>(%27 : Ctx);
            %29 : Ctx = PBS<R(10), R(12), LUT(0)>(%28 : Ctx);
            %30 : Ctx = PBS<R(13), R(6), LUT(0)>(%29 : Ctx);
            %31 : Ctx = PBS<R(14), R(3), LUT(0)>(%30 : Ctx);
            %32 : Ctx = PBSF<R(15), R(4), LUT(0)>(%31 : Ctx);
            %33 : Ctx = SUB<R(2), R(0), R(10)>(%32 : Ctx);
            %34 : Ctx = SUB<R(0), R(1), R(13)>(%33 : Ctx);
            %35 : Ctx = SUB<R(1), R(7), R(14)>(%34 : Ctx);
            %36 : Ctx = SUB<R(3), R(9), R(15)>(%35 : Ctx);
            %37 : Ctx = PBS<R(4), R(2), LUT(10)>(%36 : Ctx);
            %38 : Ctx = PBS<R(5), R(0), LUT(10)>(%37 : Ctx);
            %39 : Ctx = PBS<R(6), R(1), LUT(10)>(%38 : Ctx);
            %40 : Ctx = PBSF<R(7), R(3), LUT(10)>(%39 : Ctx);
            %41 : Ctx = ADDS<R(0), R(4), PT_I(1)>(%40 : Ctx);
            %42 : Ctx = ADDS<R(1), R(5), PT_I(1)>(%41 : Ctx);
            %43 : Ctx = ADDS<R(2), R(6), PT_I(1)>(%42 : Ctx);
            %44 : Ctx = ADDS<R(3), R(7), PT_I(1)>(%43 : Ctx);
            %45 : Ctx = MAC<R(4), R(1), R(0), PT_I(4)>(%44 : Ctx);
            %46 : Ctx = MAC<R(0), R(3), R(2), PT_I(4)>(%45 : Ctx);
            %47 : Ctx = PBS<R(1), R(4), LUT(11)>(%46 : Ctx);
            %48 : Ctx = PBSF<R(2), R(0), LUT(11)>(%47 : Ctx);
            %49 : Ctx = MAC<R(0), R(2), R(1), PT_I(4)>(%48 : Ctx);
            %50 : Ctx = PBSF<R(1), R(0), LUT(27)>(%49 : Ctx);
            %51 : Ctx = ST<TC(0, 0), R(1)>(%50 : Ctx);
        "#
        );
    }
}
