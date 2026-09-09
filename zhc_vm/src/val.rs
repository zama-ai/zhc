use tfhe::integer::RadixCiphertext;
use zhc::crypto::integer_semantics::EmulatedIntegerPlaintext;

/// A read-only reference to an input value for VM execution.
///
/// Each element in the `inputs` slice passed to [`Vm::execute`](crate::Vm::execute)
/// is a [`Value`] pointing to either an encrypted radix ciphertext or a plaintext
/// constant. The VM reads from these during `LD` and scalar-operand instructions
/// without modifying the underlying data.
#[derive(Clone, Copy)]
pub enum Value {
    /// A pointer to an encrypted radix ciphertext (read-only).
    FheUint(*const RadixCiphertext),
    /// A plaintext integer used for scalar operations.
    Uint(EmulatedIntegerPlaintext),
}

impl Value {
    /// Extracts the inner ciphertext pointer, panicking if the value is not `FheUint`.
    ///
    /// This is a convenience accessor for contexts where you know the value is an
    /// encrypted ciphertext. Prefer pattern matching when the variant is uncertain.
    ///
    /// # Panics
    ///
    /// Panics if `self` is [`Value::Uint`].
    pub fn unwrap_fhe_uint(self) -> *const RadixCiphertext {
        match self {
            Value::FheUint(a) => a,
            _ => panic!(),
        }
    }
}

/// A mutable reference to an output value for VM execution.
///
/// Each element in the `outputs` slice passed to [`Vm::execute`](crate::Vm::execute)
/// is a [`ValueMut`] pointing to a pre-allocated radix ciphertext that the VM writes
/// its results into during `ST` instructions.
#[derive(Clone, Copy)]
pub enum ValueMut {
    /// A pointer to a mutable encrypted radix ciphertext (write target).
    FheUint(*mut RadixCiphertext),
}

impl ValueMut {
    /// Extracts the inner mutable ciphertext pointer, panicking if the value is not
    /// `FheUint`.
    ///
    /// # Panics
    ///
    /// Panics if `self` is not [`ValueMut::FheUint`] (currently the only variant, so
    /// this cannot fail, but the signature is future-proofed).
    pub fn unwrap_fhe_uint(self) -> *mut RadixCiphertext {
        match self {
            ValueMut::FheUint(a) => a,
        }
    }
}
