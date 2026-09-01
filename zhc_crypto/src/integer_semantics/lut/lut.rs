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

#[derive(Clone)]
pub struct RawLut {
    lut: Vec<EmulatedCiphertextBlock>,
    name: String,
    spec: CiphertextBlockSpec,
}

impl RawLut {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn spec(&self) -> &CiphertextBlockSpec {
        &self.spec
    }

    pub fn lut(&self) -> &[EmulatedCiphertextBlock] {
        self.lut.as_slice()
    }
}

impl PartialEq for RawLut {
    fn eq(&self, other: &Self) -> bool {
        self.lut == other.lut
    }
}

impl Eq for RawLut {}

impl Debug for RawLut {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.name.fmt(f)
    }
}

impl Hash for RawLut {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.lut.hash(state);
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
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone, Hash)]
pub struct Lut1(pub RawLut);

impl Lut1 {
    /// Returns the name assigned to this LUT at construction.
    pub fn name(&self) -> &str {
        &self.0.name
    }

    /// Returns the block specification this LUT operates on.
    pub fn spec(&self) -> &CiphertextBlockSpec {
        &self.0.spec
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
        Self(RawLut { name, lut, spec })
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
        assert_eq!(inp.spec(), self.0.spec, "Spec mismatch.");
        if check.should_check_input_padding() {
            assert!(
                !inp.has_active_padding_bit(),
                "Encountered active padding bit in input when executing lookup with check {check:?}."
            );
        }
        let wop_inp = inp.raw_data_bits();
        let mut output = self.0.lut[wop_inp.sas::<usize>()];
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

/// Renders a LUT as a boxed table with one input column and `n_out` output columns.
///
/// The internal table is interpreted as `n_out` consecutive sub-tables of equal length, as laid
/// out by the `from_fn` constructors.
fn dump_lut_table(raw: &RawLut, kind: &str, n_out: usize) -> String {
    const SUBSCRIPTS: [char; 8] = ['₁', '₂', '₃', '₄', '₅', '₆', '₇', '₈'];
    let rows = raw.lut.len() / n_out;
    let cols = n_out + 1;
    // Column width based on ctblock format: "p_cc_mmmm_ctblock"
    let min_col_w = raw.spec.complete_size() as usize + 4 + 8; // bits + separators + "_ctblock"
    let title = format!("{kind}({:?}) @ {:?}", raw.name, raw.spec);
    // Total width: cols*(col_w+2) + (cols-1) = cols*col_w + 3*cols - 1
    // Ensure title fits: title.len() + 2 <= cols*col_w + 3*cols - 1
    let col_w = min_col_w.max(
        (title.len() + 2)
            .saturating_sub(3 * cols - 1)
            .div_ceil(cols),
    );
    let seps = vec!["═".repeat(col_w + 2); cols];
    let total_w = cols * col_w + 3 * cols - 1;
    let mut result = format!("╔{}╗\n║ {title}", "═".repeat(total_w));
    result.push_str(&" ".repeat(total_w - title.len() - 1));
    let headers = std::iter::once("Input".to_string())
        .chain((0..n_out).map(|i| {
            if n_out == 1 {
                "Output".to_string()
            } else {
                format!("Out{}", SUBSCRIPTS[i])
            }
        }))
        .map(|h| format!(" {h:^col_w$} "))
        .covec();
    result.push_str(&format!(
        "║\n╠{}╣\n║{}║\n╠{}╣",
        seps.join("╦"),
        headers.join("║"),
        seps.join("╬")
    ));
    for i in 0..rows {
        let inp = raw.spec.from_data(i.sas());
        let cells = std::iter::once(inp)
            .chain((0..n_out).map(|k| raw.lut[i + k * rows]))
            .map(|c| format!(" {:^col_w$} ", c.dump_to_string()))
            .covec();
        result.push_str(&format!("\n║{}║", cells.join("║")));
    }
    result.push_str(&format!("\n╚{}╝", seps.join("╩")));
    result
}

impl Dumpable for Lut1 {
    fn dump_to_string(&self) -> String {
        dump_lut_table(&self.0, "Lut1", 1)
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
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone, Hash)]
pub struct Lut2(pub RawLut);

impl Lut2 {
    /// Returns the name assigned to this LUT at construction.
    pub fn name(&self) -> &str {
        &self.0.name
    }

    /// Returns the block specification this LUT operates on.
    pub fn spec(&self) -> &CiphertextBlockSpec {
        &self.0.spec
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
        Self(RawLut { name, lut, spec })
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
        assert_eq!(inp.spec(), self.0.spec, "Spec mismatch.");
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
        let output1 = self.0.lut[wop_inp.sas::<usize>()];
        assert!(
            output1.storage >> inp.spec().complete_size() == 0,
            "Lookup output is invalid."
        );
        let output2 = self.0.lut[wop_inp.sas::<usize>() + self.0.lut.len() / 2];
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

impl Dumpable for Lut2 {
    fn dump_to_string(&self) -> String {
        dump_lut_table(&self.0, "Lut2", 2)
    }
}

/// A four-output lookup table for many-LUT PBS emulation.
///
/// Encapsulates a precomputed lookup table that evaluates four functions simultaneously on the
/// same input, returning all four results. This emulates the TFHE "many-LUT" optimization where
/// multiple outputs can be extracted from a single PBS operation by packing sub-tables into
/// different regions of the polynomial.
///
/// The input must have its padding bit clear **and** its two topmost data bits (the "many-LUT
/// index bits") clear. These bits are reserved for the many-LUT encoding.
///
/// Unlike [`Lut1`], this type does not support `AllowInputPadding` or `AllowBothPadding` modes
/// because the many-LUT encoding requires strict control over the input bit layout.
///
/// # Examples
///
/// ```rust,no_run
/// # use zhc_crypto::integer_semantics::{CiphertextBlockSpec, lut::{Lut4, LookupCheck}};
/// let spec = CiphertextBlockSpec(2, 4);
///
/// // Build a LUT that returns the message shifted by 0, 1, 2 and 3
/// let shifts = Lut4::from_fn(
///     "shifts",
///     spec,
///     |b| spec.from_message(b.raw_message_bits()),
///     |b| spec.from_message((b.raw_message_bits() << 1) & spec.message_mask()),
///     |b| spec.from_message((b.raw_message_bits() << 2) & spec.message_mask()),
///     |b| spec.from_message((b.raw_message_bits() << 3) & spec.message_mask()),
/// );
///
/// let input = spec.from_message(1);
/// let (s0, s1, s2, s3) = shifts.lookup(input, LookupCheck::Protect);
/// assert_eq!(s0.raw_message_bits(), 1);
/// assert_eq!(s1.raw_message_bits(), 2);
/// assert_eq!(s2.raw_message_bits(), 4);
/// assert_eq!(s3.raw_message_bits(), 8);
/// ```
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone, Hash)]
pub struct Lut4(pub RawLut);

impl Lut4 {
    /// Returns the name assigned to this LUT at construction.
    pub fn name(&self) -> &str {
        &self.0.name
    }

    /// Returns the block specification this LUT operates on.
    pub fn spec(&self) -> &CiphertextBlockSpec {
        &self.0.spec
    }

    /// Constructs a four-output LUT by evaluating four functions over valid inputs.
    ///
    /// All functions are called for each valid input (those with the two many-LUT index bits
    /// clear). The results are stored as four consecutive sub-tables to enable simultaneous
    /// lookup.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_crypto::integer_semantics::{CiphertextBlockSpec, lut::Lut4};
    /// let spec = CiphertextBlockSpec(2, 4);
    /// let lut = Lut4::from_fn("quad_identity", spec, |b| b, |b| b, |b| b, |b| b);
    /// ```
    pub fn from_fn(
        name: impl AsRef<str>,
        spec: CiphertextBlockSpec,
        f1: impl Fn(EmulatedCiphertextBlock) -> EmulatedCiphertextBlock,
        f2: impl Fn(EmulatedCiphertextBlock) -> EmulatedCiphertextBlock,
        f3: impl Fn(EmulatedCiphertextBlock) -> EmulatedCiphertextBlock,
        f4: impl Fn(EmulatedCiphertextBlock) -> EmulatedCiphertextBlock,
    ) -> Self {
        let name = name.as_ref().to_string();
        let fs: [&dyn Fn(EmulatedCiphertextBlock) -> EmulatedCiphertextBlock; 4] =
            [&f1, &f2, &f3, &f4];
        let lut = fs
            .iter()
            .flat_map(|f| {
                spec.iter_data_space()
                    .filter(|c| !c.has_active_last_ith_bit(1) && !c.has_active_last_ith_bit(2))
                    .map(|c| f(c))
            })
            .covec();
        assert_eq!(lut.len(), 2_usize.pow(spec.data_size().sas()));
        Self(RawLut { name, lut, spec })
    }

    /// Applies the LUT to an input block, returning all four output values.
    ///
    /// The input must have the padding bit and the two many-LUT index bits (topmost data bits)
    /// clear. The outputs come from `f1`, `f2`, `f3` and `f4` in order.
    ///
    /// # Panics
    ///
    /// Panics if the input spec does not match, if the padding bit is set, if a many-LUT index
    /// bit is set, if `check` is `AllowInputPadding` or `AllowBothPadding` (not supported), or if
    /// any output padding bit is set and `check` requires it to be clear.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_crypto::integer_semantics::{CiphertextBlockSpec, lut::{Lut4, LookupCheck}};
    /// let spec = CiphertextBlockSpec(2, 4);
    /// let lut = Lut4::from_fn("quad_identity", spec, |b| b, |b| b, |b| b, |b| b);
    /// let (o1, o2, o3, o4) = lut.lookup(spec.from_message(5), LookupCheck::Protect);
    /// assert_eq!(o1.raw_message_bits(), 5);
    /// ```
    pub fn lookup(
        &self,
        inp: EmulatedCiphertextBlock,
        check: LookupCheck,
    ) -> (
        EmulatedCiphertextBlock,
        EmulatedCiphertextBlock,
        EmulatedCiphertextBlock,
        EmulatedCiphertextBlock,
    ) {
        assert_eq!(inp.spec(), self.0.spec, "Spec mismatch.");
        assert!(
            matches!(
                check,
                LookupCheck::Protect | LookupCheck::AllowOutputPadding
            ),
            "Encountered incompatible check for many-lut lookup"
        );
        assert!(
            !inp.has_active_padding_bit(),
            "Encountered active padding bit in input when executing lookup4."
        );
        for i in [1, 2] {
            assert!(
                !inp.has_active_last_ith_bit(i),
                "Encountered active many lut bit in input when executing lookup4."
            );
        }

        let wop_inp = inp.raw_data_bits();
        let quarter = self.0.lut.len() / 4;
        let outputs = [0usize, 1, 2, 3].map(|k| {
            let output = self.0.lut[wop_inp.sas::<usize>() + k * quarter];
            assert!(
                output.storage >> inp.spec().complete_size() == 0,
                "Lookup output is invalid."
            );
            if check.should_check_output_padding() {
                assert!(
                    !output.has_active_padding_bit(),
                    "Encountered active padding bit in output when executing lookup4."
                );
            }
            output
        });
        (outputs[0], outputs[1], outputs[2], outputs[3])
    }
}

impl Dumpable for Lut4 {
    fn dump_to_string(&self) -> String {
        dump_lut_table(&self.0, "Lut4", 4)
    }
}

/// An eight-output lookup table for many-LUT PBS emulation.
///
/// Encapsulates a precomputed lookup table that evaluates eight functions simultaneously on the
/// same input, returning all eight results. This emulates the TFHE "many-LUT" optimization where
/// multiple outputs can be extracted from a single PBS operation by packing sub-tables into
/// different regions of the polynomial.
///
/// The input must have its padding bit clear **and** its three topmost data bits (the "many-LUT
/// index bits") clear. These bits are reserved for the many-LUT encoding.
///
/// Unlike [`Lut1`], this type does not support `AllowInputPadding` or `AllowBothPadding` modes
/// because the many-LUT encoding requires strict control over the input bit layout.
///
/// # Examples
///
/// ```rust,no_run
/// # use zhc_crypto::integer_semantics::{CiphertextBlockSpec, lut::{Lut8, LookupCheck}};
/// let spec = CiphertextBlockSpec(2, 4);
///
/// // Build a LUT that returns the message plus 0..8
/// let offsets = Lut8::from_fn(
///     "offsets",
///     spec,
///     |b| spec.from_message(b.raw_message_bits()),
///     |b| spec.from_message((b.raw_message_bits() + 1) & spec.message_mask()),
///     |b| spec.from_message((b.raw_message_bits() + 2) & spec.message_mask()),
///     |b| spec.from_message((b.raw_message_bits() + 3) & spec.message_mask()),
///     |b| spec.from_message((b.raw_message_bits() + 4) & spec.message_mask()),
///     |b| spec.from_message((b.raw_message_bits() + 5) & spec.message_mask()),
///     |b| spec.from_message((b.raw_message_bits() + 6) & spec.message_mask()),
///     |b| spec.from_message((b.raw_message_bits() + 7) & spec.message_mask()),
/// );
///
/// let input = spec.from_message(1);
/// let (o0, o1, _, _, _, _, _, o7) = offsets.lookup(input, LookupCheck::Protect);
/// assert_eq!(o0.raw_message_bits(), 1);
/// assert_eq!(o1.raw_message_bits(), 2);
/// assert_eq!(o7.raw_message_bits(), 8);
/// ```
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone, Hash)]
pub struct Lut8(pub RawLut);

