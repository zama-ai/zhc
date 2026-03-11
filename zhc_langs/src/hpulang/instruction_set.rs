use std::fmt::{Debug, Display};

use zhc_ir::{DialectInstructionSet, Format, FormatContext, IR, Signature, sig};
use zhc_utils::iter::CollectInSmallVec;

use super::{HpuLang, type_system::HpuTypeSystem};

/// Plaintext constant inlined into an HPU instruction.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Immediate(pub u8);

impl Display for Immediate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}_imm", self.0)
    }
}

/// Ciphertext input source identifier.
///
/// Encodes which input ciphertext (`src_pos`) and which block within
/// it (`block_pos`) a [`SrcLd`](HpuInstructionSet::SrcLd) loads from.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TSrcId {
    pub src_pos: u32,
    pub block_pos: u32,
}

impl Display for TSrcId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}_tsrc", self.src_pos, self.block_pos)
    }
}

/// Ciphertext output destination identifier.
///
/// Encodes which output ciphertext (`dst_pos`) and which block within
/// it (`block_pos`) a [`DstSt`](HpuInstructionSet::DstSt) stores to.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TDstId {
    pub dst_pos: u32,
    pub block_pos: u32,
}

impl Display for TDstId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}_tdst", self.dst_pos, self.block_pos)
    }
}

/// Numeric lookup table identifier.
///
/// Mapped from symbolic [`Lut1Def`](crate::ioplang::Lut1Def) /
/// [`Lut2Def`](crate::ioplang::Lut2Def) names during IOP-to-HPU
/// translation. Carried through to the DOP dialect and encoded into
/// the hardware translation table.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct LutId(pub usize);

impl Display for LutId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Lut@{}", self.0)
    }
}

/// Plaintext input immediate identifier.
///
/// Encodes which plaintext input (`imm_pos`) and which block within
/// it (`block_pos`) an [`ImmLd`](HpuInstructionSet::ImmLd) loads from.
/// The actual value is resolved at patch time by the microcontroller.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TImmId {
    pub imm_pos: u32,
    pub block_pos: u32,
}

impl Display for TImmId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}_timm", self.imm_pos, self.block_pos)
    }
}

