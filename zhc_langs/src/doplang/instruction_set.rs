use super::type_system::DopTypeSystem;
use serde::Serialize;
use std::fmt::{Debug, Display};
use zhc_crypto::integer_semantics::lut::{LutId, LutRegistry};
use zhc_ir::{DialectInstructionSet, Format, FormatContext, Signature, sig};

/// Register address mask that compares all bits (single-output PBS or
/// plain register).
pub const MASK_NONE: usize = usize::MAX;
/// Register address mask that ignores the lowest bit, grouping pairs
/// of consecutive registers produced by a 2-output PBS.
pub const MASK_PBS2: usize = usize::MAX << 1;
/// Register address mask that ignores the two lowest bits, grouping
/// quads of consecutive registers produced by a 4-output PBS.
pub const MASK_PBS4: usize = usize::MAX << 2;
/// Register address mask that ignores the three lowest bits, grouping
/// octets of consecutive registers produced by an 8-output PBS.
pub const MASK_PBS8: usize = usize::MAX << 3;

// -------------------------------------------------------------------------------------------
// Standalone operand types.
//
// Each of these used to be a variant of one large `Argument` enum, inlined into every
// `DopInstructionSet` field regardless of which kinds were actually valid there — which meant
// e.g. `ADD`'s destination could hold a `PtConst` just as well as a `CtReg`, with only a runtime
// check (in the asm parser) telling them apart after the fact. Splitting each kind into its own
// type lets `DopInstructionSet`'s fields declare exactly which kinds they accept, so the invalid
// combination can no longer be constructed at all.
// -------------------------------------------------------------------------------------------

/// A constant plaintext immediate value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PtConst {
    pub val: u8,
}

impl PtConst {
    pub fn new(val: u8) -> Self {
        Self { val }
    }

    pub fn asm(&self, _lreg: &LutRegistry) -> String {
        format!("{}", self.val)
    }
}

impl Display for PtConst {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PT_I({})", self.val)
    }
}

/// A ciphertext block located on the heap.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CtHeap {
    pub addr: usize,
}

impl CtHeap {
    pub fn new(addr: usize) -> Self {
        Self { addr }
    }

    pub fn asm(&self, _lreg: &LutRegistry) -> String {
        format!("TH.{}", self.addr)
    }
}

impl Display for CtHeap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CT_H({})", self.addr)
    }
}

/// A ciphertext block located in I/O memory.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CtIo {
    pub addr: usize,
}

impl CtIo {
    pub fn new(addr: usize) -> Self {
        Self { addr }
    }

    pub fn asm(&self, _lreg: &LutRegistry) -> String {
        format!("@{}", self.addr)
    }
}

impl Display for CtIo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CT_IO({})", self.addr)
    }
}

/// A symbolic ciphertext source variable, patched to a physical address by
/// the microcontroller.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CtSrcVar {
    pub id: usize,
    pub block: usize,
}

impl CtSrcVar {
    pub fn new(id: usize, block: usize) -> Self {
        Self { id, block }
    }

    pub fn asm(&self, _lreg: &LutRegistry) -> String {
        format!("TS[{}].{}", self.id, self.block)
    }
}

impl Display for CtSrcVar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TC({}, {})", self.id, self.block)
    }
}

/// A symbolic ciphertext destination variable, patched to a physical address
/// by the microcontroller.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CtDstVar {
    pub id: usize,
    pub block: usize,
}

impl CtDstVar {
    pub fn new(id: usize, block: usize) -> Self {
        Self { id, block }
    }

    pub fn asm(&self, _lreg: &LutRegistry) -> String {
        format!("TD[{}].{}", self.id, self.block)
    }
}

impl Display for CtDstVar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TC({}, {})", self.id, self.block)
    }
}

/// A symbolic plaintext source variable, patched to a `PtConst` by the
/// microcontroller.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PtSrcVar {
    pub id: usize,
    pub block: usize,
}

impl PtSrcVar {
    pub fn new(id: usize, block: usize) -> Self {
        Self { id, block }
    }

    pub fn asm(&self, _lreg: &LutRegistry) -> String {
        format!("TI[{}].{}", self.id, self.block)
    }
}

impl Display for PtSrcVar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TI({}, {})", self.id, self.block)
    }
}