impl Lut8 {
    /// Returns the name assigned to this LUT at construction.
    pub fn name(&self) -> &str {
        &self.0.name
    }

    /// Returns the block specification this LUT operates on.
    pub fn spec(&self) -> &CiphertextBlockSpec {
        &self.0.spec
    }

    /// Constructs an eight-output LUT by evaluating eight functions over valid inputs.
    ///
    /// All functions are called for each valid input (those with the three many-LUT index bits
    /// clear). The results are stored as eight consecutive sub-tables to enable simultaneous
    /// lookup.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_crypto::integer_semantics::{CiphertextBlockSpec, lut::Lut8};
    /// let spec = CiphertextBlockSpec(2, 4);
    /// let lut = Lut8::from_fn(
    ///     "octo_identity", spec,
    ///     |b| b, |b| b, |b| b, |b| b, |b| b, |b| b, |b| b, |b| b,
    /// );
    /// ```
    #[allow(clippy::too_many_arguments)]
    pub fn from_fn(
        name: impl AsRef<str>,
        spec: CiphertextBlockSpec,
        f1: impl Fn(EmulatedCiphertextBlock) -> EmulatedCiphertextBlock,
        f2: impl Fn(EmulatedCiphertextBlock) -> EmulatedCiphertextBlock,
        f3: impl Fn(EmulatedCiphertextBlock) -> EmulatedCiphertextBlock,
        f4: impl Fn(EmulatedCiphertextBlock) -> EmulatedCiphertextBlock,
        f5: impl Fn(EmulatedCiphertextBlock) -> EmulatedCiphertextBlock,
        f6: impl Fn(EmulatedCiphertextBlock) -> EmulatedCiphertextBlock,
        f7: impl Fn(EmulatedCiphertextBlock) -> EmulatedCiphertextBlock,
        f8: impl Fn(EmulatedCiphertextBlock) -> EmulatedCiphertextBlock,
    ) -> Self {
        let name = name.as_ref().to_string();
        let fs: [&dyn Fn(EmulatedCiphertextBlock) -> EmulatedCiphertextBlock; 8] =
            [&f1, &f2, &f3, &f4, &f5, &f6, &f7, &f8];
        let lut = fs
            .iter()
            .flat_map(|f| {
                spec.iter_data_space()
                    .filter(|c| {
                        !c.has_active_last_ith_bit(1)
                            && !c.has_active_last_ith_bit(2)
                            && !c.has_active_last_ith_bit(3)
                    })
                    .map(|c| f(c))
            })
            .covec();
        assert_eq!(lut.len(), 2_usize.pow(spec.data_size().sas()));
        Self(RawLut { name, lut, spec })
    }

