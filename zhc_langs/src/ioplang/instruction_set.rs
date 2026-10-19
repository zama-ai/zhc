use std::{fmt::Debug, hash::Hash};
use zhc_crypto::integer_semantics::{
    Flavor,
    lut::{LookupCheck, Lut1, Lut2, Lut4, Lut8},
};
use zhc_ir::{DialectInstructionSet, Format, FormatContext, Signature, sig};

use crate::ioplang::IopTypeSystem;

/// Instruction set for the IOP dialect.
///
/// Instructions fall into five categories:
///
/// **I/O and aliasing.** `InputCiphertextInteger`, `InputPlaintextInteger`, and
/// `OutputCiphertextInteger` mark program entry/exit points at a given
/// positional slot. `Inspect` forwards a value unchanged and is eliminated
/// by [`eliminate_aliases`](super::eliminate_aliases) before downstream
/// processing.
///
/// **Constants and declarations.** `DeclareCiphertextInteger` produces a
/// zero-initialized composite ciphertext. `LetPlaintextBlock` and
/// `LetCiphertextBlock` produce scalar block constants.
///
/// **Block arithmetic.** Ciphertext-ciphertext operations (`AddCt`,
/// `SubCt`, `ShlCt`, `PackCt`) and mixed ciphertext-plaintext
/// operations (`AddPt`, `SubPt`, `PtSub`, `MulPt`) all operate on
/// individual blocks. Every linear operation carries a
/// [`Flavor`] selecting its overflow policy: `Protect` asserts the padding
/// bit stays clear on both inputs and output, `Temper` allows the padding
/// bit to absorb overflow but forbids carry beyond it, and `Wrapping`
/// performs modular arithmetic with no overflow check. The semantics of
/// each flavor are those of the matching `protect_*`, `temper_*` and
/// `wrapping_*` methods of
/// [`EmulatedCiphertextBlock`](zhc_crypto::integer_semantics::EmulatedCiphertextBlock).
///
/// **Block extraction and storage.** `ExtractCiphertextIntegerBlock` and
/// `ExtractPlaintextIntegerBlock` decompose a composite value into a block at a given
/// index. `StoreCiphertextIntegerBlock` writes a block into a composite ciphertext at
/// a given index, producing an updated ciphertext. `StoreCiphertextBoolBlock` and
/// `ExtractCiphertextBoolBlock` do the same for a Boolean ciphertext, which holds a
/// single clean zero-or-one block.
///
/// **Programmable bootstrapping (PBS).** `Pbs` applies a single-output
/// lookup table. `Pbs2`, `Pbs4`, and `Pbs8` apply multi-output (many-LUT)
/// bootstrapping, producing 2, 4, or 8 output blocks respectively from one
/// input block. Every PBS carries a [`LookupCheck`] policy controlling the
/// padding-bit and index-bit assertions on its input and outputs.
///
/// All signatures are available via the [`DialectInstructionSet`] impl.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IopInstructionSet {
    /// Encrypted integer input at positional slot `pos`, with
    /// `int_size` message bits. `() → (CiphertextInteger)`
    InputCiphertextInteger { pos: usize, int_size: u16 },
    /// Boolean ciphertext program input at positional slot `pos`.
    InputCiphertextBool { pos: usize },
    /// Plain integer input at positional slot `pos`, with
    /// `int_size` message bits. `() → (PlaintextInteger)`
    InputPlaintextInteger { pos: usize, int_size: u16 },
    /// Encrypted integer output at positional slot `pos`.
    /// `(CiphertextInteger) → ()`
    OutputCiphertextInteger { pos: usize },
    /// Boolean ciphertext program output at positional slot `pos`.
    OutputCiphertextBool { pos: usize },
    /// Debug-only value sink. `(typ) → ()`
    _Consume { typ: IopTypeSystem },
    /// Identity forwarding. `(typ) → (typ)`.
    /// Eliminated by [`eliminate_aliases`](super::eliminate_aliases)
    /// before downstream passes.
    Inspect { typ: IopTypeSystem },
    /// Copy of a value from one partition into another. Evaluates as
    /// the identity. `(typ) → (typ)`
    _Copy { typ: IopTypeSystem },
    /// Zero-initialized composite ciphertext. `() → (Ciphertext)`
    DeclareCiphertextInteger { int_size: u16 },
    /// Plaintext block constant. `() → (PlaintextBlock)`
    LetPlaintextBlock { value: u8 },
    /// Ciphertext block constant. The value spans the complete block
    /// width (padding, carry and message bits). `() → (CiphertextBlock)`
    LetCiphertextBlock { value: u8 },
    /// Addition of two ciphertext blocks.
    /// `(CiphertextBlock, CiphertextBlock) → (CiphertextBlock)`
    AddCt { flavor: Flavor },
    /// Subtraction of two ciphertext blocks.
    /// `(CiphertextBlock, CiphertextBlock) → (CiphertextBlock)`
    SubCt { flavor: Flavor },
    /// Left shift of a ciphertext block by `amount` bits.
    /// `(CiphertextBlock) → (CiphertextBlock)`
    ShlCt { amount: u8, flavor: Flavor },
    /// Multiply-accumulate: `arg0 * mul + arg1`. With `mul` equal to
    /// `2^message_size` this packs two blocks into one.
    /// `(CiphertextBlock, CiphertextBlock) → (CiphertextBlock)`
    PackCt { mul: u8, flavor: Flavor },
    /// Addition of a ciphertext block and a plaintext block.
    /// `(CiphertextBlock, PlaintextBlock) → (CiphertextBlock)`
    AddPt { flavor: Flavor },
    /// Subtraction: ciphertext minus plaintext.
    /// `(CiphertextBlock, PlaintextBlock) → (CiphertextBlock)`
    SubPt { flavor: Flavor },
    /// Subtraction: plaintext minus ciphertext.
    /// `(PlaintextBlock, CiphertextBlock) → (CiphertextBlock)`
    PtSub { flavor: Flavor },
    /// Multiplication of a ciphertext block by a plaintext block.
    /// `(CiphertextBlock, PlaintextBlock) → (CiphertextBlock)`
    MulPt { flavor: Flavor },
    /// Extracts the ciphertext block at `index` from a composite
    /// ciphertext (index 0 = LSB).
    /// `(CiphertextInteger) → (CiphertextBlock)`
    ExtractCiphertextIntegerBlock { index: u8 },
    /// Extracts the plaintext block at `index` from a composite
    /// plaintext (index 0 = LSB).
    /// `(PlaintextInteger) → (PlaintextBlock)`
    ExtractPlaintextIntegerBlock { index: u8 },
    /// Writes a ciphertext block into a composite ciphertext at `index`,
    /// returning the updated ciphertext.
    /// `(CiphertextBlock, CiphertextInteger) → (CiphertextInteger)`
    StoreCiphertextIntegerBlock { index: u8 },
    /// Stores a clean zero-or-one block into a new Boolean ciphertext.
    /// `(CiphertextBlock) → (CiphertextBool)`
    StoreCiphertextBoolBlock,
    /// Extracts the sole block backing a Boolean ciphertext.
    /// `(CiphertextBool) → (CiphertextBlock)`
    ExtractCiphertextBoolBlock,
    /// Single-output PBS. Checked according to the given policy.
    /// `(CiphertextBlock) → (CiphertextBlock)`
    Pbs { check: LookupCheck, lut: Lut1 },
    /// 2-output many-LUT PBS. Checked according to the given policy.
    /// `(CiphertextBlock) → (CiphertextBlock, CiphertextBlock)`
    Pbs2 { check: LookupCheck, lut: Lut2 },
    /// 4-output many-LUT PBS. Checked according to the given policy.
    /// `(CiphertextBlock) → (CiphertextBlock × 4)`
    Pbs4 { check: LookupCheck, lut: Lut4 },
    /// 8-output many-LUT PBS. Checked according to the given policy.
    /// `(CiphertextBlock) → (CiphertextBlock × 8)`
    Pbs8 { check: LookupCheck, lut: Lut8 },
}

