use std::{
    fmt::{Debug, Display},
    ops::{Div, Rem},
};

use hpuc_ir::{IR, OpId, OpIdRaw, ValId, ValMap, traversal::OpWalker};
use hpuc_langs::{
    doplang::{Argument, Doplang, Operations as DopOp},
    hpulang::{Hpulang, Operations as HpuOp},
};
use hpuc_sim::hpu::HpuConfig;
use hpuc_utils::{CollectInSmallVec, CollectInVec, SmallMap, SmallVec, StoreIndex, svec};

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
            RegisterState::Empty => write!(f, "  "),
            RegisterState::Fresh(val_id) => write!(f, "\x1b[1m{:2}\x1b[0m", val_id.as_usize()),
            RegisterState::Storing(val_id) => write!(f, "{:2}", val_id.as_usize()),
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

// TODO Check that indeed, we do dst = src in some cases.

/// A structure representing a register file.
///
/// In our case, a regfile is a map from registers identifiers to optional values stored at a certain point in time.
///
/// # Notes:
/// For many-lut pbses, we need to be able to store in contiguous ranges of the regfile (with proper alignment).
/// For this reasons, some methods take a const `RANGE_SIZE`, which represents the size of the expected range to be targetted.
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
        // The uses are ordered by construction, so the first use that is greater or equal to now is the next use.
        self.uses.iter().copied().find(|u| *u >= point)
    }
}

#[derive(Debug)]
struct LiveRangeMap(ValMap<LiveRange>);

impl LiveRangeMap {
    pub fn from_scheduled_ir(ir: &IR<Hpulang>, schedule: impl OpWalker) -> Self {
        let mut live_ranges: ValMap<LiveRange> = ir.empty_valmap();
        for (point, op) in ir.walk_ops_with(schedule).enumerate() {
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
            .filter(move |(_, live_range)| point == live_range.to() + 1)
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
/// The allocator directly emits an ir in the doplang dialect. This means that no further scheduling will be performed on the spilled instructions.
/// If register pressure turns out to be a big contender, it may make sense to try to better schedule the spills (to lift them slightly up in the
/// stream, ensuring that no time is spent waiting for them).
struct Allocator<'ir> {
    input: &'ir IR<Hpulang>,
    output: IR<Doplang>,
    schedule: Vec<OpId>,
    live_ranges: LiveRangeMap,
    register_file: RegisterFile,
    heap: Heap,
    translation_map: ValMap<ValState>,
    current_ctx: ValId,
    point: OpIdRaw,
}

impl<'ir> Allocator<'ir> {
    pub fn init(ir: &IR<Hpulang>, schedule: impl OpWalker, nregs: usize) -> Allocator {
        let schedule = schedule.covec();
        let live_ranges = LiveRangeMap::from_scheduled_ir(ir, schedule.iter().copied());
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
            schedule,
            live_ranges,
            register_file,
            translation_map,
            current_ctx,
            point,
            heap,
        }
    }

    fn step(&mut self) {
        self.point += 1;
        self.register_file.use_fresh();
        self.register_file.purge(
            self.live_ranges
                .purgeable_iter(self.point)
                .filter_map(|valid| match self.translation_map[valid] {
                    ValState::Registered { reg } => Some(reg),
                    ValState::Spilled { .. } => None,
                }),
        );
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
        for op in self
            .input
            .walk_ops_with(self.schedule.iter().copied())
            .covec()
            .into_iter()
        {
            // println!("{}     {}", self.register_file, self.heap);
            self.step();

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
                        lut: Argument::lut_id(lut)
                    });
                }
                HpuOp::PbsF { lut } => {
                    let [r_dst] = self.get_dst_registers([rets[0]]);
                    let r_src = self.get_src_register(args[0]);
                    self.add_dop(DopOp::PBS_F {
                        dst: Argument::ct_reg(r_dst.0),
                        src: Argument::ct_reg(r_src.0),
                        lut: Argument::lut_id(lut)
                    });
                }
                HpuOp::Pbs2 { lut } => {
                    let [r_dst, ..] = self.get_dst_registers([rets[0], rets[1]]);
                    let r_src = self.get_src_register(args[0]);
                    self.add_dop(DopOp::PBS_ML2 {
                        dst: Argument::ct_reg2(r_dst.0),
                        src: Argument::ct_reg(r_src.0),
                        lut: Argument::lut_id(lut)
                    });
                }
                HpuOp::Pbs2F { lut } => {
                    let [r_dst, ..] = self.get_dst_registers([rets[0], rets[1]]);
                    let r_src = self.get_src_register(args[0]);
                    self.add_dop(DopOp::PBS_ML2_F {
                        dst: Argument::ct_reg2(r_dst.0),
                        src: Argument::ct_reg(r_src.0),
                        lut: Argument::lut_id(lut)
                    });
                }
                HpuOp::Pbs4 { lut } => {
                    let [r_dst, ..] = self.get_dst_registers([rets[0], rets[1], rets[2], rets[3]]);
                    let r_src = self.get_src_register(args[0]);
                    self.add_dop(DopOp::PBS_ML4 {
                        dst: Argument::ct_reg4(r_dst.0),
                        src: Argument::ct_reg(r_src.0),
                        lut: Argument::lut_id(lut)
                    });
                }
                HpuOp::Pbs4F { lut } => {
                    let [r_dst, ..] = self.get_dst_registers([rets[0], rets[1], rets[2], rets[3]]);
                    let r_src = self.get_src_register(args[0]);
                    self.add_dop(DopOp::PBS_ML4_F {
                        dst: Argument::ct_reg4(r_dst.0),
                        src: Argument::ct_reg(r_src.0),
                        lut: Argument::lut_id(lut)
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
                        lut: Argument::lut_id(lut)
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
                        lut: Argument::lut_id(lut)
                    });
                }
            }
        }
        self.output
    }
}

