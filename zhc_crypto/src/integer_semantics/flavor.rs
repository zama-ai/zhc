/// Overflow policy of a linear block operation.
///
/// Every linear operation on [`EmulatedCiphertextBlock`](super::EmulatedCiphertextBlock)
/// (addition, subtraction, multiplication by a plaintext, shifts, packing) comes in three
/// flavors that differ in how they treat the padding bit:
///
/// - [`Protect`](Self::Protect): operand padding bits must be zero, and the result must fit in the
///   data region (carry | message). The padding bit is never written. This is the default flavor
///   and the one required before a non-negacyclic PBS.
/// - [`Temper`](Self::Temper): operand padding bits may be arbitrary, and the result may set the
///   padding bit, but it must not overflow past it. Useful before a negacyclic PBS.
/// - [`Wrapping`](Self::Wrapping): no check at all. The result is reduced modulo the complete block
///   width, like Rust's `wrapping_*` integer operations.
///
/// See the [module documentation](super) for the full discussion.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Flavor {
    /// Padding bit must be clear on operands and is never written.
    Protect,
    /// Padding bit may be written, but must not overflow.
    Temper,
    /// Modular arithmetic on the complete block width.
    Wrapping,
}

impl Flavor {
    /// Returns the textual prefix used to name operations of this flavor.
    ///
    /// `Protect` is the implicit default, so it has an empty prefix. `Temper` and `Wrapping`
    /// return `"temper_"` and `"wrapping_"`.
    pub fn prefix(&self) -> &'static str {
        match self {
            Flavor::Protect => "",
            Flavor::Temper => "temper_",
            Flavor::Wrapping => "wrapping_",
        }
    }
}
