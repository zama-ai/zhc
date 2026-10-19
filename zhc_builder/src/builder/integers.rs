use std::fmt::Debug;
use zhc_crypto::integer_semantics::{
    EmulatedCiphertextBlockStorage, EmulatedCiphertextBool, EmulatedCiphertextIntegerStorage,
    EmulatedPlaintextBlock, EmulatedPlaintextBlockStorage, EmulatedPlaintextInteger,
    EmulatedPlaintextIntegerStorage, SpecBlock, SpecInteger,
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
/// [`Builder::ciphertext_integer_split`](`super::Builder::ciphertext_integer_split`) or by
/// block-level arithmetic methods, and can be recombined into a full [`CiphertextInteger`] via
/// [`Builder::ciphertext_integer_join`](`super::Builder::ciphertext_integer_join`).
///
/// This type cannot be constructed directly — it is always returned by
/// [`Builder`](`super::Builder`) methods. Use [`make_value`](Self::make_value) to create a test
/// [`IopValue`] for [`Interpreter::with_inputs`](`super::Interpreter::with_inputs`).
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct CiphertextBlock {
    pub(crate) valid: ValId,
    pub(super) spec: SpecBlock,
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
    ///
    /// The specification is the one of the [`Builder`](`super::Builder`) that created the
    /// block. Two blocks can only be combined when their specifications are equal.
    pub fn spec(&self) -> SpecBlock {
        self.spec
    }

    /// Creates a compatible value to be used in interpretation.
    ///
    /// The `val` argument is the complete block representation including both carry and
    /// message bits. It is interpreted according to this block's [`SpecBlock`].
    ///
    /// # Panics
    ///
    /// Panics if `val` overflows the complete bit width (padding + carry + message).
    pub fn make_value(&self, val: EmulatedCiphertextBlockStorage) -> IopValue {
        IopValue::CiphertextBlock(self.spec.from_complete(val))
    }
}

/// An opaque handle to an encrypted Boolean backed by one clean ciphertext block.
///
/// A ciphertext Boolean is a single [`CiphertextBlock`] whose message is exactly `0` (false) or
/// `1` (true), with clear carry and padding bits. It is the result type of operations that answer
/// a yes/no question, such as comparisons or overflow detection, and it can be a circuit input or
/// output on its own.
///
/// Booleans are produced by
/// [`Builder::ciphertext_bool_input`](`super::Builder::ciphertext_bool_input`) or by storing a
/// clean zero-or-one block with
/// [`Builder::ciphertext_bool_store_block`](`super::Builder::ciphertext_bool_store_block`). Use
/// [`Builder::ciphertext_bool_get_block`](`super::Builder::ciphertext_bool_get_block`) to get the
/// backing block back for block-level arithmetic.
///
/// This type cannot be constructed directly — it is always returned by
/// [`Builder`](`super::Builder`) methods. Use [`make_value`](Self::make_value) to create a test
/// [`IopValue`] for [`Interpreter::with_inputs`](`super::Interpreter::with_inputs`).
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct CiphertextBool {
    pub(crate) valid: ValId,
    pub(super) spec: SpecBlock,
}

impl Debug for CiphertextBool {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}_ctbool", self.valid)
    }
}

impl AsRef<CiphertextBool> for CiphertextBool {
    fn as_ref(&self) -> &CiphertextBool {
        self
    }
}

impl Dumpable for CiphertextBool {
    fn dump_to_string(&self) -> String {
        format!("{:?}", self)
    }
}

impl CiphertextBool {
    /// Returns the block specification of the block backing this Boolean.
    ///
    /// The specification is the one of the [`Builder`](`super::Builder`) that created the
    /// Boolean, since a Boolean always occupies exactly one block of the circuit.
    pub fn spec(&self) -> SpecBlock {
        self.spec
    }

    /// Creates a compatible value to be used in interpretation.
    ///
    /// The `value` argument is the clear Boolean to be encoded. It is stored as a clean block
    /// whose message is `1` for true and `0` for false, according to this Boolean's
    /// [`SpecBlock`].
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_builder::*;
    /// let builder = Builder::new(SpecBlock(2, 2));
    /// let flag = builder.ciphertext_bool_input();
    /// builder.ciphertext_bool_output(&flag);
    /// let outputs = builder
    ///     .interpret()
    ///     .with_inputs(&[flag.make_value(true)])
    ///     .get_outputs();
    /// ```
    pub fn make_value(&self, value: bool) -> IopValue {
        IopValue::CiphertextBool(EmulatedCiphertextBool::from_bool(value, self.spec))
    }
}

