use hc_crypto::integer_semantics::{
    CiphertextBlockSpec, CiphertextSpec, EmulatedCiphertextBlockStorage, EmulatedCiphertextStorage,
    EmulatedPlaintextBlockStorage, EmulatedPlaintextStorage, PlaintextBlockSpec, PlaintextSpec,
};
use hc_ir::ValId;
use hc_langs::ioplang::IopValue;
use std::fmt::Debug;

/// An opaque handle to a single encrypted block (radix digit) in the IR graph.
///
/// A ciphertext block represents one digit in the radix-decomposition of
/// an encrypted integer. Its bit layout contains `message_size` message bits (the digit
/// value), `carry_size` carry bits (to absorb arithmetic overflow), and one padding bit.
/// See the [module-level documentation](super) for the full layout diagram.
///
/// Blocks are produced by
/// [`Builder::split_ciphertext`](`super::Builder::split_ciphertext`) or by block-level
/// arithmetic methods, and can be recombined into a full [`Ciphertext`] via
/// [`Builder::join_ciphertext`](`super::Builder::join_ciphertext`).
///
/// This type cannot be constructed directly — it is always returned by
/// [`Builder`](`super::Builder`) methods. Use [`make_value`](Self::make_value) to create a test
/// [`IopValue`] for [`Builder::eval`](`super::Builder::eval`).
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct CiphertextBlock {
    pub(super) valid: ValId,
    pub(super) spec: CiphertextBlockSpec,
}

impl Debug for CiphertextBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}_ct_block", self.valid)
    }
}

impl AsRef<CiphertextBlock> for CiphertextBlock {
    fn as_ref(&self) -> &CiphertextBlock {
        self
    }
}

impl CiphertextBlock {
    /// Returns the block specification describing the message/carry bit layout.
    pub fn spec(&self) -> CiphertextBlockSpec {
        self.spec
    }

    /// Creates a compatible value to be used in evaluation.
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

/// An opaque handle to a multi-block encrypted integer in the IR graph.
///
/// A [`Ciphertext`] represents an integer stored as a radix-`2^message_size` decomposition
/// across multiple [`CiphertextBlock`]s (one block per digit). Its [`CiphertextSpec`]
/// records the total integer bit-width (`int_size`) and the per-block layout; the number
/// of blocks is `int_size / message_size`.
///
/// Use [`Builder::split_ciphertext`](`super::Builder::split_ciphertext`) to decompose it
/// into individual radix digits for block-level operations, and
/// [`Builder::join_ciphertext`](`super::Builder::join_ciphertext`) to reassemble.
///
/// This type cannot be constructed directly — it is always returned by
/// [`Builder`](`super::Builder`) methods. Use [`make_value`](Self::make_value) to create a test
/// [`IopValue`] for [`Builder::eval`](`super::Builder::eval`).
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Ciphertext {
    pub(super) valid: ValId,
    pub(super) spec: CiphertextSpec,
}

impl Debug for Ciphertext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}_ct", self.valid)
    }
}

impl AsRef<Ciphertext> for Ciphertext {
    fn as_ref(&self) -> &Ciphertext {
        self
    }
}

impl Ciphertext {
    /// Returns the specification describing the integer bit-width and per-block layout.
    pub fn spec(&self) -> CiphertextSpec {
        self.spec
    }

    /// Creates a compatible value to be used in evaluation.
    ///
    /// The `val` argument is the integer to be encoded. It is decomposed into
    /// blocks according to this ciphertext's [`CiphertextSpec`].
    pub fn make_value(&self, val: EmulatedCiphertextStorage) -> IopValue {
        IopValue::Ciphertext(self.spec().from_int(val))
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
/// [`IopValue`] for [`Builder::eval`](`super::Builder::eval`).
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct PlaintextBlock {
    pub(super) valid: ValId,
    pub(super) spec: PlaintextBlockSpec,
}

impl Debug for PlaintextBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}_pt_block", self.valid)
    }
}

impl AsRef<PlaintextBlock> for PlaintextBlock {
    fn as_ref(&self) -> &PlaintextBlock {
        self
    }
}

impl PlaintextBlock {
    /// Returns the block specification describing the message bit layout.
    pub fn spec(&self) -> PlaintextBlockSpec {
        self.spec
    }

    /// Creates a compatible value to be used in evaluation.
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
/// A [`Plaintext`] represents an unencrypted integer stored as a radix-`2^message_size`
/// decomposition across multiple [`PlaintextBlock`]s. Its [`PlaintextSpec`] records the
/// total integer bit-width and the
/// per-block layout. Use [`Builder::split_plaintext`](`super::Builder::split_plaintext`) to
/// decompose it into individual blocks.
///
/// This type cannot be constructed directly — it is always returned by
/// [`Builder`](`super::Builder`) methods. Use [`make_value`](Self::make_value) to create a test
/// [`IopValue`] for [`Builder::eval`](`super::Builder::eval`).
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Plaintext {
    pub(super) valid: ValId,
    pub(super) spec: PlaintextSpec,
}

impl Debug for Plaintext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}_pt", self.valid)
    }
}

impl AsRef<Plaintext> for Plaintext {
    fn as_ref(&self) -> &Plaintext {
        self
    }
}

impl Plaintext {
    /// Returns the specification describing the integer bit-width and per-block layout.
    pub fn spec(&self) -> PlaintextSpec {
        self.spec
    }

    /// Creates a compatible value to be used in evaluation.
    ///
    /// The `val` argument is the integer to be encoded. It is decomposed into
    /// blocks according to this plaintext's [`PlaintextSpec`].
    pub fn make_value(&self, val: EmulatedPlaintextStorage) -> IopValue {
        IopValue::Plaintext(self.spec.from_int(val))
    }
}
