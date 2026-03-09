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
    InputCiphertext {
        pos: usize,
        int_size: u16,
    },
    /// Plaintext program input at positional slot `pos`, with
    /// `int_size` radix blocks. `() → (Plaintext)`
    InputPlaintext {
        pos: usize,
        int_size: u16,
    },
    /// Ciphertext program output at positional slot `pos`.
    /// `(Ciphertext) → ()`
    OutputCiphertext {
        pos: usize,
    },
    /// Debug-only value sink. `(typ) → ()`
    _Consume {
        typ: IopTypeSystem,
    },
    /// Identity forwarding. `(typ) → (typ)`.
    /// Eliminated by [`eliminate_aliases`](super::eliminate_aliases)
    /// before downstream passes.
    Inspect {
        typ: IopTypeSystem,
    },
    /// Zero-initialized composite ciphertext. `() → (Ciphertext)`
    DeclareCiphertext {
        int_size: u16,
    },
    /// Plaintext block constant. `() → (PlaintextBlock)`
    LetPlaintextBlock {
        value: u8,
    },
    /// Ciphertext block constant. `() → (CiphertextBlock)`
    LetCiphertextBlock {
        value: u8,
    },
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
    PackCt {
        mul: u8,
    },
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
    ExtractCtBlock {
        index: u8,
    },
    /// Extracts the plaintext block at `index` from a composite
    /// plaintext (index 0 = LSB).
    /// `(Plaintext) → (PlaintextBlock)`
    ExtractPtBlock {
        index: u8,
    },
    /// Writes a ciphertext block into a composite ciphertext at `index`,
    /// returning the updated ciphertext.
    /// `(CiphertextBlock, Ciphertext) → (Ciphertext)`
    StoreCtBlock {
        index: u8,
    },
    /// Single-output PBS. Applies a [`Lut1Def`] lookup table with the
    /// given padding-check policy.
    /// `(CiphertextBlock) → (CiphertextBlock)`
    Pbs {
        check: LookupCheck,
        lut: Lut1Def,
    },
    /// 2-output many-LUT PBS. Padding is unconditionally checked.
    /// `(CiphertextBlock) → (CiphertextBlock, CiphertextBlock)`
    Pbs2 {
        lut: Lut2Def,
    },
    /// 4-output many-LUT PBS. Padding is unconditionally checked.
    /// `(CiphertextBlock) → (CiphertextBlock × 4)`
    Pbs4 {
        lut: Lut4Def,
    },
    /// 8-output many-LUT PBS. Padding is unconditionally checked.
    /// `(CiphertextBlock) → (CiphertextBlock × 8)`
    Pbs8 {
        lut: Lut8Def,
    },
    Transfer,
    TransferIn {
        uid: u8,
    },
    TransferOut {
        uid: u8,
    },
}