    /// Applies the LUT to an input block, returning all eight output values.
    ///
    /// The input must have the padding bit and the three many-LUT index bits (topmost data bits)
    /// clear. The outputs come from `f1` through `f8` in order.
    ///
    /// # Panics
    ///
    /// Panics if the input spec does not match, if the padding bit is set, if a many-LUT index
    /// bit is set, if `check` is `AllowInputPadding` or `AllowBothPadding` (not supported), or if
    /// any output padding bit is set and `check` requires it to be clear.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_crypto::integer_semantics::{CiphertextBlockSpec, lut::{Lut8, LookupCheck}};
    /// let spec = CiphertextBlockSpec(2, 4);
    /// let lut = Lut8::from_fn(
    ///     "octo_identity", spec,
    ///     |b| b, |b| b, |b| b, |b| b, |b| b, |b| b, |b| b, |b| b,
    /// );
    /// let (o1, ..) = lut.lookup(spec.from_message(5), LookupCheck::Protect);
    /// assert_eq!(o1.raw_message_bits(), 5);
    /// ```
    #[allow(clippy::type_complexity)]
    pub fn lookup(
        &self,
        inp: EmulatedCiphertextBlock,
        check: LookupCheck,
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
        assert_eq!(inp.spec(), self.0.spec, "Spec mismatch.");
        assert!(
            matches!(
                check,
                LookupCheck::Protect | LookupCheck::AllowOutputPadding
            ),
            "Encountered incompatible check for many-lut lookup"
        );
        assert!(
            !inp.has_active_padding_bit(),
            "Encountered active padding bit in input when executing lookup8."
        );
        for i in [1, 2, 3] {
            assert!(
                !inp.has_active_last_ith_bit(i),
                "Encountered active many lut bit in input when executing lookup8."
            );
        }

        let wop_inp = inp.raw_data_bits();
        let eighth = self.0.lut.len() / 8;
        let outputs = [0usize, 1, 2, 3, 4, 5, 6, 7].map(|k| {
            let output = self.0.lut[wop_inp.sas::<usize>() + k * eighth];
            assert!(
                output.storage >> inp.spec().complete_size() == 0,
                "Lookup output is invalid."
            );
            if check.should_check_output_padding() {
                assert!(
                    !output.has_active_padding_bit(),
                    "Encountered active padding bit in output when executing lookup8."
                );
            }
            output
        });
        (
            outputs[0], outputs[1], outputs[2], outputs[3], outputs[4], outputs[5], outputs[6],
            outputs[7],
        )
    }
}

