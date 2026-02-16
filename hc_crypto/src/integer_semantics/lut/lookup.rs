use super::super::EmulatedCiphertextBlock;

/// Applies a lookup table to a ciphertext block, requiring the padding bit to be zero.
///
/// This function emulates a protected PBS operation where the padding bit acts as a guard. When
/// the padding bit is zero, the lookup index is guaranteed to fall within the first half of the
/// negacyclic table, so the LUT is applied directly without any output transformation.
///
/// The `lut` parameter is a function that computes the lookup result from the input block. It
/// receives the full block (including carry bits) and returns a new block with the transformed
/// value.
///
/// # Panics
///
/// Panics if the input block has its padding bit set to 1.
///
/// # Examples
///
/// ```
/// use hc_crypto::integer_semantics::{CiphertextBlockSpec, lut::protect_lookup};
///
/// let spec = CiphertextBlockSpec(2, 4);
/// let block = spec.from_message(5);
/// let extract_msg = |b| b;
/// let result = protect_lookup(extract_msg, block);
/// ```
pub fn protect_lookup(
    lut: impl Fn(EmulatedCiphertextBlock) -> EmulatedCiphertextBlock,
    inp: EmulatedCiphertextBlock,
) -> EmulatedCiphertextBlock {
    assert!(
        inp.raw_padding_bits() == 0,
        "Tried to protect-lookup, but input has active padding bit."
    );
    lut(inp)
}

/// Applies a lookup table to a ciphertext block with negacyclic wraparound semantics.
///
/// This function emulates a PBS operation that exploits or tolerates the negacyclic structure of
/// TFHE lookup tables. When the padding bit is zero, the LUT is applied directly. When the
/// padding bit is set, the output is negated (two's complement) to emulate the negacyclic
/// behavior: accessing the second half of the table returns the negation of the corresponding
/// first-half entry.
///
/// Use this lookup mode when implementing operations that intentionally use the full table range
/// or when the padding bit may be set due to prior arithmetic overflow.
///
/// The `lut` parameter is a function that computes the lookup result from the input block. It
/// receives the full block (including carry and padding bits) and returns a new block with the
/// transformed value.
///
/// # Examples
///
/// ```
/// use hc_crypto::integer_semantics::{CiphertextBlockSpec, lut::wrapping_lookup};
///
/// let spec = CiphertextBlockSpec(2, 4);
/// // Block with padding bit set (0b1_11_0101 for a 2-carry, 4-message spec)
/// let block = spec.from_complete(0b1_11_0101);
/// let identity = |b| b;
/// let result = wrapping_lookup(identity, block); // output is negated
/// ```
pub fn wrapping_lookup(
    lut: impl Fn(EmulatedCiphertextBlock) -> EmulatedCiphertextBlock,
    inp: EmulatedCiphertextBlock,
) -> EmulatedCiphertextBlock {
    let mut output = lut(inp);
    if inp.raw_padding_bits() == 1 {
        let raw_out = (!output.raw_complete_bits() + 1) & inp.spec().complete_mask();
        output.storage = raw_out;
    }
    output
}

#[cfg(test)]
mod test {
    use crate::integer_semantics::{
        CiphertextBlockSpec, EmulatedCiphertextBlockStorage, lut::wrapping_lookup,
    };

    use super::protect_lookup;

    #[test]
    fn test_protect_lookup_valid_input() {
        let spec = CiphertextBlockSpec(2, 2);
        let identity = |a| a;
        for i in 0..=spec.data_mask() {
            let inp = spec.from_complete(i);
            let out = protect_lookup(identity, inp);
            assert_eq!(out.storage, inp.storage);
        }
    }

    #[test]
    #[should_panic(expected = "Tried to protect-lookup, but input has active padding bit.")]
    fn test_protect_lookup_panics_on_padding_bit() {
        let spec = CiphertextBlockSpec(2, 2);
        let identity = |a| a;
        let inp = spec.from_complete(spec.padding_mask());
        protect_lookup(identity, inp);
    }

    #[test]
    fn test_wrapping() {
        let spec = CiphertextBlockSpec(2, 2);
        let identity = |a| a;
        for i in 0..=spec.data_mask() {
            let inp = spec.from_complete(i);
            let out = wrapping_lookup(identity, inp);
            assert_eq!(out.storage, inp.storage);
        }
        for i in spec.padding_mask()..=spec.padding_mask() + spec.data_mask() {
            let inp = spec.from_complete(i);
            let out = wrapping_lookup(identity, inp);
            let exp = EmulatedCiphertextBlockStorage::MAX << 5 | inp.storage;
            let exp = exp.wrapping_neg();
            assert_eq!(out.storage, exp);
        }
    }
}
