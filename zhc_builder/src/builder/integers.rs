use std::fmt::Debug;
use zhc_crypto::integer_semantics::{
    CiphertextBlockSpec, EmulatedBoolCiphertext, EmulatedCiphertextBlockStorage,
    EmulatedIntegerCiphertextStorage, EmulatedIntegerPlaintextStorage,
    EmulatedPlaintextBlockStorage, IntegerCiphertextSpec, IntegerPlaintextSpec, PlaintextBlockSpec,
};
use zhc_ir::ValId;
use zhc_langs::ioplang::IopValue;
use zhc_utils::Dumpable;

/// An opaque handle to a single encrypted block (radix digit) in the IR graph.
///
/// A ciphertext block represents one digit in the radix-decomposition of
/// an encrypted integer. Its bit layout contains `message_size` message bits (the digit
/// value), `carry_size` carry bits (to absorb arithmetic overflow), and one padding bit.
/// See the [module-level documentation](super::super) for the full layout diagram.
///
/// Blocks are produced by
/// [`Builder::integer_ciphertext_split`](`super::Builder::integer_ciphertext_split`) or by block-level
/// arithmetic methods, and can be recombined into a full [`IntegerCiphertext`] via
/// [`Builder::integer_ciphertext_join`](`super::Builder::integer_ciphertext_join`).
///
/// This type cannot be constructed directly — it is always returned by
/// [`Builder`](`super::Builder`) methods. Use [`make_value`](Self::make_value) to create a test
/// [`IopValue`] for [`Interpreter::with_inputs`](`super::Interpreter::with_inputs`).
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct CiphertextBlock {
    pub(crate) valid: ValId,
    pub(super) spec: CiphertextBlockSpec,
}

impl Debug for CiphertextBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}_ctblock", self.valid)
    }
}

impl AsRef<CiphertextBlock> for CiphertextBlock {
    fn as_ref(&self) -> &CiphertextBlock {
        self
    }
}

impl Dumpable for CiphertextBlock {
    fn dump_to_string(&self) -> String {
        format!("{:?}", self)
    }
}

impl CiphertextBlock {
    /// Returns the block specification describing the message/carry bit layout.
    pub fn spec(&self) -> CiphertextBlockSpec {
        self.spec
    }

    /// Creates a compatible value to be used in interpretation.
    ///
    /// The `val` argument is the complete block representation including both carry and
    /// message bits. It is interpreted according to this block's [`CiphertextBlockSpec`].
    ///
    /// # Panics
    ///
    /// Panics if `val` overflows the complete bit width (padding + carry + message).
    pub fn make_value(&self, val: EmulatedCiphertextBlockStorage) -> IopValue {
        IopValue::CiphertextBlock(self.spec.from_complete(val))
    }
}

/// An opaque handle to an encrypted Boolean backed by one clean ciphertext block.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct BoolCiphertext {
    pub(crate) valid: ValId,
    pub(super) spec: CiphertextBlockSpec,
}

impl Debug for BoolCiphertext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}_bool_ct", self.valid)
    }
}

impl AsRef<BoolCiphertext> for BoolCiphertext {
    fn as_ref(&self) -> &BoolCiphertext {
        self
    }
}

impl Dumpable for BoolCiphertext {
    fn dump_to_string(&self) -> String {
        format!("{:?}", self)
    }
}

impl BoolCiphertext {
    /// Returns the layout of the block backing this Boolean.
    pub fn spec(&self) -> CiphertextBlockSpec {
        self.spec
    }

    /// Creates a compatible Boolean value for interpretation.
    pub fn make_value(&self, value: bool) -> IopValue {
        IopValue::BoolCiphertext(EmulatedBoolCiphertext::from_bool(value, self.spec))
    }
}

/// An opaque handle to a multi-block encrypted integer in the IR graph.
///
/// A [`IntegerCiphertext`] represents an integer stored as a radix-`2^message_size` decomposition
/// across multiple [`CiphertextBlock`]s (one block per digit). Its [`IntegerCiphertextSpec`]
/// records the total integer bit-width (`int_size`) and the per-block layout; the number
/// of blocks is `int_size / message_size`.
///
/// Use [`Builder::integer_ciphertext_split`](`super::Builder::integer_ciphertext_split`) to decompose it
/// into individual radix digits for block-level operations, and
/// [`Builder::integer_ciphertext_join`](`super::Builder::integer_ciphertext_join`) to reassemble.
///
/// This type cannot be constructed directly — it is always returned by
/// [`Builder`](`super::Builder`) methods. Use [`make_value`](Self::make_value) to create a test
/// [`IopValue`] for [`Interpreter::with_inputs`](`super::Interpreter::with_inputs`).
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct IntegerCiphertext {
    pub(crate) valid: ValId,
    pub(super) spec: IntegerCiphertextSpec,
}

