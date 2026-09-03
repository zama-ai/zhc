use std::fmt::{Debug, Display};

use super::type_system::VmTypeSystem;
use zhc_crypto::integer_semantics::lut::{Lut1, Lut2};
use zhc_ir::{DialectInstructionSet, Format, FormatContext, Signature, sig};

/// Instruction set for the VM dialect.
///
/// Instructions fall into four categories:
///
/// **Register arithmetic.** Two-operand ciphertext ops (`AddCt`,
/// `SubCt`, `Mac`) and mixed ciphertext-plaintext ops (`AddPt`,
/// `SubPt`, `PtSub`, `MulPt`) take register operands. Constant-scalar
/// variants (`AddCst`, `SubCst`, `CstSub`, `MulCst`) inline the
/// plaintext value in `cst`, dropping the plaintext operand
/// altogether. `CstCt` materializes a constant ciphertext register.
///
/// **Memory transfer.** `SrcLd` loads a ciphertext block from an input
/// slot, `DstSt` stores one to an output slot, and `ImmLd` loads a
/// plaintext block from an input slot. All three address their block
/// by a positional slot index and a block index within that slot,
/// carried inline rather than in a dedicated operand type.
///
/// **Keyswitch.** `Ks` reduces a ciphertext to the width a PBS
/// consumes. Unlike in [`HpuInstructionSet`](crate::hpulang::HpuInstructionSet),
/// it is a first-class instruction rather than an implicit part of the
/// PBS, so it is scheduled — and its result register allocated —
/// independently.
///
/// **PBS.** `Pbs` and `Pbs2` bootstrap through the lookup table named
/// by `lut`, producing respectively one and two ciphertext registers
/// from a single bootstrap. Both expect a keyswitched argument, which
/// the type system cannot express: a program whose PBS argument is not
/// produced by a `Ks` is ill-formed even though it type-checks.
///
/// All signatures are available via the [`DialectInstructionSet`] impl.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum VmInstructionSet {
    /// Addition of two ciphertext registers.
    /// `(CtRegister, CtRegister) → (CtRegister)`
    AddCt,
    /// Subtraction of two ciphertext registers.
    /// `(CtRegister, CtRegister) → (CtRegister)`
    SubCt,
    /// Multiply-accumulate: `arg0 * cst + arg1` (pack operation).
    /// `(CtRegister, CtRegister) → (CtRegister)`
    Mac { cst: u8 },
    /// Addition of a ciphertext register and a plaintext immediate.
    /// `(CtRegister, PtImmediate) → (CtRegister)`
    AddPt,
    /// Subtraction: ciphertext minus plaintext immediate.
    /// `(CtRegister, PtImmediate) → (CtRegister)`
    SubPt,
    /// Subtraction: plaintext immediate minus ciphertext.
    /// `(PtImmediate, CtRegister) → (CtRegister)`
    PtSub,
    /// Multiplication of a ciphertext register by a plaintext
    /// immediate. `(CtRegister, PtImmediate) → (CtRegister)`
    MulPt,
    /// Addition of a ciphertext register and an inline constant.
    /// `(CtRegister) → (CtRegister)`
    AddCst { cst: u8 },
    /// Subtraction: ciphertext minus inline constant.
    /// `(CtRegister) → (CtRegister)`
    SubCst { cst: u8 },
    /// Subtraction: inline constant minus ciphertext.
    /// `(CtRegister) → (CtRegister)`
    CstSub { cst: u8 },
    /// Multiplication of a ciphertext register by an inline constant.
    /// `(CtRegister) → (CtRegister)`
    MulCst { cst: u8 },
    /// Materializes an inline constant into a ciphertext register.
    /// `() → (CtRegister)`
    CstCt { cst: u8 },
    /// Loads block `from_block` of plaintext input slot `from_pos`.
    /// `() → (PtImmediate)`
    ImmLd { from_pos: u32, from_block: u32 },
    /// Stores a ciphertext register to block `to_block` of output slot
    /// `to_pos`. `(CtRegister) → ()`
    DstSt { to_pos: u32, to_block: u32 },
    /// Loads block `from_block` of ciphertext input slot `from_pos`.
    /// `() → (CtRegister)`
    SrcLd { from_pos: u32, from_block: u32 },
    /// Keyswitches a ciphertext register to the reduced PBS width.
    /// `(CtRegister) → (CtRegister)`
    Ks,
    /// Single-output programmable bootstrapping through table `lut`.
    /// `(CtRegister) → (CtRegister)`
    Pbs { lut: Lut1 },
    /// Two-output programmable bootstrapping through table `lut`.
    /// `(CtRegister) → (CtRegister, CtRegister)`
    Pbs2 { lut: Lut2 },
}

