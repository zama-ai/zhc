use super::super::EmulatedCiphertextBlock;

/// Padding-bit assertion policy for [`lookup`].
///
/// In TFHE, the padding bit guards against negacyclic wraparound during a PBS. Depending
/// on the operation being emulated, you may need to relax that guard on the input side,
/// the output side, or both.
///
/// Each variant selectively relaxes the input and/or output padding-bit check.
/// [`Protect`](Self::Protect) is the strictest mode (both ends checked);
/// [`AllowBothPadding`](Self::AllowBothPadding) disables all assertions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LookupCheck {
    /// Assert both input and output padding bits are zero.
    Protect,
    /// Skip the input check; still assert the output padding bit is zero.
    AllowInputPadding,
    /// Skip the output check; still assert the input padding bit is zero.
    AllowOutputPadding,
    /// Skip both checks.
    AllowBothPadding,
}

impl LookupCheck {
    /// Returns `true` when the input padding bit must be zero.
    pub fn should_check_input_padding(&self) -> bool {
        matches!(self, LookupCheck::Protect | LookupCheck::AllowOutputPadding)
    }

    /// Returns `true` when the output padding bit must be zero.
    pub fn should_check_output_padding(&self) -> bool {
        matches!(self, LookupCheck::Protect | LookupCheck::AllowInputPadding)
    }
}

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
        let a = !output.raw_complete_bits() & inp.spec().complete_mask();
        let raw_out = (a + 1) & inp.spec().complete_mask();
        output.storage = raw_out;
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

/// Applies 4 LUT functions to the same block in a single many-LUT PBS.
///
/// Evaluates `lut1` through `lut4` on `inp` and returns all four results. The input
/// block must have its padding bit clear **and** its 2 topmost data bits unset — those
/// bits are reserved by the many-LUT encoding to index the four sub-tables packed into
/// the polynomial.
///
/// Because the padding bit is always free, negacyclic negation never triggers: this
/// function asserts clean padding on both input and all outputs unconditionally.
///
/// # Panics
///
/// Panics if the input padding bit is set, if any of the 2 topmost data bits are set
/// (many-LUT index overflow), or if any output padding bit is set.
///
/// # Examples
///
/// ```rust,no_run
/// # use zhc_crypto::integer_semantics::{CiphertextBlockSpec, lut::lookup4};
/// let spec = CiphertextBlockSpec(2, 4);
/// let block = spec.from_message(1); // must fit in lower quarter of data range
/// let (a, b, c, d) = lookup4(|x| x, |x| x, |x| x, |x| x, block);
/// ```
pub fn lookup4(
    lut1: impl Fn(EmulatedCiphertextBlock) -> EmulatedCiphertextBlock,
    lut2: impl Fn(EmulatedCiphertextBlock) -> EmulatedCiphertextBlock,
    lut3: impl Fn(EmulatedCiphertextBlock) -> EmulatedCiphertextBlock,
    lut4: impl Fn(EmulatedCiphertextBlock) -> EmulatedCiphertextBlock,
    inp: EmulatedCiphertextBlock,
) -> (
    EmulatedCiphertextBlock,
    EmulatedCiphertextBlock,
    EmulatedCiphertextBlock,
    EmulatedCiphertextBlock,
) {
    assert!(
        inp.raw_padding_bits() == 0,
        "Encountered active padding bit in input when executing lookup4."
    );
    assert!(
        inp.raw_data_bits() <= (inp.spec().data_mask() >> 2),
        "Encountered active many lut bit in input when executing lookup4."
    );
    let output1 = lut1(inp);
    let output2 = lut2(inp);
    let output3 = lut3(inp);
    let output4 = lut4(inp);
    assert!(
        output1.raw_padding_bits() == 0,
        "Encountered active padding bit in output when executing lookup4."
    );
    assert!(
        output2.raw_padding_bits() == 0,
        "Encountered active padding bit in output when executing lookup4."
    );
    assert!(
        output3.raw_padding_bits() == 0,
        "Encountered active padding bit in output when executing lookup4."
    );
    assert!(
        output4.raw_padding_bits() == 0,
        "Encountered active padding bit in output when executing lookup4."
    );
    (output1, output2, output3, output4)
}