impl Debug for IntegerCiphertext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}_ct", self.valid)
    }
}

impl AsRef<IntegerCiphertext> for IntegerCiphertext {
    fn as_ref(&self) -> &IntegerCiphertext {
        self
    }
}

impl Dumpable for IntegerCiphertext {
    fn dump_to_string(&self) -> String {
        format!("{:?}", self)
    }
}

impl IntegerCiphertext {
    /// Returns the specification describing the integer bit-width and per-block layout.
    pub fn spec(&self) -> IntegerCiphertextSpec {
        self.spec
    }

    /// Creates a compatible value to be used in interpretation.
    ///
    /// The `val` argument is the integer to be encoded. It is decomposed into
    /// blocks according to this ciphertext's [`IntegerCiphertextSpec`].
    pub fn make_value(&self, val: EmulatedIntegerCiphertextStorage) -> IopValue {
        IopValue::IntegerCiphertext(self.spec().from_int(val))
    }
}

/// An opaque handle to a single plaintext block (radix digit) in the IR graph.
///
/// A plaintext block is the cleartext counterpart of a [`CiphertextBlock`]: it represents
/// one digit in the same radix-`2^message_size` decomposition, but carries only the
/// `message_size` message bits — no carry or padding. Plaintext blocks are used as the
/// right-hand operand in mixed ciphertext–plaintext arithmetic
/// (e.g. [`Builder::block_add_plaintext`](`super::Builder::block_add_plaintext`)).
///
/// This type cannot be constructed directly — it is always returned by
/// [`Builder`](`super::Builder`) methods. Use [`make_value`](Self::make_value) to create a test
/// [`IopValue`] for [`Interpreter::with_inputs`](`super::Interpreter::with_inputs`).
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct PlaintextBlock {
    pub(crate) valid: ValId,
    pub(super) spec: PlaintextBlockSpec,
}

impl Debug for PlaintextBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}_ptblock", self.valid)
    }
}

impl AsRef<PlaintextBlock> for PlaintextBlock {
    fn as_ref(&self) -> &PlaintextBlock {
        self
    }
}

impl Dumpable for PlaintextBlock {
    fn dump_to_string(&self) -> String {
        format!("{:?}", self)
    }
}

impl PlaintextBlock {
    /// Returns the block specification describing the message bit layout.
    pub fn spec(&self) -> PlaintextBlockSpec {
        self.spec
    }

    /// Creates a compatible value to be used in interpretation.
    ///
    /// The `val` argument is a raw message-only value. It is interpreted according to this
    /// block's [`PlaintextBlockSpec`].
    ///
    /// # Panics
    ///
    /// Panics if `val` overflows the message bit width.
    pub fn make_value(&self, val: EmulatedPlaintextBlockStorage) -> IopValue {
        IopValue::PlaintextBlock(self.spec.from_message(val))
    }
}

/// An opaque handle to a multi-block plaintext integer in the IR graph.
///
/// A [`IntegerPlaintext`] represents an unencrypted integer stored as a radix-`2^message_size`
/// decomposition across multiple [`PlaintextBlock`]s. Its [`IntegerPlaintextSpec`] records the
/// total integer bit-width and the
/// per-block layout. Use [`Builder::integer_plaintext_split`](`super::Builder::integer_plaintext_split`) to
/// decompose it into individual blocks.
///
/// This type cannot be constructed directly — it is always returned by
/// [`Builder`](`super::Builder`) methods. Use [`make_value`](Self::make_value) to create a test
/// [`IopValue`] for [`Interpreter::with_inputs`](`super::Interpreter::with_inputs`).
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct IntegerPlaintext {
    pub(crate) valid: ValId,
    pub(super) spec: IntegerPlaintextSpec,
}

impl Debug for IntegerPlaintext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}_pt", self.valid)
    }
}

impl AsRef<IntegerPlaintext> for IntegerPlaintext {
    fn as_ref(&self) -> &IntegerPlaintext {
        self
    }
}

impl Dumpable for IntegerPlaintext {
    fn dump_to_string(&self) -> String {
        format!("{:?}", self)
    }
}

impl IntegerPlaintext {
    /// Returns the specification describing the integer bit-width and per-block layout.
    pub fn spec(&self) -> IntegerPlaintextSpec {
        self.spec
    }

    /// Creates a compatible value to be used in interpretation.
    ///
    /// The `val` argument is the integer to be encoded. It is decomposed into
    /// blocks according to this plaintext's [`IntegerPlaintextSpec`].
    pub fn make_value(&self, val: EmulatedIntegerPlaintextStorage) -> IopValue {
        IopValue::IntegerPlaintext(self.spec.from_int(val))
    }
}
