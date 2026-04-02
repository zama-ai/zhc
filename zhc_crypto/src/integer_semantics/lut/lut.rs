use crate::integer_semantics::{CiphertextBlockSpec, EmulatedCiphertextBlock};
use std::fmt::Debug;
use std::hash::Hash;
use zhc_utils::iter::CollectInVec;
use zhc_utils::{Dumpable, SafeAs};

/// Padding-bit assertion policy for LUT lookups.
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

/// A single-output lookup table for PBS emulation.
///
/// Encapsulates a precomputed lookup table that maps each possible data-space input to a single
/// output block. The table is built from a closure at construction time and stored for efficient
/// repeated evaluation.
///
/// When the input padding bit is set, the output undergoes negacyclic negation to emulate the
/// behavior of real TFHE bootstrapping.
///
/// # Examples
///
/// ```rust,no_run
/// # use zhc_crypto::integer_semantics::{CiphertextBlockSpec, lut::{Lut1, LookupCheck}};
/// let spec = CiphertextBlockSpec(2, 4);
///
/// // Build a LUT that doubles the message value (mod 2^message_size)
/// let double = Lut1::from_fn("double", spec, |b| {
///     spec.from_message((b.raw_message_bits() * 2) & spec.message_mask())
/// });
///
/// let input = spec.from_message(5);
/// let output = double.lookup(input, LookupCheck::Protect);
/// assert_eq!(output.raw_message_bits(), 10);
/// ```
#[derive(Clone)]
pub struct Lut1 {
    lut: Vec<EmulatedCiphertextBlock>,
    name: String,
    spec: CiphertextBlockSpec,
}

impl Lut1 {
    /// Returns the name assigned to this LUT at construction.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the block specification this LUT operates on.
    pub fn spec(&self) -> &CiphertextBlockSpec {
        &self.spec
    }

    /// Constructs a LUT by evaluating a function over the entire data space.
    ///
    /// The function `f` is called once for each of the `2^data_size()` possible input values
    /// (with padding bit clear). The results are stored for later lookup.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_crypto::integer_semantics::{CiphertextBlockSpec, lut::Lut1};
    /// let spec = CiphertextBlockSpec(2, 4);
    /// let identity = Lut1::from_fn("identity", spec, |b| b);
    /// ```
    pub fn from_fn(
        name: impl AsRef<str>,
        spec: CiphertextBlockSpec,
        f: impl Fn(EmulatedCiphertextBlock) -> EmulatedCiphertextBlock,
    ) -> Self {
        let name = name.as_ref().to_string();
        let lut = spec.iter_data_space().map(f).covec();
        assert_eq!(lut.len(), 2_usize.pow(spec.data_size().sas()));
        Self { name, lut, spec }
    }

    /// Applies the LUT to an input block with the specified padding-bit policy.
    ///
    /// The input's data bits index into the precomputed table. If the input padding bit is set,
    /// the raw table output is negacyclically negated to emulate TFHE's negacyclic polynomial
    /// evaluation.
    ///
    /// # Panics
    ///
    /// Panics if the input spec does not match this LUT's spec, if the input padding bit is set
    /// and `check` requires it to be clear, or if the output padding bit is set and `check`
    /// requires it to be clear.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_crypto::integer_semantics::{CiphertextBlockSpec, lut::{Lut1, LookupCheck}};
    /// let spec = CiphertextBlockSpec(2, 4);
    /// let lut = Lut1::from_fn("clear_carry", spec, |b| b.mask_message());
    /// let result = lut.lookup(spec.from_data(0b11_0101), LookupCheck::Protect);
    /// assert_eq!(result.raw_message_bits(), 0b0101);
    /// ```
    pub fn lookup(
        &self,
        inp: EmulatedCiphertextBlock,
        check: LookupCheck,
    ) -> EmulatedCiphertextBlock {
        assert_eq!(inp.spec(), self.spec, "Spec mismatch.");
        if check.should_check_input_padding() {
            assert!(
                !inp.has_active_padding_bit(),
                "Encountered active padding bit in input when executing lookup with check {check:?}."
            );
        }
        let wop_inp = inp.raw_data_bits();
        let mut output = self.lut[wop_inp.sas::<usize>()];
        assert!(
            output.storage >> inp.spec().complete_size() == 0,
            "Lookup output is invalid."
        );
        if inp.has_active_padding_bit() {
            output = output.neg();
        }
        if check.should_check_output_padding() {
            assert!(
                !output.has_active_padding_bit(),
                "Encountered active padding bit in output when executing lookup with check {check:?}."
            );
        }
        output
    }
}

