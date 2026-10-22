//! Combinational equivalence checking of Boolean circuits with ABC.
//!
//! A [`Circuit`] is built incrementally from primary inputs, gates, and primary outputs. Every
//! Boolean signal of a circuit is a [`Lit`]. The two primitive gates are [`not`](Circuit::not)
//! and [`and`](Circuit::and); [`or`](Circuit::or), [`xor`](Circuit::xor) and
//! [`mux`](Circuit::mux) are conveniences expressed in terms of them. Signals that are built the
//! same way compare equal, which lets a caller detect trivially identical logic without a check.
//!
//! [`check_equivalence`] decides whether two circuits with the same number of inputs and
//! outputs compute the same Boolean functions, output by output, over all input assignments.
//! The answer is a [`Verdict`]: proven equal, proven different with a [`Counterexample`], or
//! undecided because a resource bound in [`CheckOptions`] was reached. Structural mismatches
//! between the two circuits are reported as a [`CheckError`] before any checking starts.
//!
//! Literals are only meaningful inside the circuit that produced them. Circuits are
//! independent: each one may be built and checked on its own thread, and checks on distinct
//! circuits may run concurrently. This relies on ABC keeping all state inside the circuit
//! objects it allocates, which is how its sources are written but not a documented guarantee
//! of the ABC project.

use std::ptr::NonNull;
use std::time::Duration;

use zhc_abc_sys as sys;

/// A Boolean signal of a [`Circuit`].
///
/// Equal literals compute the same function of the inputs. The converse does not hold: two
/// different literals may compute the same function, and deciding that is the job of
/// [`check_equivalence`]. A literal is produced by the gate methods of a [`Circuit`] and must
/// only be used with the circuit that created it. This is not checked, and passing a literal to
/// another circuit has unspecified effects, up to a crash.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Lit(u32);

impl Lit {
    /// The constant false literal.
    pub const FALSE: Lit = Lit(0);
    /// The constant true literal.
    pub const TRUE: Lit = Lit(1);

    /// Returns whether the literal is [`FALSE`](Self::FALSE) or [`TRUE`](Self::TRUE).
    pub fn is_constant(self) -> bool {
        self.0 < 2
    }
}

/// A combinational Boolean circuit.
///
/// Inputs and outputs are numbered by order of creation, and these positions are what
/// [`check_equivalence`] uses to pair the inputs and outputs of two circuits. Storage grows on
/// demand. ABC terminates the process with exit code 1 when the number of inputs and AND gates
/// of one circuit reaches 2^29; this is not a panic and cannot be caught.
///
/// A `Circuit` is `Send` and not `Sync`.
pub struct Circuit {
    man: NonNull<sys::Gia_Man_t>,
    n_inputs: usize,
    n_outputs: usize,
}

unsafe impl Send for Circuit {}

impl Default for Circuit {
    fn default() -> Self {
        Self::new()
    }
}

impl Circuit {
    /// Creates an empty circuit with room for 2^16 inputs and AND gates before the first growth.
    ///
    /// # Panics
    ///
    /// Panics if ABC fails to allocate the circuit.
    pub fn new() -> Self {
        Self::with_capacity(1 << 16)
    }

    /// Creates an empty circuit with room for about `n_gates` inputs and AND gates before the
    /// first growth.
    ///
    /// The value is a reservation hint only. It is clamped to the range `16..=i32::MAX`, and
    /// the circuit grows past it on demand.
    ///
    /// # Panics
    ///
    /// Panics if ABC fails to allocate the circuit.
    pub fn with_capacity(n_gates: usize) -> Self {
        let cap = n_gates.clamp(16, i32::MAX as usize) as i32;
        let man = unsafe { sys::zhc_abc_gia_start(cap) };
        Self {
            man: NonNull::new(man).expect("ABC failed to allocate a manager"),
            n_inputs: 0,
            n_outputs: 0,
        }
    }

    /// Returns the constant literal with the given value.
    ///
    /// The result is [`Lit::TRUE`] for `true` and [`Lit::FALSE`] for `false`. It creates no
    /// gate and never changes [`n_ands`](Self::n_ands).
    pub fn constant(&self, value: bool) -> Lit {
        if value { Lit::TRUE } else { Lit::FALSE }
    }

    /// Appends a primary input and returns its positive literal.
    ///
    /// The input takes position [`n_inputs`](Self::n_inputs) as it was before the call. Every
    /// call creates a new variable, never a constant.
    pub fn input(&mut self) -> Lit {
        self.n_inputs += 1;
        Lit(unsafe { sys::zhc_abc_gia_append_ci(self.man.as_ptr()) } as u32)
    }

    /// Returns the literal of `NOT a`.
    ///
    /// Negation is free and deterministic: `not(not(a)) == a`, `not(a) != a`, [`Lit::FALSE`]
    /// and [`Lit::TRUE`] are negations of each other, and [`n_ands`](Self::n_ands) never
    /// changes.
    pub fn not(&self, a: Lit) -> Lit {
        Lit(a.0 ^ 1)
    }

