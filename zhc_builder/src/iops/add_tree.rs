//! Carry-lookahead tree adder for 2-bit blocks with 2-bit carries.
//!
//! [`Builder::iop_add_tree`] computes the wrapping sum of two encrypted integers
//! with fewer PBS and fewer dependent PBS layers than the Hillis-Steele adder:
//! about 2 PBS per block for wide integers instead of about 4. It is the
//! default of [`Builder::iop_add`] when no carry-in is given.
//!
//! # Cost model
//!
//! The HPU processes PBS in batches of 12 slots that all cost about the same,
//! and linear operations are cheap but serialize on one unit. The latency of a
//! circuit is essentially proportional to its number of batches, which is
//! bounded below by both `ceil(PBS / 12)` (wide integers) and the number of
//! dependent PBS layers (narrow integers). The two designs below minimise both.
//!
//! # Building blocks
//!
//! * Raw sums `s = a + b` per block (value 0..6, noise 2).
//! * Block state 0 none / 1 propagate / 2 generate, extracted from `s` by a
//!   two-output PBS that also returns the message `s & 3`.
//! * Weighted sums: with states at weights 1, 2, 4, 8 the carry out of a group
//!   of four blocks is bit 4 of the sum, which lands on the padding bit, so a
//!   single negacyclic PBS turns the 5-bit sum into the group state (the
//!   `ReduceCarryPad` trick of the Hillis-Steele adder). The output of such a
//!   PBS carries a constant offset that is folded into the next lookup table
//!   instead of being applied with a linear op ([`Val`]).
//! * Along the leftmost path the carry-in is zero: block 0 contributes a single
//!   generate bit, the weights are halved and every carry is a clean 4-bit
//!   lookup one layer earlier.
//! * Finals: the last PBS of a block absorbs its carry from a 2-bit linear
//!   quantity (`4*m + st + c`), so no separate carry-resolution PBS is needed
//!   for three block positions out of four.
//! * Digits: a pair of blocks is a 4-bit digit; `s_lo + 4*s_hi` (value 0..30,
//!   noise 10) has its generate on the padding bit and its propagate at value
//!   15, so ONE negacyclic PBS gives the digit state with no per-block lookup.
//!   The digit is then resolved by a two-step ripple (one two-output PBS per
//!   block). This is the design used above 36 bits.
//!
//! # Shapes
//!
//! A [`Spec`] describes the tree of a size: leaves are groups of blocks with a
//! resolution mode, nodes combine up to four units (plus an optional trailing
//! single block fed by the 5-bit trick). The tables in [`shape`] and
//! [`digit_shape`] were produced by an offline hill-climbing search over
//! shapes with the HPU simulator as the oracle for the default `HpuConfig`;
//! they are data indexed by the integer size and should be regenerated rather
//! than hand-edited if the cost model changes. Sizes outside the tables use
//! the default shapes, which give most of the gain.

use std::cell::RefCell;

use zhc_crypto::integer_semantics::{CiphertextSpec, EmulatedCiphertextBlock as Blk, lut::LookupCheck};
use zhc_langs::ioplang::{Lut1Def, Lut2Def};

use crate::builder::{Builder, Ciphertext, CiphertextBlock};

// ================================================================ lookup tables
//
// LUT functions receive the emulated input block and return the output block.
// `K` is a known offset of the raw input: the true value is `(raw + K) mod 32`.

/// Negation on the complete 5-bit block value.
fn neg5(v: u16) -> u16 {
    (32 - v) & 31
}

/// Table entry for raw index `i < 16` of a function `f` of the TRUE value, defined for
/// true values below 16 with a clean output. Raw inputs with the padding bit set read the
/// negated entry, which is what the negacyclic evaluation expects.
fn shifted<const K: u16>(i: u16, f: impl Fn(u16) -> u16) -> u16 {
    let t = (i + K) & 31;
    if t < 16 { f(t) } else { neg5(f(t - 16)) }
}

/// Block state of the raw sum (2 generate, 1 propagate, 0 none) at weight W.
fn lut_state<const W: u16>(b: Blk) -> Blk {
    let x = b.raw_data_bits();
    let st = if x >= 4 { 2 } else if x == 3 { 1 } else { 0 };
    b.spec().from_complete(st * W)
}
/// Generate bit of the raw sum (zero carry-in) at weight W.
fn lut_gen<const W: u16>(b: Blk) -> Blk {
    b.spec().from_complete(if b.raw_data_bits() >= 4 { W } else { 0 })
}
/// Generate bit of the raw sum as a state {0, 2W} (zero carry-in).
fn lut_gen_block<const W: u16>(b: Blk) -> Blk {
    b.spec().from_complete(if b.raw_data_bits() >= 4 { 2 * W } else { 0 })
}
/// Message of the raw sum.
fn lut_msg(b: Blk) -> Blk {
    b.spec().from_message(b.raw_data_bits() & 3)
}
/// Unit state {0, W, 2W} from a clean weighted sum over LEN members (LEN <= 3).
fn lut_gstate<const W: u16, const LEN: u16, const K: u16>(b: Blk) -> Blk {
    let v = shifted::<K>(b.raw_data_bits(), |x| {
        let st = if x >= (1 << LEN) { 2 } else if x == (1 << LEN) - 1 { 1 } else { 0 };
        st * W
    });
    b.spec().from_complete(v)
}
/// Unit state from a 5-bit weighted sum over 4 members (negacyclic): the raw output is
/// none -> -W, propagate -> 0, generate -> W, i.e. the state minus an offset W.
fn lut_gstate5<const W: u16, const K: u16>(b: Blk) -> Blk {
    let t = (b.raw_data_bits() + K) & 31;
    let v = if t == 15 || t == 31 { 0 } else if t < 16 { neg5(W) } else { W };
    b.spec().from_complete(v)
}
/// Zero-carry-in unit state {0, 2W} from a clean sum with LEN+1 bits.
fn lut_gen_top<const LEN: u16, const W: u16, const K: u16>(b: Blk) -> Blk {
    let v = shifted::<K>(b.raw_data_bits(), |x| if x >= (1 << LEN) { 2 * W } else { 0 });
    b.spec().from_complete(v)
}
/// Zero-carry-in unit state from a 5-bit sum (negacyclic): raw -W below 16, +W above,
/// i.e. {0, 2W} minus an offset W.
fn lut_gen5<const W: u16, const K: u16>(b: Blk) -> Blk {
    let t = (b.raw_data_bits() + K) & 31;
    b.spec().from_complete(if t < 16 { neg5(W) } else { W })
}
/// Clean carry: top bit of a sum with LEN+1 bits.
fn lut_top<const LEN: u16, const K: u16>(b: Blk) -> Blk {
    let v = shifted::<K>(b.raw_data_bits(), |x| if x >= (1 << LEN) { 1 } else { 0 });
    b.spec().from_complete(v)
}
/// Carry at weight 4: top bit of a clean sum with LEN+1 bits.
fn lut_top_x4<const LEN: u16>(b: Blk) -> Blk {
    b.spec().from_data(if b.raw_data_bits() >= (1 << LEN) { 4 } else { 0 })
}
/// Carry of a 5-bit sum (negacyclic): raw -1 without carry, +1 with carry, i.e. twice
/// the carry minus an offset 1.
fn lut_carry5<const K: u16>(b: Blk) -> Blk {
    let t = (b.raw_data_bits() + K) & 31;
    b.spec().from_complete(if t < 16 { 31 } else { 1 })
}
/// x = 8c + s (raw sum s): (s + c) & 3.
fn lut_add8<const K: u16>(b: Blk) -> Blk {
    let v = shifted::<K>(b.raw_data_bits(), |x| ((x & 7) + (x >> 3)) & 3);
    b.spec().from_complete(v)
}
/// x = 4m + v with carry = v >= 2: (m + carry) & 3.
fn lut_hi_plus_carry_lo(b: Blk) -> Blk {
    let x = b.raw_data_bits();
    b.spec().from_message(((x >> 2) + u16::from((x & 3) >= 2)) & 3)
}
/// x = 4v + m with carry = v >= 2: (m + carry) & 3.
fn lut_lo_plus_carry_hi(b: Blk) -> Blk {
    let x = b.raw_data_bits();
    b.spec().from_message(((x & 3) + u16::from((x >> 2) >= 2)) & 3)
}
/// x = 4c + m with c in {0, 1}: (m + c) & 3.
fn lut_lo_plus_hi(b: Blk) -> Blk {
    let x = b.raw_data_bits();
    b.spec().from_message(((x & 3) + (x >> 2)) & 3)
}