pub fn allocate_registers(ir: &IR<Hpulang>, schedule: impl OpWalker, config: &HpuConfig) -> IR<Doplang> {
    let allocator = Allocator::init(ir, schedule, config.regf_size);
    allocator.allocate_registers()
}

#[cfg(test)]
mod test {
    use hpuc_ir::{IR, scheduling::forward::ForwardScheduler, translation::Translator};
    use hpuc_langs::{doplang::Doplang, ioplang::Ioplang};
    use hpuc_sim::hpu::{HpuConfig, PhysicalConfig};

    use crate::{scheduler::Scheduler, test::{get_add_ir, get_cmp_ir, get_sub_ir}, translation::IoplangToHpulang};

    use super::allocate_registers;

    fn pipeline(ir: &IR<Ioplang>) -> IR<Doplang> {
        let mut ir = IoplangToHpulang.translate(&ir);
        let mut config = HpuConfig::from(PhysicalConfig::gaussian_64b_fast());
        config.regf_size = 10;
        let mut scheduler = Scheduler::init(&ir, &config);
        let schedule = scheduler.schedule(&ir);
        let flusher = scheduler.into_flusher();
        flusher.apply_flushes(&mut ir);
        let allocated = allocate_registers(&ir, schedule.get_walker(), &config);
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
            %9 : Ctx = ADD<R(0), R(0), R(7)>(%8);
            %10 : Ctx = LD<R(7), TC(1, 1)>(%9);
            %11 : Ctx = LD<R(8), TC(1, 2)>(%10);
            %12 : Ctx = LD<R(9), TC(1, 3)>(%11);
            %13 : Ctx = ST<CT_H(0), R(6)>(%12);
            %14 : Ctx = LD<R(6), TC(1, 4)>(%13);
            %15 : Ctx = ADD<R(1), R(1), R(7)>(%14);
            %16 : Ctx = ST<CT_H(1), R(6)>(%15);
            %17 : Ctx = PBS2<R(6, 2), R(0), LUT(26)>(%16);
            %18 : Ctx = LD<R(0), TC(1, 5)>(%17);
            %19 : Ctx = ST<CT_H(2), R(6)>(%18);
            %20 : Ctx = LD<R(6), TC(1, 6)>(%19);
            %21 : Ctx = ADD<R(2), R(2), R(8)>(%20);
            %22 : Ctx = PBS<R(8), R(1), LUT(47)>(%21);
            %23 : Ctx = ADD<R(3), R(3), R(9)>(%22);
            %24 : Ctx = PBS<R(9), R(2), LUT(48)>(%23);
            %25 : Ctx = ST<CT_H(3), R(2)>(%24);
            %26 : Ctx = LD<R(2), CT_H(1)>(%25);
            %27 : Ctx = ADD<R(4), R(4), R(2)>(%26);
            %28 : Ctx = ST<CT_H(4), R(3)>(%27);
            %29 : Ctx = ST<CT_H(5), R(9)>(%28);
            %30 : Ctx = LD<R(9), CT_H(4)>(%29);
            %31 : Ctx = PBS<R(3), R(9), LUT(49)>(%30);
            %32 : Ctx = ADD<R(0), R(5), R(0)>(%31);
            %33 : Ctx = PBS<R(5), R(4), LUT(47)>(%32);
            %34 : Ctx = ST<CT_H(6), R(4)>(%33);
            %35 : Ctx = LD<R(4), CT_H(0)>(%34);
            %36 : Ctx = ADD<R(6), R(4), R(6)>(%35);
            %37 : Ctx = ST<CT_H(7), R(0)>(%36);
            %38 : Ctx = LD<R(9), CT_H(7)>(%37);
            %39 : Ctx = PBS<R(0), R(9), LUT(48)>(%38);
            %40 : Ctx = ST<CT_H(8), R(6)>(%39);
            %41 : Ctx = LD<R(9), CT_H(8)>(%40);
            %42 : Ctx = PBSF<R(6), R(9), LUT(49)>(%41);
            %43 : Ctx = ADD<R(1), R(1), R(7)>(%42);
            %44 : Ctx = LD<R(9), CT_H(2)>(%43);
            %45 : Ctx = ST<TC(0, 0), R(9)>(%44);
            %46 : Ctx = ADD<R(7), R(8), R(7)>(%45);
            %47 : Ctx = ST<TC(0, 1), R(1)>(%46);
            %48 : Ctx = ADD<R(0), R(0), R(5)>(%47);
            %49 : Ctx = PBSF<R(1), R(7), LUT(44)>(%48);
            %50 : Ctx = LD<R(8), CT_H(5)>(%49);
            %51 : Ctx = ADD<R(7), R(8), R(7)>(%50);
            %52 : Ctx = ADD<R(6), R(6), R(0)>(%51);
            %53 : Ctx = ST<CT_H(9), R(6)>(%52);
            %54 : Ctx = PBSF<R(6), R(7), LUT(45)>(%53);
            %55 : Ctx = ADD<R(3), R(3), R(7)>(%54);
            %56 : Ctx = LD<R(7), CT_H(3)>(%55);
            %57 : Ctx = ADD<R(1), R(7), R(1)>(%56);
            %58 : Ctx = PBSF<R(3), R(3), LUT(46)>(%57);
            %59 : Ctx = ST<CT_H(10), R(0)>(%58);
            %60 : Ctx = LD<R(0), CT_H(4)>(%59);
            %61 : Ctx = ADD<R(6), R(0), R(6)>(%60);
            %62 : Ctx = ST<TC(0, 2), R(1)>(%61);
            %63 : Ctx = ST<TC(0, 3), R(6)>(%62);
            %64 : Ctx = ADD<R(1), R(5), R(3)>(%63);
            %65 : Ctx = LD<R(6), CT_H(10)>(%64);
            %66 : Ctx = ADD<R(5), R(6), R(3)>(%65);
            %67 : Ctx = PBS<R(1), R(1), LUT(46)>(%66);
            %68 : Ctx = ST<CT_H(11), R(1)>(%67);
            %69 : Ctx = LD<R(1), CT_H(9)>(%68);
            %70 : Ctx = ADD<R(3), R(1), R(3)>(%69);
            %71 : Ctx = PBS<R(5), R(5), LUT(44)>(%70);
            %72 : Ctx = PBS<R(3), R(3), LUT(45)>(%71);
            %73 : Ctx = ST<CT_H(12), R(3)>(%72);
            %74 : Ctx = LD<R(9), CT_H(6)>(%73);
            %75 : Ctx = LD<R(8), CT_H(11)>(%74);
            %76 : Ctx = ADD<R(3), R(9), R(8)>(%75);
            %77 : Ctx = LD<R(9), CT_H(7)>(%76);
            %78 : Ctx = ADD<R(5), R(9), R(5)>(%77);
            %79 : Ctx = ST<TC(0, 4), R(3)>(%78);
            %80 : Ctx = LD<R(9), CT_H(8)>(%79);
            %81 : Ctx = LD<R(8), CT_H(12)>(%80);
            %82 : Ctx = ADD<R(3), R(9), R(8)>(%81);
            %83 : Ctx = ST<TC(0, 5), R(5)>(%82);
            %84 : Ctx = ST<TC(0, 6), R(3)>(%83);
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
            %9 : Ctx = SSUB<R(7), R(7), PT_I(3)>(%8);
            %10 : Ctx = LD<R(8), TC(1, 1)>(%9);
            %11 : Ctx = LD<R(9), TC(1, 2)>(%10);
            %12 : Ctx = ST<CT_H(0), R(6)>(%11);
            %13 : Ctx = LD<R(6), TC(1, 3)>(%12);
            %14 : Ctx = ST<CT_H(1), R(5)>(%13);
            %15 : Ctx = LD<R(5), TC(1, 4)>(%14);
            %16 : Ctx = SSUB<R(8), R(8), PT_I(3)>(%15);
            %17 : Ctx = ST<CT_H(2), R(4)>(%16);
            %18 : Ctx = LD<R(4), TC(1, 5)>(%17);
            %19 : Ctx = ST<CT_H(3), R(3)>(%18);
            %20 : Ctx = LD<R(3), TC(1, 6)>(%19);
            %21 : Ctx = SSUB<R(9), R(9), PT_I(3)>(%20);
            %22 : Ctx = SSUB<R(6), R(6), PT_I(3)>(%21);
            %23 : Ctx = SSUB<R(5), R(5), PT_I(3)>(%22);
            %24 : Ctx = SSUB<R(4), R(4), PT_I(3)>(%23);
            %25 : Ctx = SSUB<R(3), R(3), PT_I(3)>(%24);
            %26 : Ctx = ADD<R(0), R(0), R(7)>(%25);
            %27 : Ctx = ADD<R(1), R(1), R(8)>(%26);
            %28 : Ctx = ST<CT_H(4), R(6)>(%27);
            %29 : Ctx = PBS2<R(6, 2), R(0), LUT(26)>(%28);
            %30 : Ctx = ADD<R(0), R(2), R(9)>(%29);
            %31 : Ctx = PBS<R(2), R(1), LUT(47)>(%30);
            %32 : Ctx = LD<R(9), CT_H(3)>(%31);
            %33 : Ctx = ST<CT_H(5), R(2)>(%32);
            %34 : Ctx = LD<R(2), CT_H(4)>(%33);
            %35 : Ctx = ADD<R(8), R(9), R(2)>(%34);
            %36 : Ctx = ST<CT_H(6), R(0)>(%35);
            %37 : Ctx = ST<CT_H(7), R(6)>(%36);
            %38 : Ctx = LD<R(6), CT_H(6)>(%37);
            %39 : Ctx = PBS<R(0), R(6), LUT(48)>(%38);
            %40 : Ctx = LD<R(6), CT_H(2)>(%39);
            %41 : Ctx = ADD<R(5), R(6), R(5)>(%40);
            %42 : Ctx = ST<CT_H(8), R(8)>(%41);
            %43 : Ctx = ST<CT_H(9), R(0)>(%42);
            %44 : Ctx = LD<R(0), CT_H(8)>(%43);
            %45 : Ctx = PBS<R(8), R(0), LUT(49)>(%44);
            %46 : Ctx = LD<R(0), CT_H(1)>(%45);
            %47 : Ctx = ADD<R(4), R(0), R(4)>(%46);
            %48 : Ctx = ST<CT_H(10), R(5)>(%47);
            %49 : Ctx = ST<CT_H(11), R(8)>(%48);
            %50 : Ctx = LD<R(8), CT_H(10)>(%49);
            %51 : Ctx = PBS<R(5), R(8), LUT(47)>(%50);
            %52 : Ctx = LD<R(8), CT_H(0)>(%51);
            %53 : Ctx = ADD<R(3), R(8), R(3)>(%52);
            %54 : Ctx = ST<CT_H(12), R(4)>(%53);
            %55 : Ctx = ST<CT_H(13), R(5)>(%54);
            %56 : Ctx = LD<R(5), CT_H(12)>(%55);
            %57 : Ctx = PBS<R(4), R(5), LUT(48)>(%56);
            %58 : Ctx = ST<CT_H(14), R(3)>(%57);
            %59 : Ctx = LD<R(5), CT_H(14)>(%58);
            %60 : Ctx = PBS<R(3), R(5), LUT(49)>(%59);
            %61 : Ctx = ADD<R(1), R(1), R(7)>(%60);
            %62 : Ctx = ST<CT_H(15), R(3)>(%61);
            %63 : Ctx = LD<R(3), CT_H(7)>(%62);
            %64 : Ctx = PBS<R(5), R(3), LUT(1)>(%63);
            %65 : Ctx = ST<CT_H(16), R(5)>(%64);
            %66 : Ctx = LD<R(5), CT_H(5)>(%65);
            %67 : Ctx = ADD<R(7), R(5), R(7)>(%66);
            %68 : Ctx = PBS<R(1), R(1), LUT(1)>(%67);
            %69 : Ctx = ST<CT_H(17), R(1)>(%68);
            %70 : Ctx = LD<R(1), CT_H(13)>(%69);
            %71 : Ctx = ADD<R(4), R(4), R(1)>(%70);
            %72 : Ctx = PBSF<R(1), R(7), LUT(44)>(%71);
            %73 : Ctx = ST<CT_H(18), R(1)>(%72);
            %74 : Ctx = LD<R(1), CT_H(9)>(%73);
            %75 : Ctx = ADD<R(7), R(1), R(7)>(%74);
            %76 : Ctx = ST<CT_H(19), R(4)>(%75);
            %77 : Ctx = ST<CT_H(20), R(7)>(%76);
            %78 : Ctx = LD<R(7), CT_H(15)>(%77);
            %79 : Ctx = LD<R(9), CT_H(19)>(%78);
            %80 : Ctx = ADD<R(4), R(7), R(9)>(%79);
            %81 : Ctx = ST<CT_H(21), R(4)>(%80);
            %82 : Ctx = LD<R(4), CT_H(16)>(%81);
            %83 : Ctx = ST<TC(0, 0), R(4)>(%82);
            %84 : Ctx = LD<R(9), CT_H(17)>(%83);
            %85 : Ctx = ST<TC(0, 1), R(9)>(%84);
            %86 : Ctx = LD<R(8), CT_H(11)>(%85);
            %87 : Ctx = LD<R(7), CT_H(20)>(%86);
            %88 : Ctx = ADD<R(9), R(8), R(7)>(%87);
            %89 : Ctx = PBS<R(7), R(7), LUT(45)>(%88);
            %90 : Ctx = ST<CT_H(22), R(7)>(%89);
            %91 : Ctx = ST<CT_H(23), R(9)>(%90);
            %92 : Ctx = LD<R(9), CT_H(6)>(%91);
            %93 : Ctx = LD<R(8), CT_H(18)>(%92);
            %94 : Ctx = ADD<R(7), R(9), R(8)>(%93);
            %95 : Ctx = LD<R(8), CT_H(23)>(%94);
            %96 : Ctx = PBS<R(9), R(8), LUT(46)>(%95);
            %97 : Ctx = PBSF<R(7), R(7), LUT(1)>(%96);
            %98 : Ctx = ST<CT_H(24), R(9)>(%97);
            %99 : Ctx = LD<R(8), CT_H(8)>(%98);
            %100 : Ctx = ST<CT_H(25), R(7)>(%99);
            %101 : Ctx = LD<R(7), CT_H(22)>(%100);
            %102 : Ctx = ADD<R(9), R(8), R(7)>(%101);
            %103 : Ctx = ST<CT_H(26), R(9)>(%102);
            %104 : Ctx = LD<R(9), CT_H(25)>(%103);
            %105 : Ctx = ST<TC(0, 2), R(9)>(%104);
            %106 : Ctx = LD<R(8), CT_H(13)>(%105);
            %107 : Ctx = LD<R(7), CT_H(24)>(%106);
            %108 : Ctx = ADD<R(9), R(8), R(7)>(%107);
            %109 : Ctx = ST<CT_H(27), R(9)>(%108);
            %110 : Ctx = LD<R(8), CT_H(26)>(%109);
            %111 : Ctx = PBS<R(9), R(8), LUT(1)>(%110);
            %112 : Ctx = ST<CT_H(28), R(9)>(%111);
            %113 : Ctx = LD<R(7), CT_H(19)>(%112);
            %114 : Ctx = LD<R(8), CT_H(24)>(%113);
            %115 : Ctx = ADD<R(9), R(7), R(8)>(%114);
            %116 : Ctx = ST<CT_H(29), R(9)>(%115);
            %117 : Ctx = LD<R(8), CT_H(27)>(%116);
            %118 : Ctx = PBS<R(9), R(8), LUT(46)>(%117);
            %119 : Ctx = ST<CT_H(30), R(9)>(%118);
            %120 : Ctx = LD<R(8), CT_H(21)>(%119);
            %121 : Ctx = LD<R(7), CT_H(24)>(%120);
            %122 : Ctx = ADD<R(9), R(8), R(7)>(%121);
            %123 : Ctx = ST<CT_H(31), R(9)>(%122);
            %124 : Ctx = LD<R(8), CT_H(29)>(%123);
            %125 : Ctx = PBS<R(9), R(8), LUT(44)>(%124);
            %126 : Ctx = ST<CT_H(32), R(9)>(%125);
            %127 : Ctx = LD<R(8), CT_H(31)>(%126);
            %128 : Ctx = PBSF<R(9), R(8), LUT(45)>(%127);
            %129 : Ctx = ST<CT_H(33), R(9)>(%128);
            %130 : Ctx = LD<R(9), CT_H(28)>(%129);
            %131 : Ctx = ST<TC(0, 3), R(9)>(%130);
            %132 : Ctx = LD<R(8), CT_H(10)>(%131);
            %133 : Ctx = LD<R(7), CT_H(30)>(%132);
            %134 : Ctx = ADD<R(9), R(8), R(7)>(%133);
            %135 : Ctx = ST<CT_H(34), R(9)>(%134);
            %136 : Ctx = LD<R(8), CT_H(12)>(%135);
            %137 : Ctx = LD<R(7), CT_H(32)>(%136);
            %138 : Ctx = ADD<R(9), R(8), R(7)>(%137);
            %139 : Ctx = ST<CT_H(35), R(9)>(%138);
            %140 : Ctx = LD<R(8), CT_H(34)>(%139);
            %141 : Ctx = PBS<R(9), R(8), LUT(1)>(%140);
            %142 : Ctx = ST<CT_H(36), R(9)>(%141);
            %143 : Ctx = LD<R(8), CT_H(14)>(%142);
            %144 : Ctx = LD<R(7), CT_H(33)>(%143);
            %145 : Ctx = ADD<R(9), R(8), R(7)>(%144);
            %146 : Ctx = ST<CT_H(37), R(9)>(%145);
            %147 : Ctx = LD<R(8), CT_H(35)>(%146);
            %148 : Ctx = PBS<R(9), R(8), LUT(1)>(%147);
            %149 : Ctx = ST<CT_H(38), R(9)>(%148);
            %150 : Ctx = LD<R(8), CT_H(37)>(%149);
            %151 : Ctx = PBS<R(9), R(8), LUT(1)>(%150);
            %152 : Ctx = ST<CT_H(39), R(9)>(%151);
            %153 : Ctx = LD<R(9), CT_H(36)>(%152);
            %154 : Ctx = ST<TC(0, 4), R(9)>(%153);
            %155 : Ctx = LD<R(9), CT_H(38)>(%154);
            %156 : Ctx = ST<TC(0, 5), R(9)>(%155);
            %157 : Ctx = LD<R(9), CT_H(39)>(%156);
            %158 : Ctx = ST<TC(0, 6), R(9)>(%157);
            ");
    }

