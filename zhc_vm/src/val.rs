use tfhe::integer::RadixCiphertext;
use zhc::crypto::integer_semantics::EmulatedPlaintext;

#[derive(Clone, Copy)]
pub enum Value {
    FheUint(*const RadixCiphertext),
    Uint(EmulatedPlaintext)
}

impl Value {
    pub fn unwrap_fhe_uint(self) -> *const RadixCiphertext {
        match self {
            Value::FheUint(a) => a,
            _ => panic!()
        }
    }
}

#[derive(Clone, Copy)]
pub enum ValueMut {
    FheUint(*mut RadixCiphertext),
}

impl ValueMut {
    pub fn unwrap_fhe_uint(self) -> *mut RadixCiphertext {
        match self {
            ValueMut::FheUint(a) => a,
        }
    }
}
