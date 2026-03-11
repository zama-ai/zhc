use std::{fmt::Debug, hash::Hash};
use zhc_crypto::integer_semantics::lut::LookupCheck;
use zhc_ir::{DialectInstructionSet, Format, FormatContext, Signature, sig};

use crate::ioplang::{
    IopTypeSystem,
    lut::{Lut1Def, Lut2Def, Lut4Def, Lut8Def},
};

/// Instruction set for the IOP dialect.
///
/// Instructions fall into five categories:
///
/// **I/O and aliasing.** `InputCiphertext`, `InputPlaintext`, and
/// `OutputCiphertext` mark program entry/exit points at a given
/// positional slot. `Alias` forwards a value unchanged and is eliminated
/// by [`eliminate_aliases`](super::eliminate_aliases) before downstream
/// processing.
///
/// **Constants and declarations.** `DeclareCiphertext` produces a
/// zero-initialized composite ciphertext. `LetPlaintextBlock` and
/// `LetCiphertextBlock` produce scalar block constants.
///
/// **Block arithmetic.** Ciphertext-ciphertext operations (`AddCt`,
/// `WrappingAddCt`, `TemperAddCt`, `SubCt`, `PackCt`) and mixed
/// ciphertext-plaintext operations (`AddPt`, `WrappingAddPt`, `SubPt`,
/// `PtSub`, `MulPt`) all operate on individual blocks. The three
/// addition flavors differ in overflow policy: `AddCt` asserts the
/// padding bit stays clear on both inputs and output (protected),
/// `TemperAddCt` allows the padding bit to absorb overflow but forbids
/// carry beyond it (tempered), and `WrappingAddCt` performs modular
/// arithmetic with no overflow check.
///
/// **Block extraction and storage.** `ExtractCtBlock` and
/// `ExtractPtBlock` decompose a composite value into a block at a given
/// index. `StoreCtBlock` writes a block into a composite ciphertext at
/// a given index, producing an updated ciphertext.
///
/// **Programmable bootstrapping (PBS).** `Pbs` applies a single-output
/// lookup table with a configurable padding-check policy. `Pbs2`,
/// `Pbs4`, and `Pbs8` apply multi-output (many-LUT) bootstrapping,
/// producing 2, 4, or 8 output blocks respectively from one input block.
///
/// All signatures are available via the [`DialectInstructionSet`] impl.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IopInstructionSet {
    /// Ciphertext program input at positional slot `pos`, with
    /// `int_size` radix blocks. `() → (Ciphertext)`
    InputCiphertext { pos: usize, int_size: u16 },
    /// Plaintext program input at positional slot `pos`, with
    /// `int_size` radix blocks. `() → (Plaintext)`
    InputPlaintext { pos: usize, int_size: u16 },
    /// Ciphertext program output at positional slot `pos`.
    /// `(Ciphertext) → ()`
    OutputCiphertext { pos: usize },
    /// Debug-only value sink. `(typ) → ()`
    _Consume { typ: IopTypeSystem },
    /// Identity forwarding. `(typ) → (typ)`.
    /// Eliminated by [`eliminate_aliases`](super::eliminate_aliases)
    /// before downstream passes.
    Inspect { typ: IopTypeSystem },
    /// Zero-initialized composite ciphertext. `() → (Ciphertext)`
    DeclareCiphertext { int_size: u16 },
    /// Plaintext block constant. `() → (PlaintextBlock)`
    LetPlaintextBlock { value: u8 },
    /// Ciphertext block constant. `() → (CiphertextBlock)`
    LetCiphertextBlock { value: u8 },
    /// Protected addition of two ciphertext blocks. Both inputs and the
    /// output must have their padding bit clear.
    /// `(CiphertextBlock, CiphertextBlock) → (CiphertextBlock)`
    AddCt,
    /// Wrapping (modular) addition of two ciphertext blocks. No overflow
    /// check; carry beyond the complete block width is discarded.
    /// `(CiphertextBlock, CiphertextBlock) → (CiphertextBlock)`
    WrappingAddCt,
    /// Tempered addition of two ciphertext blocks. The padding bit may
    /// absorb overflow, but carry beyond the padding bit is forbidden.
    /// `(CiphertextBlock, CiphertextBlock) → (CiphertextBlock)`
    TemperAddCt,
    /// Protected subtraction of two ciphertext blocks.
    /// `(CiphertextBlock, CiphertextBlock) → (CiphertextBlock)`
    SubCt,
    /// Packs two ciphertext blocks by shifting the first left by the
    /// message width and adding the second. `mul` equals
    /// `2^message_size`, guaranteed by construction.
    /// `(CiphertextBlock, CiphertextBlock) → (CiphertextBlock)`
    PackCt { mul: u8 },
    /// Protected addition of a ciphertext block and a plaintext block.
    /// `(CiphertextBlock, PlaintextBlock) → (CiphertextBlock)`
    AddPt,
    /// Wrapping addition of a ciphertext block and a plaintext block.
    /// `(CiphertextBlock, PlaintextBlock) → (CiphertextBlock)`
    WrappingAddPt,
    /// Protected subtraction: ciphertext minus plaintext.
    /// `(CiphertextBlock, PlaintextBlock) → (CiphertextBlock)`
    SubPt,
    /// Protected subtraction: plaintext minus ciphertext.
    /// `(PlaintextBlock, CiphertextBlock) → (CiphertextBlock)`
    PtSub,
    /// Protected multiplication of a ciphertext block by a plaintext
    /// block. `(CiphertextBlock, PlaintextBlock) → (CiphertextBlock)`
    MulPt,
    /// Extracts the ciphertext block at `index` from a composite
    /// ciphertext (index 0 = LSB).
    /// `(Ciphertext) → (CiphertextBlock)`
    ExtractCtBlock { index: u8 },
    /// Extracts the plaintext block at `index` from a composite
    /// plaintext (index 0 = LSB).
    /// `(Plaintext) → (PlaintextBlock)`
    ExtractPtBlock { index: u8 },
    /// Writes a ciphertext block into a composite ciphertext at `index`,
    /// returning the updated ciphertext.
    /// `(CiphertextBlock, Ciphertext) → (Ciphertext)`
    StoreCtBlock { index: u8 },
    /// Single-output PBS. Applies a [`Lut1Def`] lookup table with the
    /// given padding-check policy.
    /// `(CiphertextBlock) → (CiphertextBlock)`
    Pbs { check: LookupCheck, lut: Lut1Def },
    /// 2-output many-LUT PBS. Padding is unconditionally checked.
    /// `(CiphertextBlock) → (CiphertextBlock, CiphertextBlock)`
    Pbs2 { lut: Lut2Def },
    /// 4-output many-LUT PBS. Padding is unconditionally checked.
    /// `(CiphertextBlock) → (CiphertextBlock × 4)`
    Pbs4 { lut: Lut4Def },
    /// 8-output many-LUT PBS. Padding is unconditionally checked.
    /// `(CiphertextBlock) → (CiphertextBlock × 8)`
    Pbs8 { lut: Lut8Def },
}