/// Instruction set for the HPU dialect.
///
/// Instructions fall into five categories:
///
/// **Register arithmetic.** Two-operand ciphertext ops (`AddCt`,
/// `SubCt`, `Mac`) and mixed ciphertext-plaintext ops (`AddPt`,
/// `SubPt`, `PtSub`, `MulPt`) take register operands. Constant-scalar
/// variants (`AddCst`, `SubCst`, `CstSub`, `MulCst`) inline the
/// plaintext value as an [`Immediate`], eliminating the need for a
/// separate plaintext register operand. `CstCt` materializes a
/// constant ciphertext register.
///
/// **Memory transfer.** `SrcLd` loads a ciphertext block from an input
/// slot, `DstSt` stores one to an output slot, and `ImmLd` loads a
/// plaintext immediate from an input slot. The slot coordinates are
/// encoded in [`TSrcId`], [`TDstId`], and [`TImmId`] respectively.
///
/// **PBS.** Regular (`Pbs`, `Pbs2`, `Pbs4`, `Pbs8`) and flush
/// (`PbsF`, `Pbs2F`, `Pbs4F`, `Pbs8F`) variants. Flush variants
/// mark the last PBS in a batch group. The output arity matches the
/// numeric suffix (1, 2, 4, or 8 ciphertext registers).
///
/// **Batching.** `Batch` wraps a nested `IR<HpuLang>` sub-program
/// containing a group of PBS operations. `BatchArg` and `BatchRet`
/// appear inside the nested IR to define the batch boundary interface.
///
/// All signatures are available via the [`DialectInstructionSet`] impl.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum HpuInstructionSet {
    /// Addition of two ciphertext registers.
    /// `(CtRegister, CtRegister) → (CtRegister)`
    AddCt,
    /// Subtraction of two ciphertext registers.
    /// `(CtRegister, CtRegister) → (CtRegister)`
    SubCt,
    /// Multiply-accumulate: `src1 * cst + src2` (pack operation).
    /// `(CtRegister, CtRegister) → (CtRegister)`
    Mac {
        cst: Immediate,
    },
    /// Addition of a ciphertext register and a plaintext immediate
    /// register. `(CtRegister, PtImmediate) → (CtRegister)`
    AddPt,
    /// Subtraction: ciphertext minus plaintext immediate register.
    /// `(CtRegister, PtImmediate) → (CtRegister)`
    SubPt,
    /// Subtraction: plaintext immediate register minus ciphertext.
    /// `(PtImmediate, CtRegister) → (CtRegister)`
    PtSub,
    /// Multiplication of a ciphertext register by a plaintext immediate
    /// register. `(CtRegister, PtImmediate) → (CtRegister)`
    MulPt,
    /// Addition of a ciphertext register and an inline constant.
    /// `(CtRegister) → (CtRegister)`
    AddCst {
        cst: Immediate,
    },
    /// Subtraction: ciphertext minus inline constant.
    /// `(CtRegister) → (CtRegister)`
    SubCst {
        cst: Immediate,
    },
    /// Subtraction: inline constant minus ciphertext.
    /// `(CtRegister) → (CtRegister)`
    CstSub {
        cst: Immediate,
    },
    /// Multiplication of a ciphertext register by an inline constant.
    /// `(CtRegister) → (CtRegister)`
    MulCst {
        cst: Immediate,
    },
    /// Materializes an inline constant into a ciphertext register.
    /// `() → (CtRegister)`
    CstCt {
        cst: Immediate,
    },
    /// Loads a plaintext immediate from an input slot.
    /// `() → (PtImmediate)`
    ImmLd {
        from: TImmId,
    },
    /// Stores a ciphertext register to an output slot.
    /// `(CtRegister) → ()`
    DstSt {
        to: TDstId,
    },
    /// Loads a ciphertext block from an input slot into a register.
    /// `() → (CtRegister)`
    SrcLd {
        from: TSrcId,
    },
    /// Single-output PBS. `(CtRegister) → (CtRegister)`
    Pbs {
        lut: LutId,
    },
    /// 2-output many-LUT PBS.
    /// `(CtRegister) → (CtRegister, CtRegister)`
    Pbs2 {
        lut: LutId,
    },
    /// 4-output many-LUT PBS.
    /// `(CtRegister) → (CtRegister × 4)`
    Pbs4 {
        lut: LutId,
    },
    /// 8-output many-LUT PBS.
    /// `(CtRegister) → (CtRegister × 8)`
    Pbs8 {
        lut: LutId,
    },
    /// Single-output PBS with flush (batch boundary marker).
    /// `(CtRegister) → (CtRegister)`
    PbsF {
        lut: LutId,
    },
    /// 2-output many-LUT PBS with flush.
    /// `(CtRegister) → (CtRegister, CtRegister)`
    Pbs2F {
        lut: LutId,
    },
    /// 4-output many-LUT PBS with flush.
    /// `(CtRegister) → (CtRegister × 4)`
    Pbs4F {
        lut: LutId,
    },
    /// 8-output many-LUT PBS with flush.
    /// `(CtRegister) → (CtRegister × 8)`
    Pbs8F {
        lut: LutId,
    },
    /// Nested sub-program grouping a batch of PBS operations. The
    /// signature is derived from the `BatchArg` and `BatchRet` ops
    /// inside `block`.
    Batch {
        block: Box<IR<HpuLang>>,
    },
    /// Batch input at positional slot `pos`. Appears inside a
    /// [`Batch`](Self::Batch) block. `() → (ty)`
    BatchArg {
        pos: u8,
        ty: HpuTypeSystem,
    },
    /// Batch output at positional slot `pos`. Appears inside a
    /// [`Batch`](Self::Batch) block. `(ty) → ()`
    BatchRet { pos: u8, ty: HpuTypeSystem },

    TransferIn {tid: u8},

    TransferOut {tid: u8}
}