/// A physical ciphertext register with an alignment mask.
///
/// Equality is masked: only the bits selected by the intersection of both
/// masks are compared. This allows multi-output PBS results (which occupy
/// consecutive aligned registers) to compare equal when their masks reflect
/// the output arity.
#[derive(Debug, Clone, Copy, Eq, Hash)]
pub struct CtReg {
    pub mask: usize,
    pub addr: usize,
}

impl CtReg {
    /// Creates a `CtReg` with [`MASK_NONE`] (all bits significant).
    pub fn new(addr: impl Into<usize>) -> Self {
        Self {
            mask: MASK_NONE,
            addr: addr.into(),
        }
    }

    /// Creates a `CtReg` with [`MASK_PBS2`] (lowest bit ignored).
    pub fn ml2(addr: impl Into<usize>) -> Self {
        Self {
            mask: MASK_PBS2,
            addr: addr.into(),
        }
    }

    /// Creates a `CtReg` with [`MASK_PBS4`] (two lowest bits ignored).
    pub fn ml4(addr: impl Into<usize>) -> Self {
        Self {
            mask: MASK_PBS4,
            addr: addr.into(),
        }
    }

    /// Creates a `CtReg` with [`MASK_PBS8`] (three lowest bits ignored).
    pub fn ml8(addr: impl Into<usize>) -> Self {
        Self {
            mask: MASK_PBS8,
            addr: addr.into(),
        }
    }

    pub fn asm(&self, _lreg: &LutRegistry) -> String {
        format!("R{}", self.addr)
    }
}

impl PartialEq for CtReg {
    fn eq(&self, other: &Self) -> bool {
        ((self.addr ^ other.addr) & (self.mask & other.mask)) == 0
    }
}

impl Display for CtReg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.mask {
            MASK_NONE => write!(f, "R({})", self.addr),
            MASK_PBS2 => write!(f, "R({}, 2)", self.addr),
            MASK_PBS4 => write!(f, "R({}, 4)", self.addr),
            MASK_PBS8 => write!(f, "R({}, 8)", self.addr),
            _ => unreachable!(),
        }
    }
}

/// A reference to a lookup table registered in a [`LutRegistry`].
///
/// Named `LutRef` rather than `LutId` to stay distinct from
/// [`zhc_crypto::integer_semantics::lut::LutId`], the registry's own compact identifier type
/// (which this simply wraps).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LutRef {
    pub id: usize,
}

impl LutRef {
    pub fn new(id: impl Into<usize>) -> Self {
        Self { id: id.into() }
    }

    pub fn asm(&self, lreg: &LutRegistry) -> String {
        format!("Pbs{}", lreg.get_raw_lut(&LutId(self.id)).name())
    }
}

impl From<LutId> for LutRef {
    fn from(value: LutId) -> Self {
        Self { id: value.0 }
    }
}

impl Display for LutRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LUT({})", self.id)
    }
}

/// A user event flag: a hash/UUID for matching Ucore instructions together.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct UserFlag {
    pub flag: u8,
}

impl UserFlag {
    pub fn new(flag: u8) -> Self {
        Self { flag }
    }

    pub fn asm(&self, _lreg: &LutRegistry) -> String {
        format!("F{}", self.flag)
    }
}

impl Display for UserFlag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "F({})", self.flag)
    }
}

/// A board (virtual HPU) identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct VirtId {
    pub id: u8,
}

impl VirtId {
    pub fn new(id: u8) -> Self {
        Self { id }
    }

    pub fn asm(&self, _lreg: &LutRegistry) -> String {
        format!("N{}", self.id)
    }
}

impl Display for VirtId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "N({})", self.id)
    }
}

// -------------------------------------------------------------------------------------------
// Role enums: for fields that legitimately accept one of a small, fixed set of operand kinds.
// -------------------------------------------------------------------------------------------

/// A ciphertext-scalar immediate: either a literal constant or a patchable template.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PtArg {
    Const(PtConst),
    Var(PtSrcVar),
}

impl PtArg {
    pub fn cst(val: u8) -> Self {
        Self::Const(PtConst::new(val))
    }

    pub fn var(id: usize, block: usize) -> Self {
        Self::Var(PtSrcVar::new(id, block))
    }