fn l1(name: &str, f: fn(Blk) -> Blk) -> Lut1Def {
    Lut1Def::custom(name, f)
}

/// Dispatches a runtime offset (0..16) to a const generic parameter.
macro_rules! with_k {
    ($k:expr, $f:ident $(, $g:tt)*) => {
        match $k {
            0 => $f::<$($g,)* 0>, 1 => $f::<$($g,)* 1>, 2 => $f::<$($g,)* 2>, 3 => $f::<$($g,)* 3>,
            4 => $f::<$($g,)* 4>, 5 => $f::<$($g,)* 5>, 6 => $f::<$($g,)* 6>, 7 => $f::<$($g,)* 7>,
            8 => $f::<$($g,)* 8>, 9 => $f::<$($g,)* 9>, 10 => $f::<$($g,)* 10>, 11 => $f::<$($g,)* 11>,
            12 => $f::<$($g,)* 12>, 13 => $f::<$($g,)* 13>, 14 => $f::<$($g,)* 14>, 15 => $f::<$($g,)* 15>,
            _ => unreachable!("offset {} out of range", $k),
        }
    };
}

// ================================================================ shapes

/// Resolution mode of a leaf group.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    /// Folded finals (block tree): the final PBS absorbs the carry for positions 0, 1, 3;
    /// position 2 has one resolve PBS. Text: no suffix.
    Fold,
    /// Ripple chain: one two-output PBS per block. Text suffix `r`.
    Ripple,
    /// Ripple chain whose carry-in is the previous sibling's ripple carry-out (no carry
    /// lookup). Text suffix `p`.
    Prev,
    /// First leaf only: ripple chain whose carry-out is also the unit's state, so no state
    /// lookup is needed. Text suffix `c`.
    Chain,
}

/// Tree shape: leaves are groups of consecutive blocks, nodes combine units.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Spec {
    Leaf { size: usize, mode: Mode },
    Node(Vec<Spec>),
}

impl Spec {
    pub fn blocks(&self) -> usize {
        match self {
            Spec::Leaf { size, .. } => *size,
            Spec::Node(v) => v.iter().map(Spec::blocks).sum(),
        }
    }

    /// Block-tree default: groups of 4 blocks (last one partial), nodes of 4 units.
    pub fn default_shape(n: usize, ripple: bool) -> Spec {
        let mode = if ripple { Mode::Ripple } else { Mode::Fold };
        let leaves = (0..n).step_by(4).map(|i| Spec::Leaf { size: (n - i).min(4), mode }).collect();
        Self::nest(leaves)
    }

    /// Groups consecutive units by four until a single root remains.
    fn nest(mut units: Vec<Spec>) -> Spec {
        while units.len() > 1 {
            units = units
                .chunks(4)
                .map(|c| if c.len() == 1 { c[0].clone() } else { Spec::Node(c.to_vec()) })
                .collect();
        }
        units.pop().unwrap()
    }

    /// Compact text form, e.g. `[4,2r,[2p,1]]`.
    pub fn to_text(&self) -> String {
        match self {
            Spec::Leaf { size, mode } => {
                let m = match mode { Mode::Fold => "", Mode::Ripple => "r", Mode::Prev => "p", Mode::Chain => "c" };
                format!("{size}{m}")
            }
            Spec::Node(v) => format!("[{}]", v.iter().map(Spec::to_text).collect::<Vec<_>>().join(",")),
        }
    }

    /// Parses the compact text form.
    pub fn parse(s: &str) -> Spec {
        fn inner(cs: &[u8], i: &mut usize) -> Spec {
            if cs[*i] == b'[' {
                *i += 1;
                let mut v = Vec::new();
                loop {
                    v.push(inner(cs, i));
                    if cs[*i] == b',' {
                        *i += 1;
                    } else {
                        assert_eq!(cs[*i], b']');
                        *i += 1;
                        break;
                    }
                }
                Spec::Node(v)
            } else {
                let mut size = 0usize;
                while *i < cs.len() && cs[*i].is_ascii_digit() {
                    size = size * 10 + usize::from(cs[*i] - b'0');
                    *i += 1;
                }
                let mode = match cs.get(*i) {
                    Some(b'r') => Mode::Ripple,
                    Some(b'p') => Mode::Prev,
                    Some(b'c') => Mode::Chain,
                    _ => Mode::Fold,
                };
                if mode != Mode::Fold {
                    *i += 1;
                }
                Spec::Leaf { size, mode }
            }
        }
        let cs = s.as_bytes();
        let mut i = 0;
        let r = inner(cs, &mut i);
        assert_eq!(i, cs.len(), "trailing characters in shape {s}");
        r
    }
}

// ================================================================ values and carries

/// A block whose true value is `(b + off) mod 32`; the offset is folded into the next LUT.
#[derive(Clone)]
struct Val {
    b: CiphertextBlock,
    off: u16,
}

impl Val {
    fn exact(b: CiphertextBlock) -> Val {
        Val { b, off: 0 }
    }
}

/// The carry into a unit.
#[derive(Clone)]
enum Carry {
    Zero,
    /// clean 0 / 1
    Bit(CiphertextBlock),
    /// 0 / 2 with an offset (from the 5-bit trick); only usable by a single block
    Two(Val),
}

struct Ctx<'a> {
    b: &'a Builder,
}

impl<'a> Ctx<'a> {
    fn add_c(&self, x: &CiphertextBlock, c: &Carry) -> CiphertextBlock {
        match c {
            Carry::Zero => x.clone(),
            Carry::Bit(cb) => self.b.block_add(x, cb),
            Carry::Two(_) => panic!("a {{0,2}} carry can only feed a single block"),
        }
    }

    /// Sum of two offset values (wrapping arithmetic when an offset is present).
    fn vadd(&self, x: &Val, y: &Val) -> Val {
        let b = if x.off == 0 && y.off == 0 {
            self.b.block_temper_add(&x.b, &y.b)
        } else {
            self.b.block_wrapping_add(&x.b, &y.b)
        };
        Val { b, off: (x.off + y.off) & 31 }
    }