impl Format for IopInstructionSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>, _ctx: &FormatContext) -> std::fmt::Result {
        match self {
            IopInstructionSet::InputCiphertext { pos, int_size } => {
                write!(f, "input_ciphertext<{pos}, {int_size}>")
            }
            IopInstructionSet::InputPlaintext { pos, int_size } => {
                write!(f, "input_plaintext<{pos}, {int_size}>")
            }
            IopInstructionSet::OutputCiphertext { pos } => write!(f, "output<{pos}>"),
            IopInstructionSet::_Consume { typ } => write!(f, "_consume<{typ}>"),
            IopInstructionSet::Inspect { .. } => write!(f, "inspect"),
            IopInstructionSet::DeclareCiphertext { int_size } => write!(f, "decl_ct<{int_size}>"),
            IopInstructionSet::LetPlaintextBlock { value } => write!(f, "let_pt_block<{value}>"),
            IopInstructionSet::LetCiphertextBlock { value } => write!(f, "let_ct_block<{value}>"),
            IopInstructionSet::PackCt { mul } => write!(f, "pack_ct<{mul}>"),
            IopInstructionSet::AddCt => write!(f, "add_ct"),
            IopInstructionSet::WrappingAddCt => write!(f, "wrapping_add_ct"),
            IopInstructionSet::TemperAddCt => write!(f, "temper_add_ct"),
            IopInstructionSet::SubCt => write!(f, "sub_ct"),
            IopInstructionSet::AddPt => write!(f, "add_pt"),
            IopInstructionSet::WrappingAddPt => write!(f, "wrapping_add_pt"),
            IopInstructionSet::SubPt => write!(f, "sub_pt"),
            IopInstructionSet::PtSub => write!(f, "pt_sub"),
            IopInstructionSet::MulPt => write!(f, "mul_pt"),
            IopInstructionSet::ExtractCtBlock { index } => write!(f, "extract_ct_block<{index}>"),
            IopInstructionSet::ExtractPtBlock { index } => write!(f, "extract_pt_block<{index}>"),
            IopInstructionSet::StoreCtBlock { index } => write!(f, "store_ct_block<{index}>"),
            IopInstructionSet::Pbs { check, lut } => write!(f, "pbs<{check:?}, {lut:?}>"),
            IopInstructionSet::Pbs2 { lut } => write!(f, "pbs2<{lut:?}>"),
            IopInstructionSet::Pbs4 { lut } => write!(f, "pbs4<{lut:?}>"),
            IopInstructionSet::Pbs8 { lut } => write!(f, "pbs8<{lut:?}>"),
        }
    }
}