    pub fn asm(&self, lreg: &LutRegistry) -> String {
        match self {
            PtArg::Const(inner) => inner.asm(lreg),
            PtArg::Var(inner) => inner.asm(lreg),
        }
    }
}

impl From<PtConst> for PtArg {
    fn from(value: PtConst) -> Self {
        Self::Const(value)
    }
}

impl From<PtSrcVar> for PtArg {
    fn from(value: PtSrcVar) -> Self {
        Self::Var(value)
    }
}

impl Display for PtArg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PtArg::Const(inner) => Display::fmt(inner, f),
            PtArg::Var(inner) => Display::fmt(inner, f),
        }
    }
}

/// A ciphertext memory location: one of the four addressing modes a `LD`/`ST`/multi-HPU slot
/// operand may name.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CtMem {
    Heap(CtHeap),
    Io(CtIo),
    Src(CtSrcVar),
    Dst(CtDstVar),
}

impl CtMem {
    pub fn heap(addr: usize) -> Self {
        Self::Heap(CtHeap::new(addr))
    }

    pub fn io(addr: usize) -> Self {
        Self::Io(CtIo::new(addr))
    }

    pub fn src(id: usize, block: usize) -> Self {
        Self::Src(CtSrcVar::new(id, block))
    }

    pub fn dst(id: usize, block: usize) -> Self {
        Self::Dst(CtDstVar::new(id, block))
    }

    pub fn asm(&self, lreg: &LutRegistry) -> String {
        match self {
            CtMem::Heap(inner) => inner.asm(lreg),
            CtMem::Io(inner) => inner.asm(lreg),
            CtMem::Src(inner) => inner.asm(lreg),
            CtMem::Dst(inner) => inner.asm(lreg),
        }
    }
}

impl From<CtHeap> for CtMem {
    fn from(value: CtHeap) -> Self {
        Self::Heap(value)
    }
}

impl From<CtIo> for CtMem {
    fn from(value: CtIo) -> Self {
        Self::Io(value)
    }
}

impl From<CtSrcVar> for CtMem {
    fn from(value: CtSrcVar) -> Self {
        Self::Src(value)
    }
}

impl From<CtDstVar> for CtMem {
    fn from(value: CtDstVar) -> Self {
        Self::Dst(value)
    }
}

impl Display for CtMem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CtMem::Heap(inner) => Display::fmt(inner, f),
            CtMem::Io(inner) => Display::fmt(inner, f),
            CtMem::Src(inner) => Display::fmt(inner, f),
            CtMem::Dst(inner) => Display::fmt(inner, f),
        }
    }
}

/// Hardware pipeline lane to which a DOP instruction is dispatched.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum Affinity {
    /// Arithmetic-logic unit: register-to-register operations.
    Alu,
    /// Memory unit: load and store operations.
    Mem,
    /// Programmable bootstrapping unit.
    Pbs,
    /// Control: synchronization and initialization.
    Ctl,
}

impl Display for Affinity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Affinity::Alu => write!(f, "Alu"),
            Affinity::Mem => write!(f, "Mem"),
            Affinity::Pbs => write!(f, "Pbs"),
            Affinity::Ctl => write!(f, "Ctl"),
        }
    }
}