impl PartialEq for Lut1 {
    fn eq(&self, other: &Self) -> bool {
        self.lut == other.lut
    }
}

impl Eq for Lut1 {}

impl Debug for Lut1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Lut1({:?})", self.name)
    }
}

impl Hash for Lut1 {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.lut.hash(state);
    }
}

impl Dumpable for Lut1 {
    fn dump_to_string(&self) -> String {
        // Column width based on ctblock format: "p_cc_mmmm_ctblock"
        let min_col_w = self.spec.complete_size() as usize + 4 + 8; // bits + separators + "_ctblock"
        let title = format!("Lut1({:?}) @ {:?}", self.name, self.spec);
        // For 2 columns: total = 2*(col_w+2) + 1 = 2*col_w + 5
        // Ensure title fits: title.len() + 2 <= 2*col_w + 5
        let col_w = min_col_w.max((title.len() + 2).saturating_sub(5).div_ceil(2));
        let sep = "═".repeat(col_w + 2);
        let total_w = 2 * col_w + 5;
        let top = "═".repeat(total_w);
        let mut result = format!("╔{top}╗\n║ {title}");
        result.push_str(&" ".repeat(total_w - title.len() - 1));
        result.push_str(&format!(
            "║
╠{sep}╦{sep}╣
║ {:^col_w$} ║ {:^col_w$} ║
╠{sep}╬{sep}╣",
            "Input", "Output"
        ));
        for (i, out) in self.lut.iter().enumerate() {
            let inp = self.spec.from_data(i.sas());
            result.push_str(&format!(
                "\n║ {:^col_w$} ║ {:^col_w$} ║",
                inp.dump_to_string(),
                out.dump_to_string()
            ));
        }
        result.push_str(&format!("\n╚{sep}╩{sep}╝"));
        result
    }
}

/// A two-output lookup table for many-LUT PBS emulation.
///
/// Encapsulates a precomputed lookup table that evaluates two functions simultaneously on the
/// same input, returning both results. This emulates the TFHE "many-LUT" optimization where
/// multiple outputs can be extracted from a single PBS operation by packing sub-tables into
/// different regions of the polynomial.
///
/// The input must have its padding bit clear **and** its second-to-last data bit (the "many-LUT
/// index bit") clear. These bits are reserved for the many-LUT encoding.
///
/// Unlike [`Lut1`], this type does not support `AllowInputPadding` or `AllowBothPadding` modes
/// because the many-LUT encoding requires strict control over the input bit layout.
///
/// # Examples
///
/// ```rust,no_run
/// # use zhc_crypto::integer_semantics::{CiphertextBlockSpec, lut::{Lut2, LookupCheck}};
/// let spec = CiphertextBlockSpec(2, 4);
///
/// // Build a LUT that returns message and carry separately
/// let split = Lut2::from_fn(
///     "split_msg_carry",
///     spec,
///     |b| spec.from_message(b.raw_message_bits()),  // first output: message
///     |b| spec.from_message(b.raw_carry_bits()),    // second output: carry
/// );
///
/// let input = spec.from_data(0b01_0101); // carry=1, message=5
/// let (msg, carry) = split.lookup(input, LookupCheck::Protect);
/// assert_eq!(msg.raw_message_bits(), 5);
/// assert_eq!(carry.raw_message_bits(), 1);
/// ```
#[derive(Clone)]
pub struct Lut2 {
    lut: Vec<EmulatedCiphertextBlock>,
    name: String,
    spec: CiphertextBlockSpec,
}

impl Lut2 {
    /// Returns the name assigned to this LUT at construction.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the block specification this LUT operates on.
    pub fn spec(&self) -> &CiphertextBlockSpec {
        &self.spec
    }

    /// Constructs a two-output LUT by evaluating two functions over valid inputs.
    ///
    /// Both functions are called for each valid input (those with the many-LUT index bit clear).
    /// The results are interleaved in the internal table to enable simultaneous lookup.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_crypto::integer_semantics::{CiphertextBlockSpec, lut::Lut2};
    /// let spec = CiphertextBlockSpec(2, 4);
    /// let lut = Lut2::from_fn("dual_identity", spec, |b| b, |b| b);
    /// ```
    pub fn from_fn(
        name: impl AsRef<str>,
        spec: CiphertextBlockSpec,
        f1: impl Fn(EmulatedCiphertextBlock) -> EmulatedCiphertextBlock,
        f2: impl Fn(EmulatedCiphertextBlock) -> EmulatedCiphertextBlock,
    ) -> Self {
        let name = name.as_ref().to_string();
        let lut = spec
            .iter_data_space()
            .filter(|c| !c.has_active_last_ith_bit(1))
            .map(|c| f1(c))
            .chain(
                spec.iter_data_space()
                    .filter(|c| !c.has_active_last_ith_bit(1))
                    .map(|c| f2(c)),
            )
            .covec();
        assert_eq!(lut.len(), 2_usize.pow(spec.data_size().sas()));
        Self { name, lut, spec }
    }