    fn vadd_c(&self, x: &Val, c: &Carry) -> Val {
        match c {
            Carry::Zero => x.clone(),
            Carry::Bit(cb) => self.vadd(x, &Val::exact(cb.clone())),
            Carry::Two(_) => panic!("a {{0,2}} carry can only feed a single block"),
        }
    }

    /// Single-output lookup on an offset value: clean when exact, negacyclic with a
    /// shifted table otherwise.
    fn lookup_k(&self, x: &Val, name: &str, f: fn(Blk) -> Blk, out_pad: bool) -> CiphertextBlock {
        if x.off == 0 {
            let check = if out_pad { LookupCheck::AllowOutputPadding } else { LookupCheck::Protect };
            self.b.block_lookup_with(&x.b, l1(name, f), check)
        } else {
            self.b.block_wrapping_lookup(&x.b, l1(&format!("{name}_k{}", x.off), f))
        }
    }

    /// Clean carry = top bit of `x`, where true(x) < 2^(len+1).
    fn top(&self, x: &Val, len: usize) -> CiphertextBlock {
        let f = match len {
            1 => with_k!(x.off, lut_top, 1),
            2 => with_k!(x.off, lut_top, 2),
            3 => with_k!(x.off, lut_top, 3),
            _ => unreachable!("top bit of a sum wider than 4 bits"),
        };
        self.lookup_k(x, &format!("top{len}"), f, false)
    }

    /// State {0, W, 2W} of a unit of `len` members from its full weighted sum. Units of four
    /// members use the 5-bit negacyclic table and return the state with an offset W.
    fn unit_state(&self, sum: &Val, len: usize, w: u16) -> Val {
        macro_rules! gs {
            ($w:literal) => {
                match len {
                    1 => Val::exact(self.lookup_k(sum, "gs1", with_k!(sum.off, lut_gstate, $w, 1), $w == 8)),
                    2 => Val::exact(self.lookup_k(sum, "gs2", with_k!(sum.off, lut_gstate, $w, 2), $w == 8)),
                    3 => Val::exact(self.lookup_k(sum, "gs3", with_k!(sum.off, lut_gstate, $w, 3), $w == 8)),
                    4 => Val {
                        b: self.b.block_wrapping_lookup(&sum.b, l1(&format!("gs5_k{}", sum.off), with_k!(sum.off, lut_gstate5, $w))),
                        off: $w,
                    },
                    _ => unreachable!("unit with more than 4 weighted members"),
                }
            };
        }
        match w {
            1 => gs!(1),
            2 => gs!(2),
            4 => gs!(4),
            8 => gs!(8),
            _ => unreachable!(),
        }
    }

    /// Same as [`Ctx::unit_state`] for a unit with zero carry-in: propagate counts as
    /// none, so the state is {0, 2W}.
    fn gen_state(&self, sum: &Val, len: usize, w: u16) -> Val {
        macro_rules! gt {
            ($w:literal) => {
                match len {
                    1 => Val::exact(self.lookup_k(sum, "gt1", with_k!(sum.off, lut_gen_top, 1, $w), $w == 8)),
                    2 => Val::exact(self.lookup_k(sum, "gt2", with_k!(sum.off, lut_gen_top, 2, $w), $w == 8)),
                    3 => Val::exact(self.lookup_k(sum, "gt3", with_k!(sum.off, lut_gen_top, 3, $w), $w == 8)),
                    4 => Val {
                        b: self.b.block_wrapping_lookup(&sum.b, l1(&format!("gen5_k{}", sum.off), with_k!(sum.off, lut_gen5, $w))),
                        off: $w,
                    },
                    _ => unreachable!("unit with more than 4 weighted members"),
                }
            };
        }
        match w {
            1 => gt!(1),
            2 => gt!(2),
            4 => gt!(4),
            8 => gt!(8),
            _ => unreachable!(),
        }
    }

    /// Carry (as {0, 2} with an offset) of a 5-bit sum whose padding bit is the carry.
    fn trick_carry(&self, x: &Val) -> Carry {
        let y = self.b.block_wrapping_lookup(&x.b, l1(&format!("carry5_k{}", x.off), with_k!(x.off, lut_carry5)));
        Carry::Two(Val { b: y, off: 1 })
    }

    /// Final of a block from its raw sum and its carry-in: (s + c) & 3.
    fn final_add8(&self, s: &CiphertextBlock, c: &Carry) -> CiphertextBlock {
        match c {
            Carry::Zero => self.b.block_lookup(s, Lut1Def::MsgOnly),
            Carry::Bit(cb) => self.b.block_lookup(&self.b.block_mac(cb, s, 8), l1("add8", lut_add8::<0>)),
            Carry::Two(v) => {
                // true value = 4 * (y + off) + s = raw + 4 * off
                let x = self.b.block_wrapping_mac(&v.b, s, 4);
                let k = (4 * v.off) & 31;
                self.lookup_k(&Val { b: x, off: k }, "add8", with_k!(k, lut_add8), false)
            }
        }
    }

    /// Ripple resolution of `s` from carry `c`; returns the carry-out when `need_co`.
    fn ripple(&self, s: &[CiphertextBlock], c: &Carry, need_co: bool, out: &mut Vec<CiphertextBlock>) -> Option<CiphertextBlock> {
        let n = s.len();
        let mut carry = c.clone();
        for (j, sj) in s.iter().enumerate() {
            let x = self.add_c(sj, &carry);
            if j + 1 < n || need_co {
                let (m, cn) = self.b.block_lookup2(&x, Lut2Def::ManyCarryMsg);
                out.push(m);
                carry = Carry::Bit(cn);
            } else {
                out.push(self.b.block_lookup(&x, Lut1Def::MsgOnly));
            }
        }
        match (need_co, carry) {
            (true, Carry::Bit(cb)) => Some(cb),
            _ => None,
        }
    }
}

/// A unit of the carry tree (a leaf group or a node).
trait Unit {
    fn len(&self) -> usize;
    /// State {0, W, 2W} at weight `w`, possibly with an offset (digit tree: {0, 2W} for
    /// units with zero carry-in; block tree: not defined for those, see `carry_out`).
    fn state(&self, cx: &Ctx, w: u16) -> Val;
    /// Block tree, zero-carry-in units only: the carry-out as a clean bit.
    fn carry_out(&self, _cx: &Ctx) -> CiphertextBlock {
        unreachable!("carry_out is only defined for zero-carry-in block-tree units")
    }
    fn is_first(&self) -> bool;
    /// Emits the output blocks given the carry into the unit.
    fn resolve(&self, cx: &Ctx, c: &Carry, out: &mut Vec<CiphertextBlock>);
    /// Ripple carry-out, available after `resolve` when requested at construction.
    fn chain_carry_out(&self) -> Option<CiphertextBlock> {
        None
    }
}

// ================================================================ block tree
//
// Leaves are groups of up to 4 blocks with per-block states from level-0 lookups.
// Weights inside a unit: zero-carry-in units use (g0: 1, 1, 2, 4), others (1, 2, 4, 8).

fn weight(first: bool, j: usize) -> u16 {
    if first {
        if j == 0 { 1 } else { 1 << (j - 1) }
    } else {
        1 << j
    }
}

/// Number of weighted members of a node (a zero-carry-in node has a 1-bit first member).
fn weighted_len(first: bool) -> usize {
    if first { 5 } else { 4 }
}