impl std::fmt::Display for IopInstructionSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Format::fmt(self, f, &FormatContext::default())
    }
}

impl DialectInstructionSet for IopInstructionSet {
    type TypeSystem = IopTypeSystem;

    fn get_signature(&self) -> Signature<Self::TypeSystem> {
        use IopTypeSystem::*;
        match self {
            IopInstructionSet::InputCiphertext { .. } => sig![() -> (Ciphertext)],
            IopInstructionSet::InputPlaintext { .. } => sig![() -> (Plaintext)],
            IopInstructionSet::OutputCiphertext { .. } => sig![(Ciphertext) -> ()],
            IopInstructionSet::_Consume { typ } => sig![(typ.clone()) -> ()],
            IopInstructionSet::DeclareCiphertext { .. } => sig![() -> (Ciphertext)],
            IopInstructionSet::LetPlaintextBlock { .. } => sig![() -> (PlaintextBlock)],
            IopInstructionSet::LetCiphertextBlock { .. } => sig![() -> (CiphertextBlock)],
            IopInstructionSet::AddCt => {
                sig![(CiphertextBlock, CiphertextBlock) -> (CiphertextBlock)]
            }
            IopInstructionSet::WrappingAddCt => {
                sig![(CiphertextBlock, CiphertextBlock) -> (CiphertextBlock)]
            }
            IopInstructionSet::TemperAddCt => {
                sig![(CiphertextBlock, CiphertextBlock) -> (CiphertextBlock)]
            }
            IopInstructionSet::SubCt => {
                sig![(CiphertextBlock, CiphertextBlock) -> (CiphertextBlock)]
            }
            IopInstructionSet::PackCt { .. } => {
                sig![(CiphertextBlock, CiphertextBlock) -> (CiphertextBlock)]
            }
            IopInstructionSet::AddPt => {
                sig![(CiphertextBlock, PlaintextBlock) -> (CiphertextBlock)]
            }
            IopInstructionSet::WrappingAddPt => {
                sig![(CiphertextBlock, PlaintextBlock) -> (CiphertextBlock)]
            }
            IopInstructionSet::SubPt => {
                sig![(CiphertextBlock, PlaintextBlock) -> (CiphertextBlock)]
            }
            IopInstructionSet::PtSub => {
                sig![(PlaintextBlock, CiphertextBlock) -> (CiphertextBlock)]
            }
            IopInstructionSet::MulPt => {
                sig![(CiphertextBlock, PlaintextBlock) -> (CiphertextBlock)]
            }
            IopInstructionSet::ExtractCtBlock { .. } => sig![(Ciphertext) -> (CiphertextBlock)],
            IopInstructionSet::ExtractPtBlock { .. } => sig![(Plaintext) -> (PlaintextBlock)],
            IopInstructionSet::StoreCtBlock { .. } => {
                sig![(CiphertextBlock, Ciphertext) -> (Ciphertext)]
            }
            IopInstructionSet::Pbs { .. } => sig![(CiphertextBlock) -> (CiphertextBlock)],
            IopInstructionSet::Pbs2 { .. } => {
                sig![(CiphertextBlock) -> (CiphertextBlock, CiphertextBlock)]
            }
            IopInstructionSet::Pbs4 { .. } => {
                sig![(CiphertextBlock) -> (CiphertextBlock, CiphertextBlock, CiphertextBlock, CiphertextBlock)]
            }
            IopInstructionSet::Pbs8 { .. } => {
                sig![(CiphertextBlock) -> (CiphertextBlock, CiphertextBlock, CiphertextBlock, CiphertextBlock, CiphertextBlock, CiphertextBlock, CiphertextBlock, CiphertextBlock)]
            }
            IopInstructionSet::Inspect { typ } => sig![(typ.clone()) -> (typ.clone())],
        }
    }
}