    /// Applies the LUT to an input block, returning both output values.
    ///
    /// The input must have both the padding bit and the many-LUT index bit (second-to-last data
    /// bit) clear. The first output comes from `f1`, the second from `f2`.
    ///
    /// # Panics
    ///
    /// Panics if the input spec does not match, if the padding bit is set, if the many-LUT index
    /// bit is set, if `check` is `AllowInputPadding` or `AllowBothPadding` (not supported), or if
    /// any output padding bit is set and `check` requires it to be clear.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_crypto::integer_semantics::{CiphertextBlockSpec, lut::{Lut2, LookupCheck}};
    /// let spec = CiphertextBlockSpec(2, 4);
    /// let lut = Lut2::from_fn("add_sub", spec,
    ///     |b| spec.from_message((b.raw_message_bits() + 1) & spec.message_mask()),
    ///     |b| spec.from_message(b.raw_message_bits().wrapping_sub(1) & spec.message_mask()),
    /// );
    /// let (plus, minus) = lut.lookup(spec.from_message(5), LookupCheck::Protect);
    /// assert_eq!(plus.raw_message_bits(), 6);
    /// assert_eq!(minus.raw_message_bits(), 4);
    /// ```
    pub fn lookup(
        &self,
        inp: EmulatedCiphertextBlock,
        check: LookupCheck,
    ) -> (EmulatedCiphertextBlock, EmulatedCiphertextBlock) {
        assert_eq!(inp.spec(), self.spec, "Spec mismatch.");
        assert!(
            matches!(
                check,
                LookupCheck::Protect | LookupCheck::AllowOutputPadding
            ),
            "Encountered incompatible check for many-lut lookup"
        );
        assert!(
            !inp.has_active_padding_bit(),
            "Encountered active padding bit in input when executing lookup2."
        );
        assert!(
            !inp.has_active_last_ith_bit(1),
            "Encountered active many lut bit in input when executing lookup2."
        );

        let wop_inp = inp.raw_data_bits();
        let output1 = self.lut[wop_inp.sas::<usize>()];
        assert!(
            output1.storage >> inp.spec().complete_size() == 0,
            "Lookup output is invalid."
        );
        let output2 = self.lut[wop_inp.sas::<usize>() + self.lut.len() / 2];
        assert!(
            output2.storage >> inp.spec().complete_size() == 0,
            "Lookup output is invalid."
        );
        if check.should_check_output_padding() {
            assert!(
                !output1.has_active_padding_bit(),
                "Encountered active padding bit in output when executing lookup2."
            );
            assert!(
                !output2.has_active_padding_bit(),
                "Encountered active padding bit in output when executing lookup2."
            );
        }
        (output1, output2)
    }
}

impl PartialEq for Lut2 {
    fn eq(&self, other: &Self) -> bool {
        self.lut == other.lut
    }
}

impl Eq for Lut2 {}

impl Debug for Lut2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Lut2({:?})", self.name)
    }
}

impl Hash for Lut2 {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.lut.hash(state);
    }
}