/// An opaque handle to a multi-block encrypted integer in the IR graph.
///
/// A [`CiphertextInteger`] represents an integer stored as a radix-`2^message_size` decomposition
/// across multiple [`CiphertextBlock`]s (one block per digit). Its [`SpecInteger`]
/// records the total integer bit-width (`int_size`) and the per-block layout; the number
/// of blocks is `int_size / message_size`.
///
/// Use [`Builder::ciphertext_integer_split`](`super::Builder::ciphertext_integer_split`) to
/// decompose it into individual radix digits for block-level operations, and
/// [`Builder::ciphertext_integer_join`](`super::Builder::ciphertext_integer_join`) to reassemble.
///
/// This type cannot be constructed directly — it is always returned by
/// [`Builder`](`super::Builder`) methods. Use [`make_value`](Self::make_value) to create a test
/// [`IopValue`] for [`Interpreter::with_inputs`](`super::Interpreter::with_inputs`).
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct CiphertextInteger {
    pub(crate) valid: ValId,
    pub(super) spec: SpecInteger,
}

impl Debug for CiphertextInteger {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}_ctint", self.valid)
    }
}

impl AsRef<CiphertextInteger> for CiphertextInteger {
    fn as_ref(&self) -> &CiphertextInteger {
        self
    }
}

impl Dumpable for CiphertextInteger {
    fn dump_to_string(&self) -> String {
        format!("{:?}", self)
    }
}

impl CiphertextInteger {
    /// Returns the specification describing the integer bit-width and per-block layout.
    ///
    /// The integer bit-width is the one given when the integer was declared. The block layout is
    /// the one of the [`Builder`](`super::Builder`) that created the integer.
    pub fn spec(&self) -> SpecInteger {
        self.spec
    }

    /// Creates a compatible value to be used in interpretation.
    ///
    /// The `val` argument is the integer to be encoded. It is decomposed into
    /// blocks according to this ciphertext's [`SpecInteger`].
    pub fn make_value(&self, val: EmulatedCiphertextIntegerStorage) -> IopValue {
        IopValue::CiphertextInteger(self.spec().from_int(val))
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
    pub(super) spec: SpecBlock,
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
    ///
    /// The specification is the one of the [`Builder`](`super::Builder`) that created the
    /// block. It must be equal to the specification of any ciphertext block it is combined with.
    pub fn spec(&self) -> SpecBlock {
        self.spec
    }

    /// Creates a compatible value to be used in interpretation.
    ///
    /// The `val` argument is a raw message-only value. It is interpreted according to this
    /// block's [`SpecBlock`].
    ///
    /// # Panics
    ///
    /// Panics if `val` overflows the message bit width.
    pub fn make_value(&self, val: EmulatedPlaintextBlockStorage) -> IopValue {
        IopValue::PlaintextBlock(EmulatedPlaintextBlock::from_message(self.spec, val))
    }
}

/// An opaque handle to a multi-block plaintext integer in the IR graph.
///
/// A [`PlaintextInteger`] represents an unencrypted integer stored as a radix-`2^message_size`
/// decomposition across multiple [`PlaintextBlock`]s. Its [`SpecInteger`] records the
/// total integer bit-width and the
/// per-block layout. Use
/// [`Builder::plaintext_integer_split`](`super::Builder::plaintext_integer_split`) to decompose it
/// into individual blocks.
///
/// This type cannot be constructed directly — it is always returned by
/// [`Builder`](`super::Builder`) methods. Use [`make_value`](Self::make_value) to create a test
/// [`IopValue`] for [`Interpreter::with_inputs`](`super::Interpreter::with_inputs`).
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct PlaintextInteger {
    pub(crate) valid: ValId,
    pub(super) spec: SpecInteger,
}

impl Debug for PlaintextInteger {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}_ptint", self.valid)
    }
}

impl AsRef<PlaintextInteger> for PlaintextInteger {
    fn as_ref(&self) -> &PlaintextInteger {
        self
    }
}

impl Dumpable for PlaintextInteger {
    fn dump_to_string(&self) -> String {
        format!("{:?}", self)
    }
}

impl PlaintextInteger {
    /// Returns the specification describing the integer bit-width and per-block layout.
    ///
    /// The integer bit-width is the one given when the integer was declared. The block layout is
    /// the one of the [`Builder`](`super::Builder`) that created the integer.
    pub fn spec(&self) -> SpecInteger {
        self.spec
    }

    /// Creates a compatible value to be used in interpretation.
    ///
    /// The `val` argument is the integer to be encoded. It is decomposed into
    /// blocks according to this plaintext's [`SpecInteger`].
    pub fn make_value(&self, val: EmulatedPlaintextIntegerStorage) -> IopValue {
        IopValue::PlaintextInteger(EmulatedPlaintextInteger::from_int(self.spec, val))
    }
}