    /// Returns the literal of `a AND b`.
    ///
    /// The result is folded when it is determined by the operands: it is [`Lit::FALSE`] when
    /// either operand is `FALSE` or when `a` is the negation of `b`, it is the other operand
    /// when one operand is [`Lit::TRUE`], and it is `a` when both operands are equal. Otherwise
    /// the result is a new AND gate, except that the same unordered pair of operands always
    /// yields the same literal: operand order does not matter and asking twice creates nothing.
    /// Only the creation of a new gate increases [`n_ands`](Self::n_ands).
    pub fn and(&mut self, a: Lit, b: Lit) -> Lit {
        Lit(unsafe { sys::zhc_abc_gia_hash_and(self.man.as_ptr(), a.0 as i32, b.0 as i32) } as u32)
    }

    /// Returns the literal of `a OR b`.
    ///
    /// Built as the negation of [`and`](Self::and) applied to the negated operands, so the same
    /// folding and sharing rules apply with the polarities inverted.
    pub fn or(&mut self, a: Lit, b: Lit) -> Lit {
        let na = self.not(a);
        let nb = self.not(b);
        let n = self.and(na, nb);
        self.not(n)
    }

    /// Returns the literal of `a XOR b`.
    ///
    /// Built from [`and`](Self::and) and [`or`](Self::or), so constant operands fold through
    /// those rules. Equal or complementary operands are not recognised as a unit; the result is
    /// correct but may be a gate output instead of a constant.
    pub fn xor(&mut self, a: Lit, b: Lit) -> Lit {
        let na = self.not(a);
        let nb = self.not(b);
        let t = self.and(a, nb);
        let u = self.and(na, b);
        self.or(t, u)
    }

    /// Returns the literal of `on_true` when `sel` is true and of `on_false` otherwise.
    ///
    /// Built from [`and`](Self::and) and [`or`](Self::or), so constant operands fold through
    /// those rules. Equal branches are not recognised as a unit; the result is correct but may
    /// be a gate output instead of the shared branch.
    pub fn mux(&mut self, sel: Lit, on_true: Lit, on_false: Lit) -> Lit {
        let nsel = self.not(sel);
        let t = self.and(sel, on_true);
        let f = self.and(nsel, on_false);
        self.or(t, f)
    }

    /// Appends a primary output driven by `lit`.
    ///
    /// The output takes position [`n_outputs`](Self::n_outputs) as it was before the call. Any
    /// literal of this circuit is accepted, including constants and literals already used by
    /// another output.
    pub fn output(&mut self, lit: Lit) {
        self.n_outputs += 1;
        unsafe { sys::zhc_abc_gia_append_co(self.man.as_ptr(), lit.0 as i32) };
    }

    /// Returns the number of primary inputs.
    pub fn n_inputs(&self) -> usize {
        self.n_inputs
    }

    /// Returns the number of primary outputs.
    pub fn n_outputs(&self) -> usize {
        self.n_outputs
    }

    /// Returns the number of AND gates.
    ///
    /// Inputs, outputs, constants, negations, and folded or shared results of
    /// [`and`](Self::and) do not count.
    pub fn n_ands(&self) -> usize {
        unsafe { sys::zhc_abc_gia_and_num(self.man.as_ptr()) as usize }
    }
}

impl std::fmt::Display for Circuit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let p = self.man.as_ptr();
        let lit = |l: i32| -> String {
            let var = l >> 1;
            let neg = l & 1 == 1;
            match (var, neg) {
                (0, false) => "false".to_string(),
                (0, true) => "true".to_string(),
                (v, false) => format!("n{v}"),
                (v, true) => format!("!n{v}"),
            }
        };
        let n_objs = unsafe { sys::zhc_abc_gia_obj_num(p) };
        for id in 0..n_objs {
            match unsafe { sys::zhc_abc_gia_obj_kind(p, id) } {
                1 => {
                    let k = unsafe { sys::zhc_abc_gia_obj_cio_id(p, id) };
                    writeln!(f, "n{id} = input {k}")?;
                }
                2 => {
                    let a = unsafe { sys::zhc_abc_gia_obj_fanin0_lit(p, id) };
                    let b = unsafe { sys::zhc_abc_gia_obj_fanin1_lit(p, id) };
                    writeln!(f, "n{id} = and({}, {})", lit(a), lit(b))?;
                }
                3 => {
                    let k = unsafe { sys::zhc_abc_gia_obj_cio_id(p, id) };
                    let a = unsafe { sys::zhc_abc_gia_obj_fanin0_lit(p, id) };
                    writeln!(f, "output {k} = {}", lit(a))?;
                }
                _ => {}
            }
        }
        Ok(())
    }
}

impl Drop for Circuit {
    fn drop(&mut self) {
        unsafe { sys::zhc_abc_gia_stop(self.man.as_ptr()) };
    }
}