/// Adds a weighted state to a prefix sum (temper when the padding bit may be set).
fn add_w(b: &Builder, pre: &CiphertextBlock, s: &CiphertextBlock, w: u16) -> CiphertextBlock {
    if w == 8 { b.block_temper_add(pre, s) } else { b.block_add(pre, s) }
}

/// A group of 1..=4 blocks resolved by folded finals or by a ripple chain.
struct Group {
    s: Vec<CiphertextBlock>,         // raw sums
    st: Vec<CiphertextBlock>,        // block states at their weights (when needed)
    m: Vec<Option<CiphertextBlock>>, // messages from the level-0 lookups (fold mode)
    pre: Vec<CiphertextBlock>,       // prefix sums of the states
    first: bool,
    ripple: bool,
}

impl Group {
    fn new(cx: &Ctx, s: Vec<CiphertextBlock>, first: bool, ripple: bool, need_state: bool) -> Self {
        let b = cx.b;
        let n = s.len();
        assert!((1..=4).contains(&n), "group of {n} blocks");
        let mut g = Group { s, st: Vec::new(), m: Vec::new(), pre: Vec::new(), first, ripple };
        if ripple && !need_state {
            return g; // no level-0 lookups at all
        }
        for (j, sj) in g.s.iter().enumerate() {
            let w = weight(first, j);
            let is_gen = first && j == 0;
            // messages are needed by folded finals, except for block 0 of a non-first group
            let need_msg = !ripple && (first || j > 0);
            let check = if w == 8 { LookupCheck::AllowOutputPadding } else { LookupCheck::Protect };
            let (stj, mj) = if need_msg {
                let def = match (is_gen, w) {
                    (true, _) => Lut2Def::custom("gen1_msg", [lut_gen::<1>, lut_msg]),
                    (false, 1) => Lut2Def::custom("st1_msg", [lut_state::<1>, lut_msg]),
                    (false, 2) => Lut2Def::custom("st2_msg", [lut_state::<2>, lut_msg]),
                    (false, 4) => Lut2Def::custom("st4_msg", [lut_state::<4>, lut_msg]),
                    (false, 8) => Lut2Def::custom("st8_msg", [lut_state::<8>, lut_msg]),
                    _ => unreachable!(),
                };
                let (a, c) = b.block_lookup2_with(sj, def, check);
                (a, Some(c))
            } else {
                let def = match (is_gen, w) {
                    (true, _) => l1("gen1", lut_gen::<1>),
                    (false, 1) => l1("st1", lut_state::<1>),
                    (false, 2) => l1("st2", lut_state::<2>),
                    (false, 4) => l1("st4", lut_state::<4>),
                    (false, 8) => l1("st8", lut_state::<8>),
                    _ => unreachable!(),
                };
                (b.block_lookup_with(sj, def, check), None)
            };
            g.st.push(stj);
            g.m.push(mj);
        }
        for j in 0..n {
            let p = if j == 0 { g.st[0].clone() } else { add_w(b, &g.pre[j - 1], &g.st[j], weight(first, j)) };
            g.pre.push(p);
        }
        g
    }

    fn msg(&self, j: usize) -> &CiphertextBlock {
        self.m[j].as_ref().expect("message of a fold block")
    }
}

impl Unit for Group {
    fn len(&self) -> usize {
        self.s.len()
    }
    fn is_first(&self) -> bool {
        self.first
    }
    fn state(&self, cx: &Ctx, w: u16) -> Val {
        assert!(!self.first);
        let n = self.len();
        cx.unit_state(&Val::exact(self.pre[n - 1].clone()), n, w)
    }
    fn carry_out(&self, cx: &Ctx) -> CiphertextBlock {
        assert!(self.first);
        let n = self.len();
        // pre[n-1] < 2^n; carry = pre[n-1] >= 2^(n-1)
        if n == 1 { self.pre[0].clone() } else { cx.top(&Val::exact(self.pre[n - 1].clone()), n - 1) }
    }
    fn resolve(&self, cx: &Ctx, c: &Carry, out: &mut Vec<CiphertextBlock>) {
        let b = cx.b;
        let n = self.len();
        if let Carry::Two(_) = c {
            assert!(n == 1, "a {{0,2}} carry can only feed a single block");
            out.push(cx.final_add8(&self.s[0], c));
            return;
        }
        if self.ripple {
            cx.ripple(&self.s, c, false, out);
            return;
        }
        if self.first {
            assert!(matches!(c, Carry::Zero));
            out.push(self.msg(0).clone());
            if n >= 2 {
                // (m1 + g0) & 3
                let x = b.block_mac(self.msg(1), &self.pre[0], 4);
                out.push(b.block_lookup(&x, l1("lo_plus_hi", lut_lo_plus_hi)));
            }
            if n >= 3 {
                // carry into block 2 = (g0 + st1) >= 2
                let x = b.block_mac(self.msg(2), &self.pre[1], 4);
                out.push(b.block_lookup(&x, l1("hi_plus_carry_lo", lut_hi_plus_carry_lo)));
            }
            if n >= 4 {
                // carry into block 3 = (g0 + st1 + 2 st2) >= 4, 3-bit input
                let (_, c3x4) = b.block_lookup2(&self.pre[2], Lut2Def::custom("top2_x1_x4", [lut_top::<2, 0>, lut_top_x4::<2>]));
                let x = b.block_add(&c3x4, self.msg(3));
                out.push(b.block_lookup(&x, l1("lo_plus_hi", lut_lo_plus_hi)));
            }
            return;
        }
        // block 0: (s0 + c) & 3
        out.push(cx.final_add8(&self.s[0], c));
        if n >= 2 {
            // block 1: (4 m1 + st0) + c, carry = low >= 2 (carry-independent part first)
            let t = b.block_mac(self.msg(1), &self.st[0], 4);
            let x = cx.add_c(&t, c);
            out.push(b.block_lookup(&x, l1("hi_plus_carry_lo", lut_hi_plus_carry_lo)));
        }
        if n >= 3 {
            // carry into block 2 (at weights 1 and 4) = (st0 + 2 st1 + c) >= 4, 3-bit input
            let x2 = cx.add_c(&self.pre[1], c);
            let (_, c2x4) = b.block_lookup2(&x2, Lut2Def::custom("top2_x1_x4", [lut_top::<2, 0>, lut_top_x4::<2>]));
            let x = b.block_add(&c2x4, self.msg(2));
            out.push(b.block_lookup(&x, l1("lo_plus_hi", lut_lo_plus_hi)));
            if n >= 4 {
                // block 3: (m3 + 4 st2) + 4 c2, carry = high >= 2
                let t = b.block_add(self.msg(3), &self.st[2]);
                let x = b.block_add(&t, &c2x4);
                out.push(b.block_lookup(&x, l1("lo_plus_carry_hi", lut_lo_plus_carry_hi)));
            }
        }
    }
}

/// A node of up to 4 weighted members (5 with zero carry-in) plus an optional trailing
/// single block fed by the 5-bit trick.
struct Node {
    members: Vec<Box<dyn Unit>>,
    pre: Vec<Val>, // prefix sums (zero carry-in: pre[0] is the clean carry-out of member 0)
    first: bool,
}