/// Applies 8 LUT functions to the same block in a single many-LUT PBS.
///
/// Evaluates `lut1` through `lut8` on `inp` and returns all eight results. The input
/// block must have its padding bit clear **and** its 3 topmost data bits unset — those
/// bits are reserved by the many-LUT encoding to index the eight sub-tables packed into
/// the polynomial.
///
/// Because the padding bit is always free, negacyclic negation never triggers: this
/// function asserts clean padding on both input and all outputs unconditionally.
///
/// # Panics
///
/// Panics if the input padding bit is set, if any of the 3 topmost data bits are set
/// (many-LUT index overflow), or if any output padding bit is set.
///
/// # Examples
///
/// ```rust,no_run
/// # use zhc_crypto::integer_semantics::{CiphertextBlockSpec, lut::lookup8};
/// let spec = CiphertextBlockSpec(2, 4);
/// let block = spec.from_message(0); // must fit in lower eighth of data range
/// let (a, b, c, d, e, f, g, h) = lookup8(
///     |x| x, |x| x, |x| x, |x| x,
///     |x| x, |x| x, |x| x, |x| x,
///     block,
/// );
/// ```
pub fn lookup8(
    lut1: impl Fn(EmulatedCiphertextBlock) -> EmulatedCiphertextBlock,
    lut2: impl Fn(EmulatedCiphertextBlock) -> EmulatedCiphertextBlock,
    lut3: impl Fn(EmulatedCiphertextBlock) -> EmulatedCiphertextBlock,
    lut4: impl Fn(EmulatedCiphertextBlock) -> EmulatedCiphertextBlock,
    lut5: impl Fn(EmulatedCiphertextBlock) -> EmulatedCiphertextBlock,
    lut6: impl Fn(EmulatedCiphertextBlock) -> EmulatedCiphertextBlock,
    lut7: impl Fn(EmulatedCiphertextBlock) -> EmulatedCiphertextBlock,
    lut8: impl Fn(EmulatedCiphertextBlock) -> EmulatedCiphertextBlock,
    inp: EmulatedCiphertextBlock,
) -> (
    EmulatedCiphertextBlock,
    EmulatedCiphertextBlock,
    EmulatedCiphertextBlock,
    EmulatedCiphertextBlock,
    EmulatedCiphertextBlock,
    EmulatedCiphertextBlock,
    EmulatedCiphertextBlock,
    EmulatedCiphertextBlock,
) {
    assert!(
        inp.raw_padding_bits() == 0,
        "Encountered active padding bit in input when executing lookup8."
    );
    assert!(
        inp.raw_data_bits() <= (inp.spec().data_mask() >> 3),
        "Encountered active many lut bit in input when executing lookup8."
    );
    let output1 = lut1(inp);
    let output2 = lut2(inp);
    let output3 = lut3(inp);
    let output4 = lut4(inp);
    let output5 = lut5(inp);
    let output6 = lut6(inp);
    let output7 = lut7(inp);
    let output8 = lut8(inp);
    assert!(
        output1.raw_padding_bits() == 0,
        "Encountered active padding bit in output when executing lookup8."
    );
    assert!(
        output2.raw_padding_bits() == 0,
        "Encountered active padding bit in output when executing lookup8."
    );
    assert!(
        output3.raw_padding_bits() == 0,
        "Encountered active padding bit in output when executing lookup8."
    );
    assert!(
        output4.raw_padding_bits() == 0,
        "Encountered active padding bit in output when executing lookup8."
    );
    assert!(
        output5.raw_padding_bits() == 0,
        "Encountered active padding bit in output when executing lookup8."
    );
    assert!(
        output6.raw_padding_bits() == 0,
        "Encountered active padding bit in output when executing lookup8."
    );
    assert!(
        output7.raw_padding_bits() == 0,
        "Encountered active padding bit in output when executing lookup8."
    );
    assert!(
        output8.raw_padding_bits() == 0,
        "Encountered active padding bit in output when executing lookup8."
    );
    (
        output1, output2, output3, output4, output5, output6, output7, output8,
    )
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::integer_semantics::CiphertextBlockSpec;

    #[test]
    fn test_lookup_identity_with_clean_padding() {
        let spec = CiphertextBlockSpec(2, 4);
        let inp = spec.from_message(5);
        let result = lookup(|x| x, inp, LookupCheck::Protect);
        assert_eq!(result, inp);
    }

    #[test]
    fn test_lookup_applies_lut_function() {
        let spec = CiphertextBlockSpec(2, 4);
        let inp = spec.from_message(3);
        let result = lookup(|x| x.spec().from_message(7), inp, LookupCheck::Protect);
        assert_eq!(result, spec.from_message(7));
    }

    #[test]
    #[should_panic(
        expected = "Encountered active padding bit in input when executing lookup with check Protect."
    )]
    fn test_lookup_protect_panics_on_input_padding_set() {
        let spec = CiphertextBlockSpec(2, 4);
        let inp = spec.from_complete(1 << spec.data_size()); // padding bit set
        lookup(|x| x, inp, LookupCheck::Protect);
    }

    #[test]
    #[should_panic(
        expected = "Encountered active padding bit in output when executing lookup with check Protect."
    )]
    fn test_lookup_protect_panics_on_output_padding_set() {
        let spec = CiphertextBlockSpec(2, 4);
        let inp = spec.from_message(0);
        lookup(
            |x| x.spec().from_complete(1 << spec.data_size()),
            inp,
            LookupCheck::Protect,
        );
    }

    #[test]
    fn test_lookup_allow_input_padding_does_not_panic_on_input_padding() {
        let spec = CiphertextBlockSpec(2, 4);
        let inp = spec.from_complete(1 << spec.data_size()); // padding bit set
        // Should not panic; negacyclic wraparound may apply
        let _ = lookup(
            |_| spec.from_message(0),
            inp,
            LookupCheck::AllowInputPadding,
        );
    }

    #[test]
    #[should_panic(
        expected = "Encountered active padding bit in output when executing lookup with check AllowInputPadding."
    )]
    fn test_lookup_allow_input_padding_still_panics_on_output_padding() {
        let spec = CiphertextBlockSpec(2, 4);
        let inp = spec.from_message(0);
        lookup(
            |x| x.spec().from_complete(1 << spec.data_size()),
            inp,
            LookupCheck::AllowInputPadding,
        );
    }

    #[test]
    fn test_lookup_allow_output_padding_does_not_panic_on_output_padding() {
        let spec = CiphertextBlockSpec(2, 4);
        let inp = spec.from_message(0);
        let _ = lookup(
            |x| x.spec().from_complete(1 << spec.data_size()),
            inp,
            LookupCheck::AllowOutputPadding,
        );
    }

    #[test]
    #[should_panic(
        expected = "Encountered active padding bit in input when executing lookup with check AllowOutputPadding."
    )]
    fn test_lookup_allow_output_padding_still_panics_on_input_padding() {
        let spec = CiphertextBlockSpec(2, 4);
        let inp = spec.from_complete(1 << spec.data_size());
        lookup(|x| x, inp, LookupCheck::AllowOutputPadding);
    }

    #[test]
    fn test_lookup_allow_both_padding_accepts_input_padding() {
        let spec = CiphertextBlockSpec(2, 4);
        let inp = spec.from_complete(1 << spec.data_size());
        let _ = lookup(|_| spec.from_message(0), inp, LookupCheck::AllowBothPadding);
    }

    #[test]
    fn test_lookup_allow_both_padding_accepts_output_padding() {
        let spec = CiphertextBlockSpec(2, 4);
        let inp = spec.from_message(0);
        let _ = lookup(
            |x| x.spec().from_complete(1 << spec.data_size()),
            inp,
            LookupCheck::AllowBothPadding,
        );
    }

    #[test]
    fn test_lookup2_applies_both_luts() {
        let spec = CiphertextBlockSpec(2, 4);
        let inp = spec.from_message(1); // fits in lower half
        let (a, b) = lookup2(
            |x| x.spec().from_message(10),
            |x| x.spec().from_message(11),
            inp,
        );
        assert_eq!(a, spec.from_message(10));
        assert_eq!(b, spec.from_message(11));
    }

    #[test]
    fn test_lookup2_identity_both_luts() {
        let spec = CiphertextBlockSpec(2, 4);
        let inp = spec.from_message(3);
        let (a, b) = lookup2(|x| x, |x| x, inp);
        assert_eq!(a, inp);
        assert_eq!(b, inp);
    }

    #[test]
    #[should_panic(expected = "Encountered active padding bit in input when executing lookup2.")]
    fn test_lookup2_panics_on_input_padding_set() {
        let spec = CiphertextBlockSpec(2, 4);
        let inp = spec.from_complete(1 << spec.data_size());
        lookup2(|x| x, |x| x, inp);
    }

    #[test]
    #[should_panic(expected = "Encountered active many lut bit in input when executing lookup2.")]
    fn test_lookup2_panics_on_topmost_data_bit_set() {
        let spec = CiphertextBlockSpec(2, 4);
        // Set the topmost data bit (many-LUT index overflow)
        let inp = spec.from_data(1 << (spec.data_size() - 1));
        lookup2(|x| x, |x| x, inp);
    }

    #[test]
    #[should_panic(expected = "Encountered active padding bit in output when executing lookup2.")]
    fn test_lookup2_panics_on_first_output_padding_set() {
        let spec = CiphertextBlockSpec(2, 4);
        let inp = spec.from_message(1);
        lookup2(
            |x| x.spec().from_complete(1 << spec.data_size()),
            |x| x,
            inp,
        );
    }

    #[test]
    #[should_panic(expected = "Encountered active padding bit in output when executing lookup2.")]
    fn test_lookup2_panics_on_second_output_padding_set() {
        let spec = CiphertextBlockSpec(2, 4);
        let inp = spec.from_message(1);
        lookup2(
            |x| x,
            |x| x.spec().from_complete(1 << spec.data_size()),
            inp,
        );
    }

    #[test]
    fn test_lookup4_applies_all_four_luts() {
        let spec = CiphertextBlockSpec(2, 4);
        let inp = spec.from_message(1); // fits in lower quarter
        let (a, b, c, d) = lookup4(
            |x| x.spec().from_message(10),
            |x| x.spec().from_message(11),
            |x| x.spec().from_message(12),
            |x| x.spec().from_message(13),
            inp,
        );
        assert_eq!(a, spec.from_message(10));
        assert_eq!(b, spec.from_message(11));
        assert_eq!(c, spec.from_message(12));
        assert_eq!(d, spec.from_message(13));
    }

    #[test]
    #[should_panic(expected = "Encountered active padding bit in input when executing lookup4.")]
    fn test_lookup4_panics_on_input_padding_set() {
        let spec = CiphertextBlockSpec(2, 4);
        let inp = spec.from_complete(1 << spec.data_size());
        lookup4(|x| x, |x| x, |x| x, |x| x, inp);
    }

    #[test]
    #[should_panic(expected = "Encountered active many lut bit in input when executing lookup4.")]
    fn test_lookup4_panics_on_two_topmost_data_bits_set() {
        let spec = CiphertextBlockSpec(2, 4);
        // Set one of the 2 topmost data bits (many-LUT index overflow)
        let inp = spec.from_data(1 << (spec.data_size() - 2));
        lookup4(|x| x, |x| x, |x| x, |x| x, inp);
    }

    #[test]
    #[should_panic(expected = "Encountered active padding bit in output when executing lookup4.")]
    fn test_lookup4_panics_on_any_output_padding_set() {
        let spec = CiphertextBlockSpec(2, 4);
        let inp = spec.from_message(0);
        lookup4(
            |x| x,
            |x| x,
            |x| x.spec().from_complete(1 << spec.data_size()),
            |x| x,
            inp,
        );
    }

    #[test]
    fn test_lookup8_applies_all_eight_luts() {
        let spec = CiphertextBlockSpec(2, 4);
        let inp = spec.from_message(0); // fits in lower eighth
        let (a, b, c, d, e, f, g, h) = lookup8(
            |x| x.spec().from_message(0),
            |x| x.spec().from_message(1),
            |x| x.spec().from_message(2),
            |x| x.spec().from_message(3),
            |x| x.spec().from_message(4),
            |x| x.spec().from_message(5),
            |x| x.spec().from_message(6),
            |x| x.spec().from_message(7),
            inp,
        );
        assert_eq!(a, spec.from_message(0));
        assert_eq!(b, spec.from_message(1));
        assert_eq!(c, spec.from_message(2));
        assert_eq!(d, spec.from_message(3));
        assert_eq!(e, spec.from_message(4));
        assert_eq!(f, spec.from_message(5));
        assert_eq!(g, spec.from_message(6));
        assert_eq!(h, spec.from_message(7));
    }

    #[test]
    #[should_panic(expected = "Encountered active padding bit in input when executing lookup8.")]
    fn test_lookup8_panics_on_input_padding_set() {
        let spec = CiphertextBlockSpec(2, 4);
        let inp = spec.from_complete(1 << spec.data_size());
        lookup8(|x| x, |x| x, |x| x, |x| x, |x| x, |x| x, |x| x, |x| x, inp);
    }

    #[test]
    #[should_panic(expected = "Encountered active many lut bit in input when executing lookup8.")]
    fn test_lookup8_panics_on_three_topmost_data_bits_set() {
        let spec = CiphertextBlockSpec(2, 4);
        // Set one of the 3 topmost data bits (many-LUT index overflow)
        let inp = spec.from_data(1 << (spec.data_size() - 3));
        lookup8(|x| x, |x| x, |x| x, |x| x, |x| x, |x| x, |x| x, |x| x, inp);
    }

    #[test]
    #[should_panic(expected = "Encountered active padding bit in output when executing lookup8.")]
    fn test_lookup8_panics_on_any_output_padding_set() {
        let spec = CiphertextBlockSpec(2, 4);
        let inp = spec.from_message(0);
        lookup8(
            |x| x,
            |x| x,
            |x| x,
            |x| x,
            |x| x,
            |x| x.spec().from_complete(1 << spec.data_size()),
            |x| x,
            |x| x,
            inp,
        );
    }
}