/// Resource bounds and reporting options of [`check_equivalence`].
///
/// The default is a conflict limit of 10 000, no time limit, and no output, which matches the
/// defaults of ABC's `cec` command.
#[derive(Clone, Debug)]
pub struct CheckOptions {
    /// Maximum number of SAT conflicts spent on each pair of signals suspected to be equivalent.
    ///
    /// Exceeding it on some pair may leave the check [`Undecided`](Verdict::Undecided). Values
    /// above `i32::MAX` are clamped.
    pub conflict_limit: u32,
    /// Overall time budget of the check.
    ///
    /// `None` means no limit. The duration is truncated to whole seconds and raised to at least
    /// one second. When the budget runs out the check returns
    /// [`Undecided`](Verdict::Undecided). The budget is checked between phases, so the actual
    /// run time may exceed it.
    pub time_limit: Option<Duration>,
    /// Whether ABC prints progress and statistics to standard output.
    pub verbose: bool,
}

impl Default for CheckOptions {
    fn default() -> Self {
        Self {
            conflict_limit: 10_000,
            time_limit: None,
            verbose: false,
        }
    }
}

/// An input assignment on which two circuits disagree.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Counterexample {
    /// The value of each primary input, indexed by input position.
    ///
    /// The length equals the number of inputs of the checked circuits.
    pub inputs: Vec<bool>,
    /// The position of one output whose value differs between the two circuits under
    /// [`inputs`](Self::inputs).
    ///
    /// Other outputs may differ as well.
    pub output: usize,
}

/// The outcome of [`check_equivalence`].
///
/// `Equivalent` means every pair of outputs at the same position computes the same function of
/// the inputs. `NotEquivalent` carries one witness assignment. `Undecided` means a bound of
/// [`CheckOptions`] was reached before a proof or a witness was found.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Verdict {
    Equivalent,
    NotEquivalent(Counterexample),
    Undecided,
}

/// A reason why [`check_equivalence`] could not compare two circuits.
///
/// The mismatch variants carry the counts of `left` and `right`. `MiterFailed` is reported when
/// ABC refuses to combine two circuits whose counts match.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum CheckError {
    InputCountMismatch { left: usize, right: usize },
    OutputCountMismatch { left: usize, right: usize },
    MiterFailed,
}

impl std::fmt::Display for CheckError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InputCountMismatch { left, right } => {
                write!(f, "input count mismatch: {left} vs {right}")
            }
            Self::OutputCountMismatch { left, right } => {
                write!(f, "output count mismatch: {left} vs {right}")
            }
            Self::MiterFailed => write!(f, "ABC failed to build the miter"),
        }
    }
}

impl std::error::Error for CheckError {}

/// Decides whether `left` and `right` compute the same functions.
///
/// Input `i` of `left` is identified with input `i` of `right`, and output `i` of `left` is
/// compared with output `i` of `right`, for every input assignment. Outputs driven by constants
/// take part like any other. Two circuits without outputs are equivalent. Neither circuit is
/// modified. The input count is compared first, then the output count, and a mismatch returns
/// the corresponding error without running ABC. The bounds in `options` decide between a
/// definite [`Verdict`] and [`Verdict::Undecided`].
///
/// # Errors
///
/// Returns [`CheckError::MiterFailed`] when ABC cannot combine the two circuits even though
/// their input and output counts match.
pub fn check_equivalence(
    left: &Circuit,
    right: &Circuit,
    options: &CheckOptions,
) -> Result<Verdict, CheckError> {
    if left.n_inputs != right.n_inputs {
        return Err(CheckError::InputCountMismatch {
            left: left.n_inputs,
            right: right.n_inputs,
        });
    }
    if left.n_outputs != right.n_outputs {
        return Err(CheckError::OutputCountMismatch {
            left: left.n_outputs,
            right: right.n_outputs,
        });
    }
    let time_limit = options
        .time_limit
        .map(|d| d.as_secs().clamp(1, i32::MAX as u64) as i32)
        .unwrap_or(0);
    let mut out_fail: i32 = -1;
    let mut cex = vec![0u8; left.n_inputs];
    let ret = unsafe {
        sys::zhc_abc_cec_verify(
            left.man.as_ptr(),
            right.man.as_ptr(),
            options.conflict_limit.min(i32::MAX as u32) as i32,
            time_limit,
            options.verbose as i32,
            &mut out_fail,
            cex.as_mut_ptr(),
            cex.len() as i32,
        )
    };
    match ret {
        1 => Ok(Verdict::Equivalent),
        0 => Ok(Verdict::NotEquivalent(Counterexample {
            inputs: cex.into_iter().map(|b| b != 0).collect(),
            output: out_fail.max(0) as usize,
        })),
        -2 => Err(CheckError::MiterFailed),
        _ => Ok(Verdict::Undecided),
    }
}