impl Dumpable for Lut2 {
    fn dump_to_string(&self) -> String {
        let half = self.lut.len() / 2;
        let min_col_w = self.spec.complete_size() as usize + 4 + 8;
        let title = format!("Lut2({:?}) @ {:?}", self.name, self.spec);
        // For 3 columns: total = 3*(col_w+2) + 2 = 3*col_w + 8
        // Ensure title fits: title.len() + 2 <= 3*col_w + 8
        let col_w = min_col_w.max((title.len() + 2).saturating_sub(8).div_ceil(3));
        let sep = "═".repeat(col_w + 2);
        let total_w = 3 * col_w + 8;
        let top = "═".repeat(total_w);
        let mut result = format!("╔{top}╗\n║ {title}");
        result.push_str(&" ".repeat(total_w - title.len() - 1));
        result.push_str(&format!(
            "║
╠{sep}╦{sep}╦{sep}╣
║ {:^col_w$} ║ {:^col_w$} ║ {:^col_w$} ║
╠{sep}╬{sep}╬{sep}╣",
            "Input", "Out₁", "Out₂"
        ));
        for i in 0..half {
            let inp = self.spec.from_data(i.sas());
            let out1 = &self.lut[i];
            let out2 = &self.lut[i + half];
            result.push_str(&format!(
                "\n║ {:^col_w$} ║ {:^col_w$} ║ {:^col_w$} ║",
                inp.dump_to_string(),
                out1.dump_to_string(),
                out2.dump_to_string()
            ));
        }
        result.push_str(&format!("\n╚{sep}╩{sep}╩{sep}╝"));
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::integer_semantics::CiphertextBlockSpec;

    #[test]
    fn test_lookup_identity_with_clean_padding() {
        let spec = CiphertextBlockSpec(2, 4);
        let lut = Lut1::from_fn("test", spec, |x| x);
        for c in spec.iter_data_space() {
            let result = lut.lookup(c, LookupCheck::AllowBothPadding);
            if c.raw_padding_bits() == 1 {
                assert_eq!(result, c.neg());
            } else {
                assert_eq!(result, c);
            }
        }
    }

    #[test]
    fn test_lookup_applies_lut_function() {
        let spec = CiphertextBlockSpec(2, 4);
        let lut = Lut1::from_fn("test", spec, |x| x.spec().from_message(7));
        for c in spec.iter_data_space() {
            let result = lut.lookup(c, LookupCheck::Protect);
            assert_eq!(result, spec.from_message(7));
        }
    }

    #[test]
    #[should_panic(
        expected = "Encountered active padding bit in input when executing lookup with check Protect."
    )]
    fn test_lookup_protect_panics_on_input_padding_set() {
        let spec = CiphertextBlockSpec(2, 4);
        let inp = spec.from_complete(1 << spec.data_size()); // padding bit set
        let lut = Lut1::from_fn("test", spec, |x| x);
        let _ = lut.lookup(inp, LookupCheck::Protect);
    }

    #[test]
    #[should_panic(
        expected = "Encountered active padding bit in output when executing lookup with check Protect."
    )]
    fn test_lookup_protect_panics_on_output_padding_set() {
        let spec = CiphertextBlockSpec(2, 4);
        let inp = spec.from_message(0);
        let lut = Lut1::from_fn("test", spec, |x| {
            x.spec().from_complete(1 << spec.data_size())
        });
        let _ = lut.lookup(inp, LookupCheck::Protect);
    }

    #[test]
    fn test_lookup_allow_input_padding_does_not_panic_on_input_padding() {
        let spec = CiphertextBlockSpec(2, 4);
        // Should not panic; negacyclic wraparound may apply
        let lut = Lut1::from_fn("test", spec, |_| spec.from_message(0));
        for c in spec.iter_complete_space() {
            let _ = lut.lookup(c, LookupCheck::AllowInputPadding);
        }
    }

    #[test]
    #[should_panic(
        expected = "Encountered active padding bit in output when executing lookup with check AllowInputPadding."
    )]
    fn test_lookup_allow_input_padding_still_panics_on_output_padding() {
        let spec = CiphertextBlockSpec(2, 4);
        let inp = spec.from_message(0);
        let lut = Lut1::from_fn("test", spec, |x| {
            x.spec().from_complete(1 << spec.data_size())
        });
        lut.lookup(inp, LookupCheck::AllowInputPadding);
    }

    #[test]
    fn test_lookup_allow_output_padding_does_not_panic_on_output_padding() {
        let spec = CiphertextBlockSpec(2, 4);
        let lut = Lut1::from_fn("test", spec, |x| {
            x.spec().from_complete(1 << spec.data_size())
        });
        for c in spec.iter_data_space() {
            let _ = lut.lookup(c, LookupCheck::AllowOutputPadding);
        }
    }

    #[test]
    #[should_panic(
        expected = "Encountered active padding bit in input when executing lookup with check AllowOutputPadding."
    )]
    fn test_lookup_allow_output_padding_still_panics_on_input_padding() {
        let spec = CiphertextBlockSpec(2, 4);
        let inp = spec.from_complete(1 << spec.data_size());
        let lut = Lut1::from_fn("test", spec, |x| x);
        lut.lookup(inp, LookupCheck::AllowOutputPadding);
    }

    #[test]
    fn test_lut2_lookup_returns_both_function_results() {
        let spec = CiphertextBlockSpec(2, 4);
        // f1 returns constant 15, f2 returns constant 7
        let lut = Lut2::from_fn(
            "test",
            spec,
            |_| spec.from_message(15),
            |_| spec.from_message(7),
        );
        let inp = spec.from_message(3);
        let (out1, out2) = lut.lookup(inp, LookupCheck::Protect);
        // out1 is from f1 (upper half), out2 is from f2 (lower half)
        assert_eq!(out1, spec.from_message(15));
        assert_eq!(out2, spec.from_message(7));
    }

    #[test]
    fn test_lut2_lookup_identity_functions() {
        let spec = CiphertextBlockSpec(2, 4);
        // f1 doubles the message, f2 is identity
        let lut = Lut2::from_fn(
            "test",
            spec,
            |x| spec.from_message((x.raw_message_bits() * 2) & spec.message_mask()),
            |x| spec.from_message(x.raw_message_bits()),
        );
        let inp = spec.from_message(5);
        let (out1, out2) = lut.lookup(inp, LookupCheck::Protect);
        assert_eq!(out1.raw_message_bits(), 10); // f1: doubled
        assert_eq!(out2.raw_message_bits(), 5); // f2: identity
    }

    #[test]
    #[should_panic(expected = "Encountered active padding bit in input when executing lookup2.")]
    fn test_lut2_panics_on_input_padding_set() {
        let spec = CiphertextBlockSpec(2, 4);
        let lut = Lut2::from_fn("test", spec, |x| x, |x| x);
        let inp = spec.from_complete(1 << spec.data_size()); // padding bit set
        let _ = lut.lookup(inp, LookupCheck::Protect);
    }

    #[test]
    #[should_panic(expected = "Encountered active many lut bit in input when executing lookup2.")]
    fn test_lut2_panics_on_many_lut_bit_set() {
        let spec = CiphertextBlockSpec(2, 4);
        let lut = Lut2::from_fn("test", spec, |x| x, |x| x);
        let inp = spec.from_data(0b0_10_0010); // bit 1 set
        let _ = lut.lookup(inp, LookupCheck::Protect);
    }

    #[test]
    #[should_panic(expected = "Encountered active padding bit in output when executing lookup2.")]
    fn test_lut2_protect_panics_on_output_padding() {
        let spec = CiphertextBlockSpec(2, 4);
        let lut = Lut2::from_fn(
            "test",
            spec,
            |_| spec.from_complete(1 << spec.data_size()), // padding set
            |_| spec.from_message(0),
        );
        let inp = spec.from_message(0);
        let _ = lut.lookup(inp, LookupCheck::Protect);
    }

    #[test]
    fn test_lut2_allow_output_padding_does_not_panic() {
        let spec = CiphertextBlockSpec(2, 4);
        let lut = Lut2::from_fn(
            "test",
            spec,
            |_| spec.from_complete(1 << spec.data_size()), // padding set
            |_| spec.from_complete(1 << spec.data_size()), // padding set
        );
        let inp = spec.from_message(0);
        let _ = lut.lookup(inp, LookupCheck::AllowOutputPadding); // should not panic
    }

    #[test]
    #[should_panic(expected = "Encountered incompatible check for many-lut lookup")]
    fn test_lut2_rejects_allow_input_padding_check() {
        let spec = CiphertextBlockSpec(2, 4);
        let lut = Lut2::from_fn("test", spec, |x| x, |x| x);
        let inp = spec.from_message(0);
        let _ = lut.lookup(inp, LookupCheck::AllowInputPadding);
    }

    #[test]
    #[should_panic(expected = "Encountered incompatible check for many-lut lookup")]
    fn test_lut2_rejects_allow_both_padding_check() {
        let spec = CiphertextBlockSpec(2, 4);
        let lut = Lut2::from_fn("test", spec, |x| x, |x| x);
        let inp = spec.from_message(0);
        let _ = lut.lookup(inp, LookupCheck::AllowBothPadding);
    }

    #[test]
    fn test_lut2_iterates_all_valid_inputs() {
        let spec = CiphertextBlockSpec(2, 4);
        let lut = Lut2::from_fn(
            "test",
            spec,
            |_| spec.from_message(1),
            |_| spec.from_message(2),
        );
        // Valid inputs: no padding, no bit 1 set
        for msg in (0..16u16).filter(|m| m & 0b10 == 0) {
            let inp = spec.from_message(msg);
            let (out1, out2) = lut.lookup(inp, LookupCheck::Protect);
            assert_eq!(out1, spec.from_message(1)); // f1
            assert_eq!(out2, spec.from_message(2)); // f2
        }
    }
}