impl VmInstructionSet {
    /// Returns whether this instruction is a PBS (any output arity).
    pub fn is_pbs(&self) -> bool {
        match self {
            VmInstructionSet::Pbs { .. } | VmInstructionSet::Pbs2 { .. } => true,
            _ => false,
        }
    }
}

impl Format for VmInstructionSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>, _ctx: &FormatContext) -> std::fmt::Result {
        use VmInstructionSet::*;
        match self {
            AddCt => write!(f, "add_ct"),
            SubCt => write!(f, "sub_ct"),
            Mac { cst } => write!(f, "mac<{cst}>"),
            AddPt => write!(f, "add_pt"),
            SubPt => write!(f, "sub_pt"),
            PtSub => write!(f, "pt_sub"),
            MulPt => write!(f, "mul_pt"),
            AddCst { cst } => write!(f, "add_cst<{cst}>"),
            SubCst { cst } => write!(f, "subs_cst<{cst}>"),
            CstSub { cst } => write!(f, "cst_sub<{cst}>"),
            MulCst { cst } => write!(f, "mul_cst<{cst}>"),
            CstCt { cst } => write!(f, "cst_ct<{cst}>"),
            ImmLd {
                from_block,
                from_pos,
            } => write!(f, "imm_ld<{from_pos}, {from_block}>"),
            SrcLd {
                from_block,
                from_pos,
            } => write!(f, "src_ld<{from_pos}, {from_block}>"),
            DstSt { to_block, to_pos } => write!(f, "dst_st<{to_pos}, {to_block}>"),
            Ks => write!(f, "ks"),
            Pbs { lut } => write!(f, "pbs<{lut:?}>"),
            Pbs2 { lut } => write!(f, "pbs_2<{lut:?}>"),
        }
    }
}

impl Display for VmInstructionSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Format::fmt(self, f, &FormatContext::default())
    }
}

impl DialectInstructionSet for VmInstructionSet {
    type TypeSystem = VmTypeSystem;

    fn get_signature(&self) -> Signature<Self::TypeSystem> {
        use VmInstructionSet::*;
        use VmTypeSystem::*;
        match self {
            AddCt => sig![(CtRegister, CtRegister) -> (CtRegister)],
            SubCt => sig![(CtRegister, CtRegister) -> (CtRegister)],
            Mac { .. } => sig![(CtRegister, CtRegister) -> (CtRegister)],
            AddPt => sig![(CtRegister, PtImmediate) -> (CtRegister)],
            SubPt => sig![(CtRegister, PtImmediate) -> (CtRegister)],
            PtSub => sig![(PtImmediate, CtRegister) -> (CtRegister)],
            MulPt => sig![(CtRegister, PtImmediate) -> (CtRegister)],
            AddCst { .. } => sig![(CtRegister) -> (CtRegister)],
            SubCst { .. } => sig![(CtRegister) -> (CtRegister)],
            CstSub { .. } => sig![(CtRegister) -> (CtRegister)],
            MulCst { .. } => sig![(CtRegister) -> (CtRegister)],
            CstCt { .. } => sig![() -> (CtRegister)],
            DstSt { .. } => sig![(CtRegister) -> ()],
            SrcLd { .. } => sig![() -> (CtRegister)],
            ImmLd { .. } => sig![() -> (PtImmediate)],
            Pbs { .. } => sig![(CtRegister) -> (CtRegister)],
            Pbs2 { .. } => sig![(CtRegister) -> (CtRegister, CtRegister)],
            Ks => sig![(CtRegister) -> (CtRegister)],
        }
    }
}