impl Node {
    fn new(cx: &Ctx, members: Vec<Box<dyn Unit>>) -> Self {
        let n = members.len();
        let first = members[0].is_first();
        let wl = weighted_len(first);
        assert!(n >= 1 && n <= wl + 1, "node of {n} members");
        if n == wl + 1 {
            assert!(members[wl].len() == 1, "the trailing member of a node must be a single block");
        }
        let mut pre: Vec<Val> = Vec::new();
        for j in 0..(n - 1).min(wl) {
            let sj = if j == 0 && first {
                Val::exact(members[0].carry_out(cx))
            } else {
                members[j].state(cx, weight(first, j))
            };
            let p = if j == 0 { sj } else { cx.vadd(&pre[j - 1], &sj) };
            pre.push(p);
        }
        Node { members, pre, first }
    }

    /// Full weighted sum including the last member (units of at most 4 members).
    fn full_sum(&self, cx: &Ctx) -> Val {
        let n = self.len();
        assert!(n <= 4, "state of a node with more than 4 members");
        let last = if n == 1 && self.first {
            Val::exact(self.members[0].carry_out(cx))
        } else {
            self.members[n - 1].state(cx, weight(self.first, n - 1))
        };
        if n == 1 { last } else { cx.vadd(&self.pre[n - 2], &last) }
    }
}

impl Unit for Node {
    fn len(&self) -> usize {
        self.members.len()
    }
    fn is_first(&self) -> bool {
        self.first
    }
    fn state(&self, cx: &Ctx, w: u16) -> Val {
        assert!(!self.first);
        let n = self.len();
        cx.unit_state(&self.full_sum(cx), n, w)
    }
    fn carry_out(&self, cx: &Ctx) -> CiphertextBlock {
        assert!(self.first);
        let n = self.len();
        let sum = self.full_sum(cx);
        if n == 1 { sum.b } else { cx.top(&sum, n - 1) }
    }
    fn resolve(&self, cx: &Ctx, c: &Carry, out: &mut Vec<CiphertextBlock>) {
        let n = self.len();
        let wl = weighted_len(self.first);
        self.members[0].resolve(cx, c, out);
        for j in 1..n {
            let cj = if j == wl {
                // trailing single block: carry of the 5-bit sum via the trick
                let x = if self.first { self.pre[j - 1].clone() } else { cx.vadd_c(&self.pre[j - 1], c) };
                cx.trick_carry(&x)
            } else if self.first {
                // pre[j-1] < 2^j; carry into member j = pre[j-1] >= 2^(j-1)
                if j == 1 { Carry::Bit(self.pre[0].b.clone()) } else { Carry::Bit(cx.top(&self.pre[j - 1], j - 1)) }
            } else {
                // pre[j-1] + c < 2^(j+1); carry into member j = >= 2^j
                Carry::Bit(cx.top(&cx.vadd_c(&self.pre[j - 1], c), j))
            };
            self.members[j].resolve(cx, &cj, out);
        }
    }
}

fn build_block_tree(cx: &Ctx, spec: &Spec, sums: &[CiphertextBlock], start: usize, need_state: bool) -> Box<dyn Unit> {
    match spec {
        Spec::Leaf { size, mode } => {
            assert!(matches!(mode, Mode::Fold | Mode::Ripple), "block-tree leaf mode");
            Box::new(Group::new(cx, sums[start..start + size].to_vec(), start == 0, *mode == Mode::Ripple, need_state))
        }
        Spec::Node(children) => {
            let last = children.len() - 1;
            let mut pos = start;
            let members = children
                .iter()
                .enumerate()
                .map(|(i, ch)| {
                    let m = build_block_tree(cx, ch, sums, pos, i != last || need_state);
                    pos += ch.blocks();
                    m
                })
                .collect();
            Box::new(Node::new(cx, members))
        }
    }
}

// ================================================================ digit tree
//
// Leaves are 4-bit digits (block pairs) whose state comes from one negacyclic PBS on
// `s_lo + 4 s_hi`; they are resolved by a two-step ripple. Units with zero carry-in have
// a {0, 2W} state at weight 1 (no halved weights are needed).

/// A digit of 1 or 2 blocks.
struct Digit {
    s: Vec<CiphertextBlock>,
    first: bool,
    need_co: bool,
    ripple_co: RefCell<Option<CiphertextBlock>>,
    /// Chain mode (first leaf): outputs computed eagerly, the chain carry-out is the state.
    chain_out: Vec<CiphertextBlock>,
    chain_co: Option<CiphertextBlock>,
}

impl Digit {
    fn new(cx: &Ctx, s: Vec<CiphertextBlock>, first: bool, mode: Mode, need_state: bool, need_co: bool) -> Self {
        let n = s.len();
        assert!(n == 1 || n == 2, "digit of {n} blocks");
        let mut d = Digit { s, first, need_co, ripple_co: RefCell::new(None), chain_out: Vec::new(), chain_co: None };
        if mode == Mode::Chain {
            assert!(first, "chain mode is for the first leaf only");
            let mut out = Vec::new();
            let co = cx.ripple(&d.s, &Carry::Zero, need_state || need_co, &mut out);
            d.chain_out = out;
            d.chain_co = co.clone();
            *d.ripple_co.borrow_mut() = co;
        }
        d
    }
}

impl Unit for Digit {
    fn len(&self) -> usize {
        self.s.len()
    }
    fn is_first(&self) -> bool {
        self.first
    }
    fn chain_carry_out(&self) -> Option<CiphertextBlock> {
        self.ripple_co.borrow().clone()
    }
    fn state(&self, cx: &Ctx, w: u16) -> Val {
        let b = cx.b;
        if let Some(co) = &self.chain_co {
            assert!(w == 1, "a chained first leaf must be member 0");
            return Val::exact(b.block_add(co, co));
        }
        if self.s.len() == 2 {
            // v = s_lo + 4 s_hi <= 30 (padding bit = generate), noise 10
            let v = Val::exact(b.block_temper_mac(&self.s[1], &self.s[0], 4));
            return if self.first { cx.gen_state(&v, 4, w) } else { cx.unit_state(&v, 4, w) };
        }
        let check = if w == 8 { LookupCheck::AllowOutputPadding } else { LookupCheck::Protect };
        let f = match (self.first, w) {
            (true, 1) => lut_gen_block::<1>,
            (true, 2) => lut_gen_block::<2>,
            (true, 4) => lut_gen_block::<4>,
            (true, 8) => lut_gen_block::<8>,
            (false, 1) => lut_state::<1>,
            (false, 2) => lut_state::<2>,
            (false, 4) => lut_state::<4>,
            (false, 8) => lut_state::<8>,
            _ => unreachable!(),
        };
        Val::exact(b.block_lookup_with(&self.s[0], l1(if self.first { "genb" } else { "st" }, f), check))
    }
    fn resolve(&self, cx: &Ctx, c: &Carry, out: &mut Vec<CiphertextBlock>) {
        if !self.chain_out.is_empty() {
            out.extend(self.chain_out.iter().cloned());
            return;
        }
        if let Carry::Two(_) = c {
            assert!(self.s.len() == 1, "a {{0,2}} carry can only feed a single block");
            out.push(cx.final_add8(&self.s[0], c));
            return;
        }
        let co = cx.ripple(&self.s, c, self.need_co, out);
        *self.ripple_co.borrow_mut() = co;
    }
}

/// A block-tree fold group used as a digit-tree leaf (2..=4 blocks, not first).
struct FoldLeaf(Group);