/// HPU hardware instruction set.
///
/// Each variant corresponds to a single hardware opcode. Operands are
/// carried inline as concretely-typed fields (e.g. [`CtReg`], [`CtMem`],
/// [`LutRef`]) rather than through IR SSA references — the DOP stream is a
/// flat, register-allocated instruction sequence.
///
/// Instructions fall into four categories matching the [`Affinity`]
/// lanes: register arithmetic (`ADD`, `SUB`, `MAC`, `ADDS`, `SUBS`,
/// `SSUB`, `MULS`), memory transfer (`LD`, `ST`), programmable
/// bootstrapping (`PBS` family), and control (`_START`, `_END`).
/// Scalar-operand arithmetic variants (`ADDS`, `SUBS`, `SSUB`,
/// `MULS`, `MAC`) take a plaintext immediate in `cst`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub enum DopInstructionSet {
    /// Stream start marker. Produces the initial context
    /// token.
    _START,
    /// Stream end marker. Consumes the context token, ensuring
    /// all preceding instructions have completed.
    _END,
    /// `dst = src1 + src2` — ciphertext addition.
    ADD {
        dst: CtReg,
        src1: CtReg,
        src2: CtReg,
    },
    /// `dst = src1 - src2` — ciphertext subtraction.
    SUB {
        dst: CtReg,
        src1: CtReg,
        src2: CtReg,
    },
    /// `dst = src1 * cst + src2` — multiply-accumulate.
    MAC {
        dst: CtReg,
        src1: CtReg,
        src2: CtReg,
        cst: PtArg,
    },
    /// `dst = src + cst` — ciphertext-scalar addition.
    ADDS { dst: CtReg, src: CtReg, cst: PtArg },
    /// `dst = src - cst` — ciphertext minus scalar.
    SUBS { dst: CtReg, src: CtReg, cst: PtArg },
    /// `dst = cst - src` — scalar minus ciphertext.
    SSUB { dst: CtReg, src: CtReg, cst: PtArg },
    /// `dst = src * cst` — ciphertext-scalar multiplication.
    MULS { dst: CtReg, src: CtReg, cst: PtArg },
    /// Loads a ciphertext block from memory into a register.
    LD { dst: CtReg, src: CtMem },
    /// Stores a ciphertext register to memory.
    ST { dst: CtMem, src: CtReg },
    /// Single-output programmable bootstrapping.
    PBS { dst: CtReg, src: CtReg, lut: LutRef },
    /// 2-output many-LUT programmable bootstrapping.
    PBS_ML2 { dst: CtReg, src: CtReg, lut: LutRef },
    /// 4-output many-LUT programmable bootstrapping.
    PBS_ML4 { dst: CtReg, src: CtReg, lut: LutRef },
    /// 8-output many-LUT programmable bootstrapping.
    PBS_ML8 { dst: CtReg, src: CtReg, lut: LutRef },
    /// Single-output PBS with flush (batch boundary marker).
    PBS_F { dst: CtReg, src: CtReg, lut: LutRef },
    /// 2-output many-LUT PBS with flush.
    PBS_ML2_F { dst: CtReg, src: CtReg, lut: LutRef },
    /// 4-output many-LUT PBS with flush.
    PBS_ML4_F { dst: CtReg, src: CtReg, lut: LutRef },
    /// 8-output many-LUT PBS with flush.
    PBS_ML8_F { dst: CtReg, src: CtReg, lut: LutRef },
    /// Synchronization barrier.
    SYNC,
    /// Wait virtual op for Multi-HPU
    WAIT { flag: UserFlag, slot: Option<CtMem> },
    /// Notify virtual op for Multi-HPU
    NOTIFY {
        virt_id: VirtId,
        flag: UserFlag,
        slot: CtMem,
    },
    /// Load B2B virtual op for Multi-HPU
    LD_B2B { flag: UserFlag, slot: CtMem },
}

impl Format for DopInstructionSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>, _ctx: &FormatContext) -> std::fmt::Result {
        use DopInstructionSet::*;
        match self {
            ADD { dst, src1, src2 } => write!(f, "ADD<{dst}, {src1}, {src2}>"),
            SUB { dst, src1, src2 } => write!(f, "SUB<{dst}, {src1}, {src2}>"),
            MAC {
                dst,
                src1,
                src2,
                cst,
            } => write!(f, "MAC<{dst}, {src1}, {src2}, {cst}>"),
            ADDS { dst, src, cst } => write!(f, "ADDS<{dst}, {src}, {cst}>"),
            SUBS { dst, src, cst } => write!(f, "SUBS<{dst}, {src}, {cst}>"),
            SSUB { dst, src, cst } => write!(f, "SSUB<{dst}, {src}, {cst}>"),
            MULS { dst, src, cst } => write!(f, "MULS<{dst}, {src}, {cst}>"),
            LD { dst, src } => write!(f, "LD<{dst}, {src}>"),
            ST { dst, src } => write!(f, "ST<{dst}, {src}>"),
            PBS { dst, src, lut } => write!(f, "PBS<{dst}, {src}, {lut}>"),
            PBS_ML2 { dst, src, lut } => write!(f, "PBS2<{dst}, {src}, {lut}>"),
            PBS_ML4 { dst, src, lut } => write!(f, "PBS4<{dst}, {src}, {lut}>"),
            PBS_ML8 { dst, src, lut } => write!(f, "PBS8<{dst}, {src}, {lut}>"),
            PBS_F { dst, src, lut } => write!(f, "PBSF<{dst}, {src}, {lut}>"),
            PBS_ML2_F { dst, src, lut } => write!(f, "PBS2F<{dst}, {src}, {lut}>"),
            PBS_ML4_F { dst, src, lut } => write!(f, "PBS4F<{dst}, {src}, {lut}>"),
            PBS_ML8_F { dst, src, lut } => write!(f, "PBS8F<{dst}, {src}, {lut}>"),
            _START => write!(f, "_START"),
            _END => write!(f, "_END"),
            SYNC => write!(f, "SYNC"),
            WAIT { flag, slot } => match slot {
                Some(slot) => write!(f, "WAIT<{flag}, {slot}>"),
                None => write!(f, "WAIT<{flag}>"),
            },
            NOTIFY {
                virt_id,
                flag,
                slot,
            } => write!(f, "NOTIFY<{virt_id}, {flag}, {slot}>"),
            LD_B2B { flag, slot } => write!(f, "LD_B2B<{flag}, {slot}>"),
        }
    }
}