impl Format for IopInstructionSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>, _ctx: &FormatContext) -> std::fmt::Result {
        use IopInstructionSet::*;
        match self {
            InputCiphertext { pos, int_size } => {
                write!(f, "input_ciphertext<{pos}, {int_size}>")
            }
            InputPlaintext { pos, int_size } => {
                write!(f, "input_plaintext<{pos}, {int_size}>")
            }
            OutputCiphertext { pos } => write!(f, "output<{pos}>"),
            _Consume { typ } => write!(f, "_consume<{typ}>"),
            Inspect { .. } => write!(f, "inspect"),
            DeclareCiphertext { int_size } => write!(f, "decl_ct<{int_size}>"),
            LetPlaintextBlock { value } => write!(f, "let_pt_block<{value}>"),
            LetCiphertextBlock { value } => write!(f, "let_ct_block<{value}>"),
            PackCt { mul } => write!(f, "pack_ct<{mul}>"),
            AddCt => write!(f, "add_ct"),
            WrappingAddCt => write!(f, "wrapping_add_ct"),
            TemperAddCt => write!(f, "temper_add_ct"),
            SubCt => write!(f, "sub_ct"),
            AddPt => write!(f, "add_pt"),
            WrappingAddPt => write!(f, "wrapping_add_pt"),
            SubPt => write!(f, "sub_pt"),
            PtSub => write!(f, "pt_sub"),
            MulPt => write!(f, "mul_pt"),
            ExtractCtBlock { index } => write!(f, "extract_ct_block<{index}>"),
            ExtractPtBlock { index } => write!(f, "extract_pt_block<{index}>"),
            StoreCtBlock { index } => write!(f, "store_ct_block<{index}>"),
            Pbs { check, lut } => write!(f, "pbs<{check:?}, {lut:?}>"),
            Pbs2 { lut } => write!(f, "pbs2<{lut:?}>"),
            Pbs4 { lut } => write!(f, "pbs4<{lut:?}>"),
            Pbs8 { lut } => write!(f, "pbs8<{lut:?}>"),
            Transfer => write!(f, "transfer"),
            TransferIn { uid } => write!(f, "transfer_in<#{uid}>"),
            TransferOut { uid } => write!(f, "transfer_out<#{uid}>"),
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
        use IopInstructionSet::*;
        use IopTypeSystem::*;
        match self {
            InputCiphertext { .. } => sig![() -> (Ciphertext)],
            InputPlaintext { .. } => sig![() -> (Plaintext)],
            OutputCiphertext { .. } => sig![(Ciphertext) -> ()],
            _Consume { typ } => sig![(typ.clone()) -> ()],
            DeclareCiphertext { .. } => sig![() -> (Ciphertext)],
            LetPlaintextBlock { .. } => sig![() -> (PlaintextBlock)],
            LetCiphertextBlock { .. } => sig![() -> (CiphertextBlock)],
            AddCt => {
                sig![(CiphertextBlock, CiphertextBlock) -> (CiphertextBlock)]
            }
            WrappingAddCt => {
                sig![(CiphertextBlock, CiphertextBlock) -> (CiphertextBlock)]
            }
            TemperAddCt => {
                sig![(CiphertextBlock, CiphertextBlock) -> (CiphertextBlock)]
            }
            SubCt => {
                sig![(CiphertextBlock, CiphertextBlock) -> (CiphertextBlock)]
            }
            PackCt { .. } => {
                sig![(CiphertextBlock, CiphertextBlock) -> (CiphertextBlock)]
            }
            AddPt => {
                sig![(CiphertextBlock, PlaintextBlock) -> (CiphertextBlock)]
            }
            WrappingAddPt => {
                sig![(CiphertextBlock, PlaintextBlock) -> (CiphertextBlock)]
            }
            SubPt => {
                sig![(CiphertextBlock, PlaintextBlock) -> (CiphertextBlock)]
            }
            PtSub => {
                sig![(PlaintextBlock, CiphertextBlock) -> (CiphertextBlock)]
            }
            MulPt => {
                sig![(CiphertextBlock, PlaintextBlock) -> (CiphertextBlock)]
            }
            ExtractCtBlock { .. } => sig![(Ciphertext) -> (CiphertextBlock)],
            ExtractPtBlock { .. } => sig![(Plaintext) -> (PlaintextBlock)],
            StoreCtBlock { .. } => {
                sig![(CiphertextBlock, Ciphertext) -> (Ciphertext)]
            }
            Pbs { .. } => sig![(CiphertextBlock) -> (CiphertextBlock)],
            Pbs2 { .. } => {
                sig![(CiphertextBlock) -> (CiphertextBlock, CiphertextBlock)]
            }
            Pbs4 { .. } => {
                sig![(CiphertextBlock) -> (CiphertextBlock, CiphertextBlock, CiphertextBlock, CiphertextBlock)]
            }
            Pbs8 { .. } => {
                sig![(CiphertextBlock) -> (CiphertextBlock, CiphertextBlock, CiphertextBlock, CiphertextBlock, CiphertextBlock, CiphertextBlock, CiphertextBlock, CiphertextBlock)]
            }
            Inspect { typ } => sig![(typ.clone()) -> (typ.clone())],
            Transfer => sig![(CiphertextBlock) -> (CiphertextBlock)],
            TransferIn { .. } => sig![() -> (CiphertextBlock)],
            TransferOut { .. } => sig![(CiphertextBlock) -> ()],
        }
    }
}