impl HpuInstructionSet {
    pub fn is_pbs(&self) -> bool {
        match self {
            HpuInstructionSet::Pbs { .. }
            | HpuInstructionSet::Pbs2 { .. }
            | HpuInstructionSet::Pbs4 { .. }
            | HpuInstructionSet::Pbs8 { .. }
            | HpuInstructionSet::PbsF { .. }
            | HpuInstructionSet::Pbs2F { .. }
            | HpuInstructionSet::Pbs4F { .. }
            | HpuInstructionSet::Pbs8F { .. } => true,
            _ => false,
        }
    }

    pub fn is_batch(&self) -> bool {
        matches!(self, HpuInstructionSet::Batch { .. })
    }
}

impl Format for HpuInstructionSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>, ctx: &FormatContext) -> std::fmt::Result {
        match self {
            HpuInstructionSet::AddCt => write!(f, "add_ct"),
            HpuInstructionSet::SubCt => write!(f, "sub_ct"),
            HpuInstructionSet::Mac { cst } => write!(f, "mac<{cst}>"),
            HpuInstructionSet::AddPt => write!(f, "add_pt"),
            HpuInstructionSet::SubPt => write!(f, "sub_pt"),
            HpuInstructionSet::PtSub => write!(f, "pt_sub"),
            HpuInstructionSet::MulPt => write!(f, "mul_pt"),
            HpuInstructionSet::AddCst { cst } => write!(f, "add_cst<{cst}>"),
            HpuInstructionSet::SubCst { cst } => write!(f, "subs_cst<{cst}>"),
            HpuInstructionSet::CstSub { cst } => write!(f, "cst_sub<{cst}>"),
            HpuInstructionSet::MulCst { cst } => write!(f, "mul_cst<{cst}>"),
            HpuInstructionSet::CstCt { cst } => write!(f, "cst_ct<{cst}>"),
            HpuInstructionSet::ImmLd { from } => write!(f, "imm_ld<{from}>"),
            HpuInstructionSet::SrcLd { from } => write!(f, "src_ld<{from}>"),
            HpuInstructionSet::DstSt { to } => write!(f, "dst_st<{to}>"),
            HpuInstructionSet::Pbs { lut } => write!(f, "pbs<{lut}>"),
            HpuInstructionSet::Pbs2 { lut } => write!(f, "pbs_2<{lut}>"),
            HpuInstructionSet::Pbs4 { lut } => write!(f, "pbs_4<{lut}>"),
            HpuInstructionSet::Pbs8 { lut } => write!(f, "pbs_8<{lut}>"),
            HpuInstructionSet::PbsF { lut } => write!(f, "pbs_f<{lut}>"),
            HpuInstructionSet::Pbs2F { lut } => write!(f, "pbs_2f<{lut}>"),
            HpuInstructionSet::Pbs4F { lut } => write!(f, "pbs_4f<{lut}>"),
            HpuInstructionSet::Pbs8F { lut } => write!(f, "pbs_8f<{lut}>"),
            HpuInstructionSet::Batch { block, .. } => {
                // Format nested IR with proper prefix propagation and unique nested prefix
                let inner_ctx = ctx.with_prefix("    ").with_next_nested_prefix();
                writeln!(f, "batch {{")?;
                Format::fmt(block.as_ref(), f, &inner_ctx)?;
                write!(f, "\n{}}}", ctx.prefix())
            }
            HpuInstructionSet::BatchArg { pos, ty } => write!(f, "batch_arg<{pos}, {ty}>"),
            HpuInstructionSet::BatchRet { pos, ty } => write!(f, "batch_ret<{pos}, {ty}>"),
            HpuInstructionSet::TransferIn { tid } => write!(f, "transfer_in<#{tid}>"),
            HpuInstructionSet::TransferOut { tid } => write!(f, "transfer_out<#{tid}>"),
        }
    }
}

impl Display for HpuInstructionSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Format::fmt(self, f, &FormatContext::default())
    }
}

impl DialectInstructionSet for HpuInstructionSet {
    type TypeSystem = HpuTypeSystem;