impl Display for DopInstructionSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Format::fmt(self, f, &FormatContext::default())
    }
}

impl DopInstructionSet {
    /// Returns true if this instruction is a PBS.
    pub fn is_pbs(&self) -> bool {
        use DopInstructionSet::*;
        match self {
            PBS { .. } | PBS_ML2 { .. } | PBS_ML4 { .. } | PBS_ML8 { .. } => true,
            PBS_F { .. } | PBS_ML2_F { .. } | PBS_ML4_F { .. } | PBS_ML8_F { .. } => true,
            _ => false,
        }
    }

    /// Returns true if this instruction is a PBS flush variant.
    pub fn is_pbs_flush(&self) -> bool {
        use DopInstructionSet::*;
        match self {
            PBS_F { .. } | PBS_ML2_F { .. } | PBS_ML4_F { .. } | PBS_ML8_F { .. } => true,
            _ => false,
        }
    }

    /// Returns the hardware pipeline lane for this instruction.
    pub fn affinity(&self) -> Affinity {
        use Affinity::*;
        use DopInstructionSet::*;
        match self {
            ADD { .. } => Alu,
            SUB { .. } => Alu,
            MAC { .. } => Alu,
            ADDS { .. } => Alu,
            SUBS { .. } => Alu,
            SSUB { .. } => Alu,
            MULS { .. } => Alu,
            LD { .. } => Mem,
            ST { .. } => Mem,
            PBS { .. } => Pbs,
            PBS_ML2 { .. } => Pbs,
            PBS_ML4 { .. } => Pbs,
            PBS_ML8 { .. } => Pbs,
            PBS_F { .. } => Pbs,
            PBS_ML2_F { .. } => Pbs,
            PBS_ML4_F { .. } => Pbs,
            PBS_ML8_F { .. } => Pbs,
            _START => Ctl,
            _END => Ctl,
            SYNC => Ctl,
            NOTIFY { .. } => Ctl,
            WAIT { .. } => Ctl,
            LD_B2B { .. } => Ctl,
        }
    }
}

impl DialectInstructionSet for DopInstructionSet {
    type TypeSystem = DopTypeSystem;

    fn get_signature(&self) -> Signature<Self::TypeSystem> {
        use DopInstructionSet::*;
        use DopTypeSystem::*;
        match self {
            ADD { .. }
            | SUB { .. }
            | MAC { .. }
            | ADDS { .. }
            | SUBS { .. }
            | SSUB { .. }
            | MULS { .. }
            | LD { .. }
            | ST { .. }
            | PBS { .. }
            | PBS_ML2 { .. }
            | PBS_ML4 { .. }
            | PBS_ML8 { .. }
            | PBS_F { .. }
            | PBS_ML2_F { .. }
            | PBS_ML4_F { .. }
            | PBS_ML8_F { .. }
            | SYNC
            | WAIT { .. }
            | NOTIFY { .. }
            | LD_B2B { .. } => sig![(Ctx(0)) -> (Ctx(0))],
            _START => sig![() -> (Ctx(0))],
            _END => sig![(Ctx(0)) -> ()],
        }
    }
}