    #[test]
    fn test_allocate_cmp_ir() {
        let ir = pipeline(&get_cmp_ir(16, 2, 2));
        ir.check_ir("
            %0 : Ctx = _INIT();
            %1 : Ctx = LD<R(0), TC(0, 0)>(%0);
            %2 : Ctx = LD<R(1), TC(0, 1)>(%1);
            %3 : Ctx = MAC<R(0), R(1), R(0), PT_I(4)>(%2);
            %4 : Ctx = LD<R(1), TC(0, 2)>(%3);
            %5 : Ctx = LD<R(2), TC(0, 3)>(%4);
            %6 : Ctx = LD<R(3), TC(0, 4)>(%5);
            %7 : Ctx = LD<R(4), TC(0, 5)>(%6);
            %8 : Ctx = MAC<R(1), R(2), R(1), PT_I(4)>(%7);
            %9 : Ctx = LD<R(2), TC(0, 6)>(%8);
            %10 : Ctx = LD<R(5), TC(0, 7)>(%9);
            %11 : Ctx = LD<R(6), TC(1, 0)>(%10);
            %12 : Ctx = LD<R(7), TC(1, 1)>(%11);
            %13 : Ctx = MAC<R(3), R(4), R(3), PT_I(4)>(%12);
            %14 : Ctx = LD<R(4), TC(1, 2)>(%13);
            %15 : Ctx = LD<R(8), TC(1, 3)>(%14);
            %16 : Ctx = LD<R(9), TC(1, 4)>(%15);
            %17 : Ctx = ST<CT_H(0), R(3)>(%16);
            %18 : Ctx = LD<R(3), TC(1, 5)>(%17);
            %19 : Ctx = MAC<R(2), R(5), R(2), PT_I(4)>(%18);
            %20 : Ctx = LD<R(5), TC(1, 6)>(%19);
            %21 : Ctx = ST<CT_H(1), R(2)>(%20);
            %22 : Ctx = LD<R(2), TC(1, 7)>(%21);
            %23 : Ctx = MAC<R(6), R(7), R(6), PT_I(4)>(%22);
            %24 : Ctx = MAC<R(4), R(8), R(4), PT_I(4)>(%23);
            %25 : Ctx = MAC<R(3), R(3), R(9), PT_I(4)>(%24);
            %26 : Ctx = MAC<R(2), R(2), R(5), PT_I(4)>(%25);
            %27 : Ctx = SUB<R(0), R(0), R(6)>(%26);
            %28 : Ctx = SUB<R(1), R(1), R(4)>(%27);
            %29 : Ctx = PBS<R(0), R(0), LUT(10)>(%28);
            %30 : Ctx = LD<R(4), CT_H(0)>(%29);
            %31 : Ctx = SUB<R(3), R(4), R(3)>(%30);
            %32 : Ctx = PBS<R(1), R(1), LUT(10)>(%31);
            %33 : Ctx = LD<R(5), CT_H(1)>(%32);
            %34 : Ctx = SUB<R(2), R(5), R(2)>(%33);
            %35 : Ctx = PBS<R(3), R(3), LUT(10)>(%34);
            %36 : Ctx = PBSF<R(2), R(2), LUT(10)>(%35);
            %37 : Ctx = MAC<R(0), R(1), R(0), PT_I(4)>(%36);
            %38 : Ctx = MAC<R(1), R(2), R(3), PT_I(4)>(%37);
            %39 : Ctx = PBS<R(0), R(0), LUT(11)>(%38);
            %40 : Ctx = PBSF<R(1), R(1), LUT(11)>(%39);
            %41 : Ctx = MAC<R(0), R(1), R(0), PT_I(4)>(%40);
            %42 : Ctx = PBS<R(0), R(0), LUT(27)>(%41);
            %43 : Ctx = ST<TC(0, 0), R(0)>(%42);
            ");
    }
}