impl IopInstructionSet {
    /// Returns true if this instruction is a PBS operation.
    pub fn is_pbs(&self) -> bool {
        use IopInstructionSet::*;
        matches!(self, Pbs { .. } | Pbs2 { .. } | Pbs4 { .. } | Pbs8 { .. })
    }

    /// Returns the flavor of a linear block operation, if it has one.
    pub fn flavor(&self) -> Option<Flavor> {
        use IopInstructionSet::*;
        match self {
            AddCt { flavor }
            | SubCt { flavor }
            | ShlCt { flavor, .. }
            | PackCt { flavor, .. }
            | AddPt { flavor }
            | SubPt { flavor }
            | PtSub { flavor }
            | MulPt { flavor } => Some(*flavor),
            _ => None,
        }
    }
}

impl Format for IopInstructionSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>, _ctx: &FormatContext) -> std::fmt::Result {
        use IopInstructionSet::*;
        match self {
            InputCiphertextInteger { pos, int_size } => {
                write!(f, "input_ciphertext<{pos}, {int_size}>")
            }
            InputCiphertextBool { pos } => write!(f, "input_ciphertext_bool<{pos}>"),
            InputPlaintextInteger { pos, int_size } => {
                write!(f, "input_plaintext<{pos}, {int_size}>")
            }
            OutputCiphertextInteger { pos } => write!(f, "output<{pos}>"),
            OutputCiphertextBool { pos } => write!(f, "output_bool<{pos}>"),
            _Consume { typ } => write!(f, "_consume<{typ}>"),
            Inspect { .. } => write!(f, "inspect"),
            _Copy { .. } => write!(f, "_copy"),
            DeclareCiphertextInteger { int_size } => write!(f, "decl_ct<{int_size}>"),
            LetPlaintextBlock { value } => write!(f, "let_pt_block<{value}>"),
            LetCiphertextBlock { value } => write!(f, "let_ct_block<{value}>"),
            AddCt { flavor } => write!(f, "{}add_ct", flavor.prefix()),
            SubCt { flavor } => write!(f, "{}sub_ct", flavor.prefix()),
            ShlCt { amount, flavor } => write!(f, "{}shl_ct<{amount}>", flavor.prefix()),
            PackCt { mul, flavor } => write!(f, "{}pack_ct<{mul}>", flavor.prefix()),
            AddPt { flavor } => write!(f, "{}add_pt", flavor.prefix()),
            SubPt { flavor } => write!(f, "{}sub_pt", flavor.prefix()),
            PtSub { flavor } => write!(f, "{}pt_sub", flavor.prefix()),
            MulPt { flavor } => write!(f, "{}mul_pt", flavor.prefix()),
            ExtractCiphertextIntegerBlock { index } => write!(f, "extract_ct_block<{index}>"),
            ExtractPlaintextIntegerBlock { index } => write!(f, "extract_pt_block<{index}>"),
            StoreCiphertextIntegerBlock { index } => write!(f, "store_ct_block<{index}>"),
            StoreCiphertextBoolBlock => write!(f, "store_ctbool_block"),
            ExtractCiphertextBoolBlock => write!(f, "extract_ctbool_block"),
            Pbs { check, lut } => write!(f, "pbs<{check:?}, {lut:?}>"),
            Pbs2 { check, lut } => write!(f, "pbs2<{check:?}, {lut:?}>"),
            Pbs4 { check, lut } => write!(f, "pbs4<{check:?}, {lut:?}>"),
            Pbs8 { check, lut } => write!(f, "pbs8<{check:?}, {lut:?}>"),
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
            InputCiphertextInteger { .. } => sig![() -> (CiphertextInteger)],
            InputCiphertextBool { .. } => sig![() -> (CiphertextBool)],
            InputPlaintextInteger { .. } => sig![() -> (PlaintextInteger)],
            OutputCiphertextInteger { .. } => sig![(CiphertextInteger) -> ()],
            OutputCiphertextBool { .. } => sig![(CiphertextBool) -> ()],
            _Consume { typ } => sig![(typ.clone()) -> ()],
            Inspect { typ } => sig![(typ.clone()) -> (typ.clone())],
            _Copy { typ } => sig![(typ.clone()) -> (typ.clone())],
            DeclareCiphertextInteger { .. } => sig![() -> (CiphertextInteger)],
            LetPlaintextBlock { .. } => sig![() -> (PlaintextBlock)],
            LetCiphertextBlock { .. } => sig![() -> (CiphertextBlock)],
            AddCt { .. } | SubCt { .. } | PackCt { .. } => {
                sig![(CiphertextBlock, CiphertextBlock) -> (CiphertextBlock)]
            }
            ShlCt { .. } => sig![(CiphertextBlock) -> (CiphertextBlock)],
            AddPt { .. } | SubPt { .. } | MulPt { .. } => {
                sig![(CiphertextBlock, PlaintextBlock) -> (CiphertextBlock)]
            }
            PtSub { .. } => {
                sig![(PlaintextBlock, CiphertextBlock) -> (CiphertextBlock)]
            }
            ExtractCiphertextIntegerBlock { .. } => sig![(CiphertextInteger) -> (CiphertextBlock)],
            ExtractPlaintextIntegerBlock { .. } => sig![(PlaintextInteger) -> (PlaintextBlock)],
            StoreCiphertextIntegerBlock { .. } => {
                sig![(CiphertextBlock, CiphertextInteger) -> (CiphertextInteger)]
            }
            StoreCiphertextBoolBlock => sig![(CiphertextBlock) -> (CiphertextBool)],
            ExtractCiphertextBoolBlock => sig![(CiphertextBool) -> (CiphertextBlock)],
            Pbs { .. } => sig![(CiphertextBlock) -> (CiphertextBlock)],
            Pbs2 { .. } => {
                sig![(CiphertextBlock) -> (CiphertextBlock, CiphertextBlock)]
            }
            Pbs4 { .. } => {
                sig![(CiphertextBlock) -> (CiphertextBlock, CiphertextBlock, CiphertextBlock, CiphertextBlock)]
            }
            Pbs8 { .. } => {
                sig![(CiphertextBlock) -> (
                    CiphertextBlock, CiphertextBlock, CiphertextBlock, CiphertextBlock,
                    CiphertextBlock, CiphertextBlock, CiphertextBlock, CiphertextBlock
                )]
            }
        }
    }
}