    fn get_signature(&self) -> Signature<Self::TypeSystem> {
        use HpuTypeSystem::*;
        match self {
            HpuInstructionSet::AddCt => sig![(CtRegister, CtRegister) -> (CtRegister)],
            HpuInstructionSet::SubCt => sig![(CtRegister, CtRegister) -> (CtRegister)],
            HpuInstructionSet::Mac { .. } => sig![(CtRegister, CtRegister) -> (CtRegister)],
            HpuInstructionSet::AddPt => sig![(CtRegister, PtImmediate) -> (CtRegister)],
            HpuInstructionSet::SubPt => sig![(CtRegister, PtImmediate) -> (CtRegister)],
            HpuInstructionSet::PtSub => sig![(PtImmediate, CtRegister) -> (CtRegister)],
            HpuInstructionSet::MulPt => sig![(CtRegister, PtImmediate) -> (CtRegister)],
            HpuInstructionSet::AddCst { .. } => sig![(CtRegister) -> (CtRegister)],
            HpuInstructionSet::SubCst { .. } => sig![(CtRegister) -> (CtRegister)],
            HpuInstructionSet::CstSub { .. } => sig![(CtRegister) -> (CtRegister)],
            HpuInstructionSet::MulCst { .. } => sig![(CtRegister) -> (CtRegister)],
            HpuInstructionSet::CstCt { .. } => sig![() -> (CtRegister)],
            HpuInstructionSet::DstSt { .. } => sig![(CtRegister) -> ()],
            HpuInstructionSet::SrcLd { .. } => sig![() -> (CtRegister)],
            HpuInstructionSet::ImmLd { .. } => sig![() -> (PtImmediate)],
            HpuInstructionSet::Pbs { .. } => sig![(CtRegister) -> (CtRegister)],
            HpuInstructionSet::Pbs2 { .. } => sig![(CtRegister) -> (CtRegister, CtRegister)],
            HpuInstructionSet::Pbs4 { .. } => {
                sig![(CtRegister) -> (CtRegister, CtRegister, CtRegister, CtRegister)]
            }
            HpuInstructionSet::Pbs8 { .. } => {
                sig![(CtRegister) -> (CtRegister, CtRegister, CtRegister, CtRegister, CtRegister, CtRegister, CtRegister, CtRegister)]
            }
            HpuInstructionSet::PbsF { .. } => sig![(CtRegister) -> (CtRegister)],
            HpuInstructionSet::Pbs2F { .. } => sig![(CtRegister) -> (CtRegister, CtRegister)],
            HpuInstructionSet::Pbs4F { .. } => {
                sig![(CtRegister) -> (CtRegister, CtRegister, CtRegister, CtRegister)]
            }
            HpuInstructionSet::Pbs8F { .. } => {
                sig![(CtRegister) -> (CtRegister, CtRegister, CtRegister, CtRegister, CtRegister, CtRegister, CtRegister, CtRegister)]
            }
            HpuInstructionSet::Batch { block } => {
                let mut inputs = block
                    .walk_ops_linear()
                    .filter_map(|op| match op.get_instruction() {
                        HpuInstructionSet::BatchArg { pos, ty } => Some((pos, ty)),
                        _ => None,
                    })
                    .cosvec();
                inputs.sort_unstable_by_key(|a| a.0);
                let inputs = inputs.into_iter().map(|a| a.1).cosvec();
                let mut outputs = block
                    .walk_ops_linear()
                    .filter_map(|op| match op.get_instruction() {
                        HpuInstructionSet::BatchRet { pos, ty } => Some((pos, ty)),
                        _ => None,
                    })
                    .cosvec();
                outputs.sort_unstable_by_key(|a| a.0);
                let outputs = outputs.into_iter().map(|a| a.1).cosvec();
                Signature(inputs, outputs)
            }
            HpuInstructionSet::BatchArg { ty, .. } => sig![() -> (ty.clone())],
            HpuInstructionSet::BatchRet { ty, .. } => sig![(ty.clone()) -> ()],
            HpuInstructionSet::TransferIn { .. } => sig![() -> (CtRegister)],
            HpuInstructionSet::TransferOut { .. } => sig![(CtRegister) -> ()],
        }
    }
}