impl Dumpable for Lut8 {
    fn dump_to_string(&self) -> String {
        dump_lut_table(&self.0, "Lut8", 8)
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

    #[test]
    fn test_lut4_lookup_returns_all_function_results() {
        let spec = CiphertextBlockSpec(2, 4);
        let lut = Lut4::from_fn(
            "test",
            spec,
            |x| spec.from_message(x.raw_message_bits()),
            |x| spec.from_message((x.raw_message_bits() + 1) & spec.message_mask()),
            |x| spec.from_message((x.raw_message_bits() + 2) & spec.message_mask()),
            |x| spec.from_message((x.raw_message_bits() + 3) & spec.message_mask()),
        );
        // Valid inputs: data bits 4 and 5 clear, i.e. no carry, any message
        for msg in 0..16u16 {
            let (o1, o2, o3, o4) = lut.lookup(spec.from_message(msg), LookupCheck::Protect);
            assert_eq!(o1.raw_message_bits(), msg);
            assert_eq!(o2.raw_message_bits(), (msg + 1) & spec.message_mask());
            assert_eq!(o3.raw_message_bits(), (msg + 2) & spec.message_mask());
            assert_eq!(o4.raw_message_bits(), (msg + 3) & spec.message_mask());
        }
    }

    #[test]
    #[should_panic(expected = "Encountered active padding bit in input when executing lookup4.")]
    fn test_lut4_panics_on_input_padding_set() {
        let spec = CiphertextBlockSpec(2, 4);
        let lut = Lut4::from_fn("test", spec, |x| x, |x| x, |x| x, |x| x);
        let inp = spec.from_complete(1 << spec.data_size()); // padding bit set
        let _ = lut.lookup(inp, LookupCheck::Protect);
    }

    #[test]
    #[should_panic(expected = "Encountered active many lut bit in input when executing lookup4.")]
    fn test_lut4_panics_on_first_many_lut_bit_set() {
        let spec = CiphertextBlockSpec(2, 4);
        let lut = Lut4::from_fn("test", spec, |x| x, |x| x, |x| x, |x| x);
        let inp = spec.from_data(0b10_0000); // data bit 5 set
        let _ = lut.lookup(inp, LookupCheck::Protect);
    }

    #[test]
    #[should_panic(expected = "Encountered active many lut bit in input when executing lookup4.")]
    fn test_lut4_panics_on_second_many_lut_bit_set() {
        let spec = CiphertextBlockSpec(2, 4);
        let lut = Lut4::from_fn("test", spec, |x| x, |x| x, |x| x, |x| x);
        let inp = spec.from_data(0b01_0000); // data bit 4 set
        let _ = lut.lookup(inp, LookupCheck::Protect);
    }

    #[test]
    #[should_panic(expected = "Encountered active padding bit in output when executing lookup4.")]
    fn test_lut4_protect_panics_on_output_padding() {
        let spec = CiphertextBlockSpec(2, 4);
        let lut = Lut4::from_fn(
            "test",
            spec,
            |_| spec.from_message(0),
            |_| spec.from_message(0),
            |_| spec.from_message(0),
            |_| spec.from_complete(1 << spec.data_size()), // padding set
        );
        let _ = lut.lookup(spec.from_message(0), LookupCheck::Protect);
    }

    #[test]
    fn test_lut4_allow_output_padding_does_not_panic() {
        let spec = CiphertextBlockSpec(2, 4);
        let padded = move |_| spec.from_complete(1 << spec.data_size());
        let lut = Lut4::from_fn("test", spec, padded, padded, padded, padded);
        let _ = lut.lookup(spec.from_message(0), LookupCheck::AllowOutputPadding);
    }

    #[test]
    #[should_panic(expected = "Encountered incompatible check for many-lut lookup")]
    fn test_lut4_rejects_allow_input_padding_check() {
        let spec = CiphertextBlockSpec(2, 4);
        let lut = Lut4::from_fn("test", spec, |x| x, |x| x, |x| x, |x| x);
        let _ = lut.lookup(spec.from_message(0), LookupCheck::AllowInputPadding);
    }

    #[test]
    #[should_panic(expected = "Encountered incompatible check for many-lut lookup")]
    fn test_lut4_rejects_allow_both_padding_check() {
        let spec = CiphertextBlockSpec(2, 4);
        let lut = Lut4::from_fn("test", spec, |x| x, |x| x, |x| x, |x| x);
        let _ = lut.lookup(spec.from_message(0), LookupCheck::AllowBothPadding);
    }

    #[test]
    fn test_lut8_lookup_returns_all_function_results() {
        let spec = CiphertextBlockSpec(2, 4);
        let offset = |k: u16| {
            move |x: EmulatedCiphertextBlock| {
                x.spec()
                    .from_message((x.raw_message_bits() + k) & x.spec().message_mask())
            }
        };
        let lut = Lut8::from_fn(
            "test",
            spec,
            offset(0),
            offset(1),
            offset(2),
            offset(3),
            offset(4),
            offset(5),
            offset(6),
            offset(7),
        );
        // Valid inputs: data bits 3, 4 and 5 clear, i.e. no carry, message < 8
        for msg in 0..8u16 {
            let outs = lut.lookup(spec.from_message(msg), LookupCheck::Protect);
            let outs = [
                outs.0, outs.1, outs.2, outs.3, outs.4, outs.5, outs.6, outs.7,
            ];
            for (k, out) in outs.iter().enumerate() {
                assert_eq!(
                    out.raw_message_bits(),
                    (msg + k as u16) & spec.message_mask()
                );
            }
        }
    }

    #[test]
    #[should_panic(expected = "Encountered active padding bit in input when executing lookup8.")]
    fn test_lut8_panics_on_input_padding_set() {
        let spec = CiphertextBlockSpec(2, 4);
        let lut = Lut8::from_fn(
            "test",
            spec,
            |x| x,
            |x| x,
            |x| x,
            |x| x,
            |x| x,
            |x| x,
            |x| x,
            |x| x,
        );
        let inp = spec.from_complete(1 << spec.data_size()); // padding bit set
        let _ = lut.lookup(inp, LookupCheck::Protect);
    }

    #[test]
    #[should_panic(expected = "Encountered active many lut bit in input when executing lookup8.")]
    fn test_lut8_panics_on_many_lut_bit_set() {
        let spec = CiphertextBlockSpec(2, 4);
        let lut = Lut8::from_fn(
            "test",
            spec,
            |x| x,
            |x| x,
            |x| x,
            |x| x,
            |x| x,
            |x| x,
            |x| x,
            |x| x,
        );
        let inp = spec.from_data(0b00_1000); // data bit 3 set
        let _ = lut.lookup(inp, LookupCheck::Protect);
    }

    #[test]
    #[should_panic(expected = "Encountered active padding bit in output when executing lookup8.")]
    fn test_lut8_protect_panics_on_output_padding() {
        let spec = CiphertextBlockSpec(2, 4);
        let zero = move |_| spec.from_message(0);
        let padded = move |_| spec.from_complete(1 << spec.data_size());
        let lut = Lut8::from_fn(
            "test", spec, zero, zero, zero, zero, zero, zero, zero, padded,
        );
        let _ = lut.lookup(spec.from_message(0), LookupCheck::Protect);
    }

    #[test]
    fn test_lut8_allow_output_padding_does_not_panic() {
        let spec = CiphertextBlockSpec(2, 4);
        let padded = move |_| spec.from_complete(1 << spec.data_size());
        let lut = Lut8::from_fn(
            "test", spec, padded, padded, padded, padded, padded, padded, padded, padded,
        );
        let _ = lut.lookup(spec.from_message(0), LookupCheck::AllowOutputPadding);
    }

    #[test]
    #[should_panic(expected = "Encountered incompatible check for many-lut lookup")]
    fn test_lut8_rejects_allow_input_padding_check() {
        let spec = CiphertextBlockSpec(2, 4);
        let lut = Lut8::from_fn(
            "test",
            spec,
            |x| x,
            |x| x,
            |x| x,
            |x| x,
            |x| x,
            |x| x,
            |x| x,
            |x| x,
        );
        let _ = lut.lookup(spec.from_message(0), LookupCheck::AllowInputPadding);
    }
}