impl Unit for FoldLeaf {
    fn len(&self) -> usize {
        self.0.len()
    }
    fn is_first(&self) -> bool {
        false
    }
    fn state(&self, cx: &Ctx, w: u16) -> Val {
        self.0.state(cx, w)
    }
    fn resolve(&self, cx: &Ctx, c: &Carry, out: &mut Vec<CiphertextBlock>) {
        self.0.resolve(cx, c, out)
    }
}

/// A node of up to 4 digit-tree units at weights 1, 2, 4, 8, plus an optional trailing
/// single block fed by the 5-bit trick. Members in Prev mode take their carry from the
/// previous member's ripple chain.
struct DNode {
    members: Vec<Box<dyn Unit>>,
    pre: Vec<Val>,
    first: bool,
    prev: Vec<bool>,
}

impl DNode {
    fn new(cx: &Ctx, members: Vec<Box<dyn Unit>>, prev: Vec<bool>, first: bool, need_state: bool) -> Self {
        let n = members.len();
        assert!(n >= 1 && n <= 5, "digit node of {n} members");
        if n == 5 {
            assert!(!need_state && members[4].len() == 1 && !prev[4], "the trailing member is a single block fed by the 5-bit trick");
        }
        // pre[j] is needed by the last later member that takes a tree carry, or by the node state
        let last_user = if need_state { n } else { (1..n).rev().find(|&k| !prev[k]).unwrap_or(0) };
        let mut pre: Vec<Val> = Vec::new();
        for j in 0..(n - 1).min(4).min(last_user) {
            let sj = members[j].state(cx, 1 << j);
            let p = if j == 0 { sj } else { cx.vadd(&pre[j - 1], &sj) };
            pre.push(p);
        }
        DNode { members, pre, first, prev }
    }

    fn full_sum(&self, cx: &Ctx) -> Val {
        let n = self.len();
        assert!(n <= 4, "state of a node with more than 4 members");
        let last = self.members[n - 1].state(cx, 1 << (n - 1));
        if n == 1 { last } else { cx.vadd(&self.pre[n - 2], &last) }
    }
}

impl Unit for DNode {
    fn len(&self) -> usize {
        self.members.len()
    }
    fn is_first(&self) -> bool {
        self.first
    }
    fn state(&self, cx: &Ctx, w: u16) -> Val {
        let n = self.len();
        let sum = self.full_sum(cx);
        if self.first { cx.gen_state(&sum, n, w) } else { cx.unit_state(&sum, n, w) }
    }
    fn resolve(&self, cx: &Ctx, c: &Carry, out: &mut Vec<CiphertextBlock>) {
        let n = self.len();
        self.members[0].resolve(cx, c, out);
        for j in 1..n {
            let cj = if self.prev[j] {
                Carry::Bit(self.members[j - 1].chain_carry_out().expect("previous sibling has no chain carry-out"))
            } else if j == 4 {
                cx.trick_carry(&cx.vadd_c(&self.pre[3], c))
            } else {
                // pre[j-1] + c < 2^(j+1); carry into member j = >= 2^j
                Carry::Bit(cx.top(&cx.vadd_c(&self.pre[j - 1], c), j))
            };
            self.members[j].resolve(cx, &cj, out);
        }
    }
}

fn build_digit_tree(cx: &Ctx, spec: &Spec, sums: &[CiphertextBlock], start: usize, need_state: bool, need_co: bool) -> Box<dyn Unit> {
    match spec {
        Spec::Leaf { size, mode: Mode::Fold } if *size >= 2 => {
            assert!(start != 0 && !need_co, "fold leaves of a digit tree are not first and have no chain carry-out");
            Box::new(FoldLeaf(Group::new(cx, sums[start..start + size].to_vec(), false, false, need_state)))
        }
        Spec::Leaf { size, mode } => {
            let mode = if *mode == Mode::Fold { Mode::Ripple } else { *mode };
            Box::new(Digit::new(cx, sums[start..start + size].to_vec(), start == 0, mode, need_state, need_co))
        }
        Spec::Node(children) => {
            assert!(!need_co, "a node has no chain carry-out");
            let is_prev = |c: &Spec| matches!(c, Spec::Leaf { mode: Mode::Prev, .. });
            let last = children.len() - 1;
            let mut pos = start;
            let mut members = Vec::new();
            let mut prev = Vec::new();
            for (i, ch) in children.iter().enumerate() {
                let next_is_prev = i < last && is_prev(&children[i + 1]);
                let child_need_state = need_state || children[i + 1..].iter().any(|c| !is_prev(c));
                members.push(build_digit_tree(cx, ch, sums, pos, child_need_state, next_is_prev));
                prev.push(i > 0 && is_prev(ch));
                pos += ch.blocks();
            }
            Box::new(DNode::new(cx, members, prev, start == 0, need_state))
        }
    }
}

// ================================================================ entry points

/// Digit-tree default: pairs (last single if odd), nodes of 4 units.
pub fn default_digit_shape(n: usize) -> Spec {
    let leaves = (0..n).step_by(2).map(|i| Spec::Leaf { size: (n - i).min(2), mode: Mode::Ripple }).collect();
    Spec::nest(leaves)
}

/// Block-tree shape of a size (table entry or default; used when [`digit_shape`] is `None`).
pub fn shape(bits: u16) -> Spec {
    let n = usize::from(bits / 2);
    let text = match bits {
        20 => "[4,2r,2,2]",
        24 => "[4,2r,2,2,2]",
        26 => "[4,3,2,2,2]",
        28 => "[4,4,2,2,2]",
        34 => "[[3,3,2],3,1,2,2,1]",
        36 => "[[[1,4,4,3],2r,2],2]",
        _ => return Spec::default_shape(n, false),
    };
    Spec::parse(text)
}

