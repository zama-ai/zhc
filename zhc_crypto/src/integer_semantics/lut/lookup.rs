use crate::integer_semantics::lut::LookupCheck;

use super::super::EmulatedCiphertextBlock;

/// Emulates a PBS lookup with negacyclic wraparound semantics.
///
/// Applies `lut` to `inp`. When the input padding bit is set, the raw output is
/// two's-complement negated (masked to `complete_mask`) to reproduce the negacyclic
/// table folding of a real TFHE bootstrap.
///
/// Padding-bit assertions on input and output are controlled by `check`
/// (see [`LookupCheck`]).
///
/// # Panics
///
/// Panics if a checked padding bit is nonzero.
pub fn lookup(
    lut: impl Fn(EmulatedCiphertextBlock) -> EmulatedCiphertextBlock,
    inp: EmulatedCiphertextBlock,
    check: LookupCheck,
) -> EmulatedCiphertextBlock {
    if check.should_check_input_padding() {
        assert!(
            inp.raw_padding_bits() == 0,
            "Encountered active padding bit in input when executing lookup with check {check:?}."
        );
    }
    let mut output = lut(inp);
    output.storage &= inp.spec().complete_mask();
    if inp.raw_padding_bits() == 1 {
        output = output.neg();
    }
    if check.should_check_output_padding() {
        assert!(
            output.raw_padding_bits() == 0,
            "Encountered active padding bit in output when executing lookup with check {check:?}."
        );
    }
    output
}

/// Applies 2 LUT functions to the same block in a single many-LUT PBS.
///
/// Evaluates `lut1` and `lut2` on `inp` and returns both results. The input block must
/// have its padding bit clear **and** its topmost data bit unset — that bit is reserved
/// by the many-LUT encoding to index the two sub-tables packed into the polynomial.
///
/// Because the padding bit is always free, negacyclic negation never triggers: this
/// function asserts clean padding on both input and all outputs unconditionally.
///
/// # Panics
///
/// Panics if the input padding bit is set, if the topmost data bit is set (many-LUT
/// index overflow), or if any output padding bit is set.
///
/// # Examples
///
/// ```rust,no_run
/// # use zhc_crypto::integer_semantics::{CiphertextBlockSpec, lut::lookup2};
/// let spec = CiphertextBlockSpec(2, 4);
/// let block = spec.from_message(3); // must fit in lower half of data range
/// let (a, b) = lookup2(|x| x, |x| x, block);
/// ```
pub fn lookup2(
    lut1: impl Fn(EmulatedCiphertextBlock) -> EmulatedCiphertextBlock,
    lut2: impl Fn(EmulatedCiphertextBlock) -> EmulatedCiphertextBlock,
    inp: EmulatedCiphertextBlock,
) -> (EmulatedCiphertextBlock, EmulatedCiphertextBlock) {
    assert!(
        inp.raw_padding_bits() == 0,
        "Encountered active padding bit in input when executing lookup2."
    );
    assert!(
        inp.raw_data_bits() <= (inp.spec().data_mask() >> 1),
        "Encountered active many lut bit in input when executing lookup2."
    );
    let output1 = lut1(inp);
    let output2 = lut2(inp);
    assert!(
        output1.raw_padding_bits() == 0,
        "Encountered active padding bit in output when executing lookup2."
    );
    assert!(
        output2.raw_padding_bits() == 0,
        "Encountered active padding bit in output when executing lookup2."
    );
    (output1, output2)
}