/// Digit-tree shape of a size (table entry, or the default above 36 bits), or `None` for
/// the sizes where the block tree is faster.
pub fn digit_shape(bits: u16) -> Option<Spec> {
    let n = usize::from(bits / 2);
    let text = match bits {
        4 | 64 => return Some(default_digit_shape(n)),
        14 => "[2p,2p,1r,1r,1r]",
        18 => "[2r,2p,2r,2r,1r]",
        22 => "[[2p,2p,2r,2r],1r,1r,1r]",
        30 => "[2p,2p,2r,[2r,2r,2r,[2,1r]]]",
        32 => "[[2r,2r,2r,2r],[2r,2,2,2]]",
        38 => "[[2p,2p,2p,2r],[[2r,2r,2r,2r],[2r,1r]]]",
        40 => "[[2p,2p,2p,2r],[2p,2r,2r,2r],2r,2r]",
        42 => "[[2r,2p,2r,2r],2r,2r,[2r,2r,2r,2r,1r]]",
        44 => "[[2p,2p,2p,2r],[2p,1p,1r],[2r,2r,2r,2r],2r]",
        46 => "[[2p,2p,2p,2r],[2r,2r,1r,1r],[2r,2r,2r,2r,1r]]",
        48 => "[[2r,2p,2p,2r],[2p,2r,2r,2r],[[2r,2r,2r],2r]]",
        50 => "[[2r,2p,2r,2p],[2r,2r,2r,2r],[[2r,2r,2r,2r],1r]]",
        52 => "[[2r,2p,2p,2r],[2p,2r,2r,2r],[2r,2r,2r,2r],2r]",
        54 => "[[2r,2p,2p,2r],[2r,2r,2r,2r],[2,2r,2r,2r],2r,1r]",
        56 => "[[2r,2p,2r,2r],[2r,2r,2r,2r],[2r,2r,2r],[2r,1r,1r,2r]]",
        58 => "[[2r,2p,2r,2p],[2p,2r,2r,2r],[2r,2r,2r],[2r,1r,2r,2r]]",
        60 => "[[2r,2p,2r,2r],[2p,2r,2r],[2r,2r,2r,2r],[2r,2r,2r,1r,1r]]",
        62 => "[[2r,2r,2r,2r],[2r,2r,2r,2r],[2r,2r,2r,2r],[[2r,2r,2r],1r]]",
        66 => "[[2p,2r,2r,2r],[2r,2r,2r,2r],[[2r,2r,2r],2r],[[2r,2r,2r],2r],1r]",
        68 => "[[[2r,2r,2r,2r],[2r,2r,2r,2r],[2r,2r,2r,2r],[2r,2r,2r,2r]],[1r,1r]]",
        70 => "[[[2p,2p,2p,2r],[2p,2p,2r,2r],[2p,2p,2r,2r],[2p,2r,2r,2r]],2,1r]",
        72 => "[[[2r,2p,2p,2r],[2r,2r,2r,2r],[2p,2r,2r,2r],[2p,2r,2r,2r]],2,2r]",
        74 => "[[2r,2p,2r,2r],[2r,2r,2r,2r],[2r,2p,[2p,2r],[[[2r,2r,2r,2r],2r,2r],1r]]]",
        76 => "[[[2r,2r,2r,2r],[2r,2r,2r,2r],[[2r,2r,2r],2r,2r,2r],[2r,2r]],2r,2r,2r]",
        78 => "[[[2r,2r,2r,2r],[2p,2r,2r,2r],[2p,2r,2r,2r],[2r,2r,2r,2r]],[[2p,2r],1r,1r,1r]]",
        80 => "[[[2r,2r,2r,2r],[2r,2r,2r,2r],[2r,2r,2r,2p],[2r,2r,2r,2r]],[2r,2r,2r,2r]]",
        82 => "[[[2r,2p,2r,2r],[2p,2r,2r,2r],[2p,2p,2r,2p],[2r,2r,2r,2]],[2r,2r,2r,2r],1r]",
        84 => "[[[[2p,2p,2p],[2p,2r,2r,2r],[2p,2r,[1p,2p,1p],2r],[2r,2r,2r,2r]],[2r,2r,2r,2r]],2r]",
        86 => "[[[2r,2r,2r,2r],[2r,2r,2r,2r],[2r,2r,2r,2r],[2r,2r,2r,2r]],[[2r,2p,2r,2r],[2r,1p]]]",
        88 => "[[[2r,[2r,1p,1p],2r],[2r,2r,2p,2r],[2r,2r,2p,2r],[2r,2p,2r,2r]],[[2r,2r,2r],2r],2r,2r]",
        90 => "[[2r,2r,2r,2r],[2r,2r,2r,2r],[2p,2r,2r,2r],[[2r,2r,2p,2r],[2p,2p,2r,2r],[2r,2p,1r]]]",
        92 => "[[[2r,2r,2r,2r],[2p,2p,2r,2p],[2r,2r,2r,2r]],[[2r,2r,[2r,2r]],[2r,2r,2r,2r],[2r,2r,2r]]]",
        94 => "[[[2p,2r,2p,2r],[2r,2r,2r,2r],[2r,2r,2r,2r],[2p,2p,2r,2r]],[2p,2p,2r,2r],[2r,1r,2r,2r]]",
        96 => "[[[[2r,2p,2p,2r],[2r,2r,2r,2r]],[2r,2r,2r,2p],[2r,2r,2r,2r],2r],[2r,2p,2r],[2r,2,2r,2r]]",
        98 => "[[[[2p,2p,2r,2r],[2r,2r,2r,2r]],[2r,2p,2r,2r],[2p,2p,2r,2r]],[2p,2p,2r,2r],[2r,2r,2r,2r,1r]]",
        100 => "[[2r,2r,2r,2p],[2r,2p,2r],[2r,2r,2r,2r],[[2r,[2p,2p,2r,2r],[2r,2p,2r,2r],1r],[1p,[2r,2r,2r],2r]]]",
        102 => "[[[2r,2r,2p,2r],[2p,2p,2r,2p],[2r,2p,2p,2r],[2p,2r,2r,2p]],[2r,2p,2r,2r],[2r,2r],[2r,2r,[2r,1p]]]",
        104 => "[[[2r,2r,2r,2p],[2r,2r,2r,2r],[2r,2p,2r,2r]],[[[2r,2r,2r],2r],[2r,2r,2r,2r],[2r,2r,2r,2r],[2r,1p,1p]]]",
        106 => "[[[2r,2r,2r,2p],[2p,2r,2r,2r],[2r,2r,2p,2r],[2p,2r,2r,2p]],[2r,2r,2r,2r],[[2r,[2r,2r]],[2r,2p,2r,1p]]]",
        108 => "[[[2p,2p,2r,2r],[2r,2r,2r,2r],[2p,2r,2p,2r],[2p,2p,2r,2r]],[2p,2r,2p,2r],[[1r,1p,2r,2r],2r],[2p,[2p,2r]]]",
        110 => "[[[2r,2p,2r,2r],[2r,2r,2r,2r],[2p,2r,2r,2r],[2p,2r,2r,2r]],[2r,2r,2r,2r],[2r,2r,2r,2r],[[2p,2r],2r,1r]]",
        112 => "[[[2r,2p,2r,2r],[2r,2r,2p,2r],[2r,2r,2r,2p],[2r,2r,2r,2r]],[2r,2r,2r,2r],[2r,2r,2r,2r],[[[2r,2r],2r],2r]]",
        114 => "[[[[2p,2r,2r,2r],[2p,2r,2r,2r],[2p,2r,2r,2r],[2r,2r,2r,2r]],[[2p,2r,2r],2r],[2r,2r,2r,2r]],[2p,2r,2r,2r],1r]",
        116 => "[[[2p,2r,2r,2r],[2r,2r,2r,2r],[2r,2r,2r,2r],[2r,2r,2r,2r]],[[[[2r,2r,2r,2p],2r],[2r,2r,2r,2r]],[2r,2r,2r],2r]]",
        118 => "[[[2p,2p,2r,2r],[2r,2r,2p,2p],[2p,2r,2r,2r],[2p,2r,2p,2r]],[2r,[2p,2r,2p,2r],[2r,2p,2r]],[[2p,2r,2r],[2p,2p,1r]]]",
        120 => "[[[2r,2r,2r,2r],[2r,2r,2r,2r],[2r,2r,2r,2r],[2r,2r,2p,2r]],[[2r,2p,2r,2r],[2p,2r,2r,2r],[2r,2r,2r,2p],[2r,2p]]]",
        122 => "[[[2c,2r,2r,2r],[2p,2r,2r,2r],[2r,2p,2r,2r],[2r,2p,2p,2r]],[[2r,2r,2r,2r],[2r,2r,2r,2p],[2r,2r,2r],[2r,2r,2r,1r]]]",
        124 => "[[[2c,2p,2r,2p],[2r,2p,2r,2r],[[2r,2p,2r,2p],[2r,2p,2r,2r]]],[[[2r,2r,2r],[2r,2r,2r,2r]],[2r,2r,2r,2p],[2p,2r,[2r,1p,1p]]]]",
        126 => "[[[2r,2p,2r,2r],[2p,2p,2r,2r],[2p,2p,2r,2r],[2r,2p,2r,2r]],[[2p,2p,2r,2r],[2r,2r,2r,2p],[2r,2p,2r,2p]],2r,[2r,[2r,1p]]]",
        _ => return if bits > 36 { Some(default_digit_shape(n)) } else { None },
    };
    Some(Spec::parse(text))
}

impl Builder {
    /// Adds two block vectors of equal length with the carry-lookahead tree adder and returns
    /// the blocks of the wrapping sum (fresh, message only). Requires 2-bit messages and 2-bit
    /// carries. No carry-in and no carry-out: see [`Builder::iop_add_raw`] for those.
    pub fn iop_add_tree_raw(
        &self,
        lhs_blocks: impl AsRef<[CiphertextBlock]>,
        rhs_blocks: impl AsRef<[CiphertextBlock]>,
    ) -> Vec<CiphertextBlock> {
        let n = lhs_blocks.as_ref().len();
        let bits = u16::try_from(2 * n).expect("integer size");
        match digit_shape(bits) {
            Some(ds) => self.iop_add_shape_raw(lhs_blocks, rhs_blocks, &ds, true),
            None => self.iop_add_shape_raw(lhs_blocks, rhs_blocks, &shape(bits), false),
        }
    }

    /// Same as [`Builder::iop_add_tree_raw`] with an explicit tree shape (`digit` selects the
    /// digit tree, otherwise the block tree). Used to evaluate and regenerate the shape tables.
    pub fn iop_add_shape_raw(
        &self,
        lhs_blocks: impl AsRef<[CiphertextBlock]>,
        rhs_blocks: impl AsRef<[CiphertextBlock]>,
        spec: &Spec,
        digit: bool,
    ) -> Vec<CiphertextBlock> {
        let (lhs, rhs) = (lhs_blocks.as_ref(), rhs_blocks.as_ref());
        assert_eq!(lhs.len(), rhs.len(), "the tree adder needs operands of equal size");
        assert!(
            self.spec().carry_size() == 2 && self.spec().message_size() == 2,
            "the tree adder needs 2-bit messages and 2-bit carries"
        );
        assert_eq!(spec.blocks(), lhs.len(), "shape {} does not cover {} blocks", spec.to_text(), lhs.len());
        let sums: Vec<CiphertextBlock> = lhs.iter().zip(rhs).map(|(x, y)| self.block_add(x, y)).collect();
        let cx = Ctx { b: self };
        let root = if digit {
            build_digit_tree(&cx, spec, &sums, 0, false, false)
        } else {
            build_block_tree(&cx, spec, &sums, 0, false)
        };
        let mut out = Vec::with_capacity(sums.len());
        root.resolve(&cx, &Carry::Zero, &mut out);
        out
    }

    /// Adds two encrypted integers of the same size with the carry-lookahead tree adder.
    ///
    /// Returns the wrapping sum. This is the algorithm behind [`Builder::iop_add`] when no
    /// carry-in is given; see the [module documentation](self) for the design.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_builder::{CiphertextSpec, Builder};
    /// # let spec = CiphertextSpec::new(64, 2, 2);
    /// # let builder = Builder::new(spec.block_spec());
    /// # let a = builder.ciphertext_input(spec.int_size());
    /// # let b = builder.ciphertext_input(spec.int_size());
    /// let sum = builder.iop_add_tree(&a, &b);
    /// ```
    pub fn iop_add_tree(&self, lhs: &Ciphertext, rhs: &Ciphertext) -> Ciphertext {
        let lhs_blocks = self.ciphertext_split(lhs);
        let rhs_blocks = self.ciphertext_split(rhs);
        let out = self.iop_add_tree_raw(lhs_blocks, rhs_blocks);
        self.comment("Join Output").ciphertext_join(out, None)
    }
}

/// Creates an IR for addition with the carry-lookahead tree adder.
///
/// Convenience wrapper that declares inputs/outputs and calls [`Builder::iop_add_tree`].
pub fn add_tree(spec: CiphertextSpec) -> Builder {
    let builder = Builder::new(spec.block_spec());
    let src_a = builder.ciphertext_input(spec.int_size());
    let src_b = builder.ciphertext_input(spec.int_size());
    let res = builder.iop_add_tree(&src_a, &src_b);
    builder.ciphertext_output(res);
    builder
}

#[cfg(test)]
mod test {
    use super::*;
    use zhc_langs::ioplang::IopValue;

    fn semantic(inp: &[IopValue]) -> Option<Vec<IopValue>> {
        let [IopValue::Ciphertext(lhs), IopValue::Ciphertext(rhs)] = inp else {
            unreachable!()
        };
        Some(vec![IopValue::Ciphertext(lhs.add(*rhs))])
    }

    #[test]
    fn correctness_add_tree() {
        for size in (2..=128).step_by(2) {
            add_tree(CiphertextSpec::new(size, 2, 2)).test_random(100, semantic);
        }
    }

    #[test]
    fn noise_add_tree() {
        for size in (2..=128).step_by(2) {
            add_tree(CiphertextSpec::new(size, 2, 2)).check_noise();
        }
    }

    /// Random inputs almost never produce long propagate chains: exercise them explicitly.
    #[test]
    fn carry_chains_add_tree() {
        for size in (2..=128u16).step_by(2) {
            let spec = CiphertextSpec::new(size, 2, 2);
            let bd = add_tree(spec);
            let n = u32::from(size / 2);
            let mask: u128 = if size == 128 { u128::MAX } else { (1u128 << size) - 1 };
            let p5: u128 = (0..n).fold(0, |acc, k| acc | (1u128 << (2 * k)));
            let pa: u128 = (0..n).fold(0, |acc, k| acc | (2u128 << (2 * k)));
            let mut cases = vec![(0, 0), (mask, 1), (1, mask), (mask, mask), (mask >> 1, mask >> 1), (p5, pa), (pa, pa)];
            for j in (0..n).step_by(3) {
                let shift = 2 * j;
                // generate at block j, propagate above
                let above = mask & !((1u128 << (shift + 2)) - 1);
                cases.push((above | (2u128 << shift), 2u128 << shift));
                // propagate everywhere, carry-in at block j
                cases.push((mask, 1u128 << shift));
                // full carry chain from block 0 to block j
                cases.push(((1u128 << (shift + 2)) - 1, 1));
            }
            for (a, b) in cases {
                let (ea, eb) = (spec.from_int(a), spec.from_int(b));
                let expected = vec![IopValue::Ciphertext(ea.add(eb))];
                let inputs = vec![IopValue::Ciphertext(ea), IopValue::Ciphertext(eb)];
                let outputs = bd.interpret().with_inputs(&inputs).get_outputs();
                assert_eq!(outputs, expected, "size {size} a={a:#x} b={b:#x}");
            }
        }
    }

    #[test]
    fn shape_text_roundtrip() {
        for text in ["[4,2r,2,2]", "[[3,3,2],3,1,2,2,1]", "[[2c,2p,2r,2r],[2r,2r,2r],1r]"] {
            assert_eq!(Spec::parse(text).to_text(), text);
        }
        for bits in (2..=126u16).step_by(2) {
            let n = usize::from(bits / 2);
            assert_eq!(shape(bits).blocks(), n);
            if let Some(d) = digit_shape(bits) {
                assert_eq!(d.blocks(), n);
            }
        }
    }
}
