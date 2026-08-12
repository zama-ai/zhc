//! Bit-level AES circuits: the S-box and xtime.
//!
//! The S-box uses 58 PBS and xtime uses 7 PBS.
//! Generic `iop_match_value` tree uses 184 PBS for each of them.
//!
//! These circuits are cheaper because XOR and NOT are linear operations.
//! A [`Bit`] holds a sum that the code does not reduce. The next lookup reads
//! the parity of that sum. Only the non-linear gates and the cleaning steps use a PBS.
//!
//! The [`Bit`] algebra in the first part of this file is not specific to AES.
//! Maybe we should move it to its own module if a second bit-level circuit needs it.
//!
//! `iop_match_value` recognizes the two tables and calls these circuits.
//! The caller continues to supply a table. No code above zhc changes.
//!
//! TOREVIEW: there is an open question here about NU. We know it is set to a pessimistic value, we
//! could recompute it at zhc compile, leading to more perf. Here I know we can ignore, marked w/
//! keyword "TOREVIEW" might need to revert.

use crate::{Builder, Ciphertext, CiphertextBlock, NU_BOOL};
use zhc_langs::ioplang::{Lut1Def, Lut2Def};
use zhc_utils::SafeAs;

#[rustfmt::skip]
/// The AES S-box, in the table format of `iop_match_value`.
pub const AES_SBOX: [u8; 256] = [
    0x63,0x7c,0x77,0x7b,0xf2,0x6b,0x6f,0xc5,0x30,0x01,0x67,0x2b,0xfe,0xd7,0xab,0x76,
    0xca,0x82,0xc9,0x7d,0xfa,0x59,0x47,0xf0,0xad,0xd4,0xa2,0xaf,0x9c,0xa4,0x72,0xc0,
    0xb7,0xfd,0x93,0x26,0x36,0x3f,0xf7,0xcc,0x34,0xa5,0xe5,0xf1,0x71,0xd8,0x31,0x15,
    0x04,0xc7,0x23,0xc3,0x18,0x96,0x05,0x9a,0x07,0x12,0x80,0xe2,0xeb,0x27,0xb2,0x75,
    0x09,0x83,0x2c,0x1a,0x1b,0x6e,0x5a,0xa0,0x52,0x3b,0xd6,0xb3,0x29,0xe3,0x2f,0x84,
    0x53,0xd1,0x00,0xed,0x20,0xfc,0xb1,0x5b,0x6a,0xcb,0xbe,0x39,0x4a,0x4c,0x58,0xcf,
    0xd0,0xef,0xaa,0xfb,0x43,0x4d,0x33,0x85,0x45,0xf9,0x02,0x7f,0x50,0x3c,0x9f,0xa8,
    0x51,0xa3,0x40,0x8f,0x92,0x9d,0x38,0xf5,0xbc,0xb6,0xda,0x21,0x10,0xff,0xf3,0xd2,
    0xcd,0x0c,0x13,0xec,0x5f,0x97,0x44,0x17,0xc4,0xa7,0x7e,0x3d,0x64,0x5d,0x19,0x73,
    0x60,0x81,0x4f,0xdc,0x22,0x2a,0x90,0x88,0x46,0xee,0xb8,0x14,0xde,0x5e,0x0b,0xdb,
    0xe0,0x32,0x3a,0x0a,0x49,0x06,0x24,0x5c,0xc2,0xd3,0xac,0x62,0x91,0x95,0xe4,0x79,
    0xe7,0xc8,0x37,0x6d,0x8d,0xd5,0x4e,0xa9,0x6c,0x56,0xf4,0xea,0x65,0x7a,0xae,0x08,
    0xba,0x78,0x25,0x2e,0x1c,0xa6,0xb4,0xc6,0xe8,0xdd,0x74,0x1f,0x4b,0xbd,0x8b,0x8a,
    0x70,0x3e,0xb5,0x66,0x48,0x03,0xf6,0x0e,0x61,0x35,0x57,0xb9,0x86,0xc1,0x1d,0x9e,
    0xe1,0xf8,0x98,0x11,0x69,0xd9,0x8e,0x94,0x9b,0x1e,0x87,0xe9,0xce,0x55,0x28,0xdf,
    0x8c,0xa1,0x89,0x0d,0xbf,0xe6,0x42,0x68,0x41,0x99,0x2d,0x0f,0xb0,0x54,0xbb,0x16,
];

/// The PBS count of one `iop_sbox`. The count does not change with the
/// data. A test holds this value.
///
/// The count has these parts: 4 for the split, 9 for the norm ANDs, 8 for
/// the y reductions, 9 for the inverter, 18 for the digit ANDs, 8 for the
/// output parities and 2 for the sums that become too wide.
pub const SBOX_PBS: usize = 58;

/// The maximum number of terms in a sum.
const MAX_SUM: u8 = NU_BOOL as u8;

/// The maximum value of `(ta + 1) * (tb + 1)` for a mixed-radix pack `(tb + 1) * a + b`.
/// The pack has a value and a noise of `(ta + 1) * (tb + 1) - 1`.
/// Noise budget thus gives this limit.
const MAX_PACK_SPACE: u16 = NU_BOOL as u16 + 1;

/// An encrypted bit. The block holds a sum of `terms` one-bit values. The
/// logical bit is the parity of that sum. If `inv` is true, invert it.
#[derive(Clone, Copy)]
pub struct Bit {
    block: CiphertextBlock,
    terms: u8,
    inv: bool,
}

impl Bit {
    /// Makes a clean bit. The block holds 0 or 1, and nothing is pending.
    fn clean(block: CiphertextBlock) -> Self {
        Bit {
            block,
            terms: 1,
            inv: false,
        }
    }
}

/// A bit inside a 2-bit digit. An inverter lookup makes these digits.
///
/// The next lookup reads `Lo`, `Hi` or their parity `Xor` directly from the
/// digit. Therefore the code does not split the digit.
#[derive(Clone, Copy)]
struct DigitBit {
    digit: CiphertextBlock,
    sel: DigitSel,
}

#[derive(Clone, Copy)]
enum DigitSel {
    Lo,
    Hi,
    Xor,
}

impl DigitSel {
    fn read(self, digit: u8) -> u8 {
        match self {
            DigitSel::Lo => digit & 1,
            DigitSel::Hi => (digit >> 1) & 1,
            DigitSel::Xor => (digit & 1) ^ ((digit >> 1) & 1),
        }
    }
}

impl Builder {
    fn bit_table(&self, name: &str, f: impl Fn(u8) -> u8) -> Lut1Def {
        Lut1Def::Table {
            name: name.to_string(),
            table: (0..1u8 << self.spec().data_size().sas::<u32>())
                .map(f)
                .collect(),
        }
    }

    /// Reduces a pending sum to a clean bit. Uses one PBS.
    pub fn bit_reduce(&self, a: Bit) -> Bit {
        if a.terms == 1 && !a.inv {
            return a;
        }
        let inv = a.inv as u8;
        let lut = self.bit_table("BitParity", |v| (v & 1) ^ inv);
        Bit::clean(self.block_lookup(&a.block, lut))
    }

    /// Adds two bits. The operation is linear and uses no PBS.
    /// The consumer of the sum reduces it later.
    /// reduces an operand only if the sum becomes larger than the noise budget.
    pub fn bit_xor(&self, a: Bit, b: Bit) -> Bit {
        let (mut a, mut b) = (a, b);
        while a.terms + b.terms > MAX_SUM {
            if a.terms >= b.terms {
                a = self.bit_reduce(a);
            } else {
                b = self.bit_reduce(b);
            }
        }
        Bit {
            block: self.block_add(&a.block, &b.block),
            terms: a.terms + b.terms,
            inv: a.inv ^ b.inv,
        }
    }

    /// Inverts a bit. The next lookup applies the inversion, so this
    /// function uses no PBS.
    pub fn bit_not(&self, a: Bit) -> Bit {
        Bit { inv: !a.inv, ..a }
    }

    /// Computes the AND of two bits. Uses one PBS. The lookup applies the
    /// parity and the inversion of each operand.
    ///
    /// The pack is mixed-radix: `(tb + 1) * a + b`. Each pair of sums stays
    /// different if `(ta + 1) * (tb + 1)` is not larger than the pack space.
    /// A clean bit thus fits beside a sum of 3 terms. The function reduces
    /// the operands until the pair fits.
    pub fn bit_and(&self, a: Bit, b: Bit) -> Bit {
        let (mut a, mut b) = (a, b);
        while (u16::from(a.terms) + 1) * (u16::from(b.terms) + 1) > MAX_PACK_SPACE {
            if a.terms >= b.terms {
                a = self.bit_reduce(a);
            } else {
                b = self.bit_reduce(b);
            }
        }
        let radix = b.terms + 1;
        let (ia, ib) = (a.inv as u8, b.inv as u8);
        let lut = self.bit_table("BitAnd", move |v| {
            (((v / radix) & 1) ^ ia) & (((v % radix) & 1) ^ ib)
        });
        Bit::clean(self.block_lookup(self.block_mac(&a.block, &b.block, radix), lut))
    }

    /// Computes the AND of a digit bit and a sum bit. Uses one PBS.
    ///
    /// The digit uses two bits of the pack. Therefore the sum can have 3
    /// terms at a maximum. The pack `(ty + 1) * d + y` then fills the 4-bit
    /// data space.
    ///
    /// NOTE: NU is not taken into account here as it is safe. The pack has a
    /// * noise of `(ty + 1) + ty`, which is 7 at a maximum.
    /// TOREVIEW: This adds 3 PBS to each S-box decreasing greatly perf.
    fn bit_and_digit(&self, t: DigitBit, y: Bit) -> Bit {
        let mut y = y;
        // while u16::from(y.terms) > (crate::NU as u16 - 1) / 2 {
        while y.terms > 3 {
            y = self.bit_reduce(y);
        }
        let radix = y.terms + 1;
        let (sel, iy) = (t.sel, y.inv as u8);
        let lut = self.bit_table("BitAndDigit", move |v| {
            sel.read(v / radix) & (((v % radix) & 1) ^ iy)
        });
        Bit::clean(self.block_lookup(self.block_mac(&t.digit, &y.block, radix), lut))
    }

    /// Computes the inversion at the centre of the S-box. Uses three table
    /// lookups.
    ///
    /// The middle section of the Boyar-Peralta circuit contains 5 of the 32
    /// AND gates. This section is a function of four bits only. Therefore
    /// the code does not evaluate its gates. It reduces the four bits, packs
    /// them into one block, and reads the nine values that the second
    /// multiplier layer needs.
    ///
    /// One PBS gives a 2-bit message. Each lookup thus returns a digit that
    /// holds two of the nine values. The parity of each digit gives one more
    /// value: t42 = t29 ^ t33, t41 = t37 ^ t40 and t45 = t43 ^ t44.
    fn sbox_inverter(&self, t: [Bit; 4]) -> [DigitBit; 9] {
        let r = t.map(|t| self.bit_reduce(t));
        // The lookup input is P = 8*t24 + 4*t23 + 2*t22 + t21. A direct
        // pack of the four clean bits has a noise of 8+4+2+1 = 15. Thus the
        // code first packs each pair into a new digit, which has a noise of
        // 2*1+1 = 3. The pack P = 4*Dhi + Dlo then has a noise of 4+1 = 5.
        let compress = |hi: &Bit, lo: &Bit, name: &str| {
            let lut = self.bit_table(name, |v| v & 3);
            self.block_lookup(&self.block_mac(&hi.block, &lo.block, 2), lut)
        };
        let d_lo = compress(&r[1], &r[0], "SboxInvPackLo");
        let d_hi = compress(&r[3], &r[2], "SboxInvPackHi");
        let p = self.block_mac(&d_hi, &d_lo, 4);
        let digit = |name: &str, lo: usize, hi: usize| {
            let lut = self.bit_table(name, move |v| {
                let t = sbox_inverter_clear(std::array::from_fn(|i| (v >> i) & 1 == 1));
                u8::from(t[hi]) << 1 | u8::from(t[lo])
            });
            self.block_lookup(&p, lut)
        };
        let d0 = digit("SboxInvD0", 0, 1); // (t29, t33), xor = t42
        let d1 = digit("SboxInvD1", 2, 3); // (t37, t40), xor = t41
        let d2 = digit("SboxInvD2", 6, 7); // (t43, t44), xor = t45
        let bit = |digit, sel| DigitBit { digit, sel };
        [
            bit(d0, DigitSel::Lo),  // t29
            bit(d0, DigitSel::Hi),  // t33
            bit(d1, DigitSel::Lo),  // t37
            bit(d1, DigitSel::Hi),  // t40
            bit(d1, DigitSel::Xor), // t41
            bit(d0, DigitSel::Xor), // t42
            bit(d2, DigitSel::Lo),  // t43
            bit(d2, DigitSel::Hi),  // t44
            bit(d2, DigitSel::Xor), // t45
        ]
    }

    /// Splits an integer into bits. The first bit is the least significant.
    /// Uses one many-LUT PBS for each block. Each block must hold a clean
    /// message.
    pub fn bits_split(&self, src: &Ciphertext) -> Vec<Bit> {
        self.ciphertext_split(src)
            .iter()
            .flat_map(|block| {
                let (lsb, msb) = self.block_lookup2(block, Lut2Def::ManyMsgSplit);
                [Bit::clean(lsb), Bit::clean(msb)]
            })
            .collect()
    }

    /// Records `bits` as the bit form of `ct`. The first bit is the least
    /// significant. A later bit-level operation uses these bits and does not
    /// split `ct`.
    pub(crate) fn register_bits(&self, ct: &Ciphertext, bits: &[Bit]) {
        self.bit_form_set(ct, bits.iter().map(|b| (b.block, b.terms, b.inv)).collect());
    }

    /// Gives the recorded bit form of `ct`, if it exists. The first bit is
    /// the least significant.
    pub(crate) fn registered_bits(&self, ct: &Ciphertext) -> Option<Vec<Bit>> {
        self.bit_form_get(ct).map(|bits| {
            bits.into_iter()
                .map(|(block, terms, inv)| Bit { block, terms, inv })
                .collect()
        })
    }

    /// Gives the bits of `src`. Uses the recorded bit form if it exists.
    /// That form can contain pending sums. If it does not exist, this
    /// function splits `src`.
    pub(crate) fn bits_of(&self, src: &Ciphertext) -> Vec<Bit> {
        self.registered_bits(src)
            .unwrap_or_else(|| self.bits_split(src))
    }

    /// Joins bits into an integer. The first bit is the least significant.
    ///
    /// Each bit that is not clean needs one PBS. The pack itself is linear.
    /// If the two bits of a block are both pending, and if the pair fits the
    /// pack space, one lookup gives the 2-bit digit. This replaces two
    /// reductions.
    pub fn bits_join(&self, bits: &[Bit], int_size: u16) -> Ciphertext {
        let dirty = |b: &Bit| b.terms > 1 || b.inv;
        let blocks = bits
            .chunks(2)
            .map(|pair| {
                let lsb = pair[0];
                let Some(&msb) = pair.get(1) else {
                    return self.bit_reduce(lsb).block;
                };
                if dirty(&lsb)
                    && dirty(&msb)
                    && (u16::from(lsb.terms) + 1) * (u16::from(msb.terms) + 1) <= MAX_PACK_SPACE
                {
                    let radix = lsb.terms + 1;
                    let (im, il) = (msb.inv as u8, lsb.inv as u8);
                    let lut = self.bit_table("BitJoinDigit", move |v| {
                        ((((v / radix) & 1) ^ im) << 1) | (((v % radix) & 1) ^ il)
                    });
                    return self.block_lookup(self.block_mac(&msb.block, &lsb.block, radix), lut);
                }
                let lsb = self.bit_reduce(lsb).block;
                let msb = self.bit_reduce(msb).block;
                let msb = self.block_add(&msb, &msb);
                self.block_add(&lsb, &msb)
            })
            .collect::<Vec<_>>();
        self.ciphertext_join(blocks, Some(int_size))
    }

    /// Computes the AES S-box of an 8-bit ciphertext.
    pub fn iop_sbox(&self, src: &Ciphertext) -> Ciphertext {
        assert_eq!(
            src.spec().int_size(),
            8,
            "The AES S-box takes an 8-bit ciphertext."
        );
        self.push_comment("sbox");
        // Recorded bits can be pending sums, for example the output of
        // MixColumns. The packs in the schedule need clean inputs. Thus the
        // code reduces the bits here. Bits that come from a split are
        // already clean, and this step then uses no PBS.
        let bits: Vec<Bit> = self
            .bits_of(src)
            .into_iter()
            .map(|b| self.bit_reduce(b))
            .collect();
        let out = sbox_schedule(
            std::array::from_fn(|i| bits[7 - i]),
            &mut |a, b| self.bit_xor(a, b),
            &mut |a, b| self.bit_and(a, b),
            &mut |a| self.bit_not(a),
            &mut |a| self.bit_reduce(a),
            &mut |t| self.sbox_inverter(t),
            &mut |d, y| self.bit_and_digit(d, y),
        );
        // The output bits are sums. MixColumns adds several of them, and
        // the result would become too wide. Thus the code reduces them here.
        // The join needs the same reductions, so this step adds no PBS.
        let lsb_first: Vec<Bit> = out.iter().rev().map(|bit| self.bit_reduce(*bit)).collect();
        let joined = self.bits_join(&lsb_first, 8);
        self.register_bits(&joined, &lsb_first);
        self.pop_comment();
        joined
    }
    /// Multiplies by two in GF(2^8), as AES MixColumns needs.
    ///
    /// The shift only changes the position of each bit. The constant 0x1B is
    /// in the clear. Therefore only the join of the byte uses a PBS.
    pub fn iop_xtime(&self, src: &Ciphertext) -> Ciphertext {
        assert_eq!(src.spec().int_size(), 8, "xtime takes an 8-bit ciphertext.");
        self.push_comment("xtime");
        let b = self.bits_of(src);
        let top = b[7];
        // The result is (src << 1) ^ 0x1B if the top bit is 1. The constant
        // 0x1B has bits 0, 1, 3 and 4 set. The shift puts a 0 in bit 0.
        // Therefore bit 0 of the result is the top bit.
        let mut out = [top, b[0], b[1], b[2], b[3], b[4], b[5], b[6]];
        for i in [1, 3, 4] {
            out[i] = self.bit_xor(out[i], top);
        }
        let joined = self.bits_join(&out, 8);
        // The code records the bits with their pending sums. A bit-level
        // operation then adds them at no cost. If no operation reads the
        // byte, dead code elimination removes the join.
        self.register_bits(&joined, &out);
        self.pop_comment();
        joined
    }
}

/// Computes the middle section of the Boyar-Peralta circuit on clear bits.
///
/// The function maps the four bits (t21, t22, t23, t24) to the nine values
/// that the second multiplier layer needs. The order of the results is
/// (t29, t33, t37, t40, t41, t42, t43, t44, t45).
fn sbox_inverter_clear(b: [bool; 4]) -> [bool; 9] {
    let (b21, b22, b23, b24) = (b[0], b[1], b[2], b[3]);
    let t25 = b21 ^ b22;
    let t26 = b21 & b23;
    let t27 = b24 ^ t26;
    let t28 = t25 & t27;
    let t29 = t28 ^ b22;
    let t30 = b23 ^ b24;
    let t31 = b22 ^ t26;
    let t32 = t31 & t30;
    let t33 = t32 ^ b24;
    let t34 = b23 ^ t33;
    let t35 = t27 ^ t33;
    let t36 = b24 & t35;
    let t37 = t36 ^ t34;
    let t38 = t27 ^ t36;
    let t39 = t29 & t38;
    let t40 = t25 ^ t39;
    let t41 = t40 ^ t37;
    let t42 = t29 ^ t33;
    let t43 = t29 ^ t40;
    let t44 = t33 ^ t37;
    let t45 = t42 ^ t41;
    [t29, t33, t37, t40, t41, t42, t43, t44, t45]
}

/// Computes the AES S-box with the Boyar-Peralta circuit.
///
/// The caller supplies the bit operations. Thus a test can run this schedule
/// on clear bits, and `iop_sbox` can run it on encrypted bits. The first bit
/// of `x` is the most significant.
///
/// The schedule is different from the published circuit in three points.
/// Each difference decreases the PBS count:
/// * The schedule calls `reduce` at specified positions. It cleans y12, y11 and y5 before it makes
///   the values that use them. Each later sum then has 2 to 4 terms, and each operand of the second
///   multiplier layer has 3 terms at a maximum. On clear bits, `reduce` does nothing.
/// * The schedule makes the wide values y17 to y21 after the first multiplier layer. These values
///   then have 2 or 3 terms.
/// * The schedule calls `inverter` for the middle section. On encrypted bits,
///   `Builder::sbox_inverter` does three lookups on one packed block. On clear bits,
///   `sbox_inverter_clear` computes the gates.
fn sbox_schedule<B: Copy, D: Copy>(
    x: [B; 8],
    xor: &mut impl FnMut(B, B) -> B,
    and: &mut impl FnMut(B, B) -> B,
    not: &mut impl FnMut(B) -> B,
    reduce: &mut impl FnMut(B) -> B,
    inverter: &mut impl FnMut([B; 4]) -> [D; 9],
    and_digit: &mut impl FnMut(D, B) -> B,
) -> [B; 8] {
    let (u0, u1, u2, u3, u4, u5, u6, u7) = (x[0], x[1], x[2], x[3], x[4], x[5], x[6], x[7]);

    // The first linear layer, with reductions between the operations. The
    // code cleans y12, y11 and y5 before it makes the values that use them.
    // Each later sum then has 2 to 4 terms, as the noise budget requires.
    // The values y15, y3 and y1 thus reach their AND gate without a
    // reduction.
    let y14 = xor(u3, u5);
    let y13 = xor(u0, u6);
    let y9 = xor(u0, u3);
    let y8 = xor(u0, u5);
    let t0 = xor(u1, u2);
    let y1 = xor(t0, u7);
    let y4 = xor(y1, u3);
    let y12 = xor(y13, y14);
    let y2 = xor(y1, u0);
    let y5 = xor(y1, u6);
    let y12 = reduce(y12);
    let t1 = xor(u4, y12);
    let y15 = xor(t1, u5);
    let y20 = xor(t1, u1);
    let y6 = xor(y15, u7);
    let y10 = xor(y15, t0);
    let y11 = xor(y20, y9);
    let y11 = reduce(y11);
    let y7 = xor(u7, y11);
    let y16 = xor(t0, y11);
    let y5 = reduce(y5);
    let y3 = xor(y5, y8);

    // The first multiplier layer: 9 AND gates. The code reduces the operands
    // that are still wide. The second multiplier layer uses the same clean
    // values.
    let t2 = and(y12, y15);
    let y6 = reduce(y6);
    let t3 = and(y3, y6);
    let t4 = xor(t3, t2);
    let y4 = reduce(y4);
    let t5 = and(y4, u7);
    let t6 = xor(t5, t2);
    let y16 = reduce(y16);
    let t7 = and(y13, y16);
    let t8 = and(y5, y1);
    let t9 = xor(t8, t7);
    let y2 = reduce(y2);
    let t10 = and(y2, y7);
    let t11 = xor(t10, t7);
    let t12 = and(y9, y11);
    let y10 = reduce(y10);
    let t15 = and(y8, y10);
    let y17 = xor(y10, y11);
    let t13 = and(y14, y17);
    let t14 = xor(t13, t12);
    let t16 = xor(t15, t12);

    // The four input bits of the inverter. The late y values are narrow
    // here.
    let t17 = xor(t4, t14);
    let t18 = xor(t6, t16);
    let t19 = xor(t9, t14);
    let t20 = xor(t11, t16);
    let t21 = xor(t17, y20);
    let y19 = xor(y10, y8);
    let t22 = xor(t18, y19);
    let y21 = xor(y13, y16);
    let t23 = xor(t19, y21);
    let y18 = xor(u0, y16);
    let t24 = xor(t20, y18);

    let [t29, t33, t37, t40, t41, t42, t43, t44, t45] = inverter([t21, t22, t23, t24]);

    // The second multiplier layer: 18 AND gates. Each gate uses one digit
    // bit. Each y operand has 3 terms at a maximum, which the digit pack
    // permits.
    let z0 = and_digit(t44, y15);
    let z1 = and_digit(t37, y6);
    let z2 = and_digit(t33, u7);
    let z3 = and_digit(t43, y16);
    let z4 = and_digit(t40, y1);
    let z5 = and_digit(t29, y7);
    let z6 = and_digit(t42, y11);
    let z7 = and_digit(t45, y17);
    let z8 = and_digit(t41, y10);
    let z9 = and_digit(t44, y12);
    let z10 = and_digit(t37, y3);
    let z11 = and_digit(t33, y4);
    let z12 = and_digit(t43, y13);
    let z13 = and_digit(t40, y5);
    let z14 = and_digit(t29, y2);
    let z15 = and_digit(t42, y9);
    let z16 = and_digit(t45, y14);
    let z17 = and_digit(t41, y8);

    // The last linear layer.
    let t46 = xor(z15, z16);
    let t47 = xor(z10, z11);
    let t48 = xor(z5, z13);
    let t49 = xor(z9, z10);
    let t50 = xor(z2, z12);
    let t51 = xor(z2, z5);
    let t52 = xor(z7, z8);
    let t53 = xor(z0, z3);
    let t54 = xor(z6, z7);
    let t55 = xor(z16, z17);
    let t56 = xor(z12, t48);
    let t57 = xor(t50, t53);
    let t58 = xor(z4, t46);
    let t59 = xor(z3, t54);
    let t60 = xor(t46, t57);
    let t61 = xor(z14, t57);
    let t62 = xor(t52, t58);
    let t63 = xor(t49, t58);
    let t64 = xor(z4, t59);
    let t65 = xor(t61, t62);
    let t66 = xor(z1, t63);
    let s0 = xor(t59, t63);
    let s6 = not(xor(t56, t62));
    let s7 = not(xor(t48, t60));
    let t67 = xor(t64, t65);
    let s3 = xor(t53, t66);
    let s4 = xor(t51, t66);
    let s5 = xor(t47, t65);
    let s1 = not(xor(t64, s3));
    let s2 = not(xor(t55, t67));

    [s0, s1, s2, s3, s4, s5, s6, s7]
}

/// Multiplies a clear byte by two in GF(2^8). A circuit that supplies xtime
/// as a table gives these values to `iop_match_value`.
pub fn clear_xtime(x: u8) -> u8 {
    (x << 1) ^ if x & 0x80 != 0 { 0x1B } else { 0 }
}

/// Tests if `table` is the full 8-bit table of `f`. If it is, then
/// `iop_match_value` can use a manual circuit in place of the table.
fn is_dense_table(table: &[(u128, u128)], f: impl Fn(u8) -> u8) -> bool {
    let mut seen = [false; 256];
    table.len() == 256
        && table.iter().all(|&(k, v)| {
            k < 256
                && v == u128::from(f(k as u8))
                && !std::mem::replace(&mut seen[k as usize], true)
        })
}

pub(crate) fn is_aes_sbox(table: &[(u128, u128)]) -> bool {
    is_dense_table(table, |x| AES_SBOX[x as usize])
}

pub(crate) fn is_xtime(table: &[(u128, u128)]) -> bool {
    is_dense_table(table, clear_xtime)
}

/// Makes a circuit that applies the S-box to one 8-bit input.
pub fn sbox(spec: zhc_crypto::integer_semantics::CiphertextSpec) -> Builder {
    let builder = Builder::new(spec.block_spec());
    let src = builder.ciphertext_input(spec.int_size());
    let out = builder.iop_sbox(&src);
    builder.ciphertext_output(out);
    builder
}

#[cfg(test)]
mod test {
    use super::*;
    use zhc_crypto::integer_semantics::CiphertextSpec;
    use zhc_langs::ioplang::{IopInstructionSet, IopValue};

    /// Runs the schedule on clear bits. A failure thus shows an error in the
    /// gates, and not in the encrypted bit algebra. On clear bits, `reduce`
    /// returns its input.
    fn clear_sbox(byte: u8) -> u8 {
        let bits: [bool; 8] = std::array::from_fn(|i| (byte >> (7 - i)) & 1 == 1);
        let out = sbox_schedule(
            bits,
            &mut |a, b| a ^ b,
            &mut |a, b| a & b,
            &mut |a| !a,
            &mut |a| a,
            &mut sbox_inverter_clear,
            &mut |a, b| a & b,
        );
        out.iter().fold(0u8, |acc, b| (acc << 1) | *b as u8)
    }

    fn count_pbs(builder: &Builder) -> usize {
        builder
            .optimize_ir()
            .walk_ops_linear()
            .filter(|op| {
                matches!(
                    op.get_instruction(),
                    IopInstructionSet::Pbs { .. } | IopInstructionSet::Pbs2 { .. }
                )
            })
            .count()
    }

    #[test]
    fn netlist_matches_sbox_table() {
        for x in 0..=255u8 {
            assert_eq!(clear_sbox(x), AES_SBOX[x as usize], "netlist at {x:#04x}");
        }
    }

    #[test]
    fn correctness_sbox() {
        let spec = CiphertextSpec::new(8, 2, 2);
        let builder = sbox(spec);
        for x in 0..=255u8 {
            let outputs = builder
                .interpret()
                .with_inputs([IopValue::Ciphertext(spec.from_int(x.into()))])
                .get_outputs();
            assert_eq!(
                outputs,
                vec![IopValue::Ciphertext(
                    spec.from_int(AES_SBOX[x as usize].into())
                )],
                "sbox({x:#04x})"
            );
        }
    }

    /// Tests the bypass in `iop_match_value`. The AES table must give the
    /// bit-level circuit. The result and the flag must be correct.
    #[test]
    fn match_value_uses_bitsliced_sbox() {
        let spec = CiphertextSpec::new(8, 2, 2);
        let table: Vec<(u128, u128)> = (0..256u128)
            .map(|i| (i, AES_SBOX[i as usize].into()))
            .collect();
        let builder = Builder::new(spec.block_spec());
        let src = builder.ciphertext_input(spec.int_size());
        let (out, flag) = builder.iop_match_value(&src, &table, 8);
        builder.ciphertext_output(out);
        builder.ciphertext_output(flag);
        assert_eq!(count_pbs(&builder), SBOX_PBS, "should route to iop_sbox");

        for x in [0u8, 1, 0x53, 0xc6, 0xff] {
            let outputs = builder
                .interpret()
                .with_inputs([IopValue::Ciphertext(spec.from_int(x.into()))])
                .get_outputs();
            let flag_spec = CiphertextSpec::new(2, 2, 2);
            assert_eq!(
                outputs,
                vec![
                    IopValue::Ciphertext(spec.from_int(AES_SBOX[x as usize].into())),
                    IopValue::Ciphertext(flag_spec.from_int(1)),
                ],
                "match_value sbox({x:#04x})"
            );
        }
    }

    #[test]
    fn correctness_xtime() {
        let spec = CiphertextSpec::new(8, 2, 2);
        let builder = Builder::new(spec.block_spec());
        let src = builder.ciphertext_input(spec.int_size());
        let out = builder.iop_xtime(&src);
        builder.ciphertext_output(out);
        for x in 0..=255u8 {
            let outputs = builder
                .interpret()
                .with_inputs([IopValue::Ciphertext(spec.from_int(x.into()))])
                .get_outputs();
            assert_eq!(
                outputs,
                vec![IopValue::Ciphertext(spec.from_int(clear_xtime(x).into()))],
                "xtime({x:#04x})"
            );
        }
    }

    // ------------------------------------------------------------- AES DAG
    //
    // The full circuit of one block encryption. It uses the same operations
    // as the client circuit in
    // `tfhe-rs-dag/tfhe/examples/aes128_zhc.rs::encrypt_block`. Therefore
    // the test below measures the PBS count of the real pipeline, with the
    // bypasses. It does not estimate the count.
    //
    // The count is 10,624, which is near to 0.77 s on the device. The state
    // stays in bit form between the S-box, xtime and XOR operations. The
    // code makes a byte only where an operation reads one. The XOR
    // operations and the xtime joins are then linear. Each S-box reduces its
    // input bits, which have 5 to 7 terms, and does not split a byte.
    //
    // In byte form the count is 160 * 58 + 144 * 7 + 752 * 4 = 13,296.

    const RCON: [u8; 10] = [0x01, 0x02, 0x04, 0x08, 0x10, 0x20, 0x40, 0x80, 0x1B, 0x36];

    fn clear_key_expansion(key: [u8; 16]) -> [[u8; 16]; 11] {
        let mut keys = [[0u8; 16]; 11];
        keys[0] = key;
        for round in 1..=10 {
            let prev = keys[round - 1];
            let mut t = [prev[13], prev[14], prev[15], prev[12]];
            for byte in t.iter_mut() {
                *byte = AES_SBOX[*byte as usize];
            }
            t[0] ^= RCON[round - 1];
            for i in 0..4 {
                keys[round][i] = prev[i] ^ t[i];
            }
            for j in 1..4 {
                for i in 0..4 {
                    keys[round][j * 4 + i] = prev[j * 4 + i] ^ keys[round][(j - 1) * 4 + i];
                }
            }
        }
        keys
    }

    fn clear_shift_rows(state: &mut [u8; 16]) {
        let s = *state;
        for r in 1..4 {
            for c in 0..4 {
                state[c * 4 + r] = s[((c + r) % 4) * 4 + r];
            }
        }
    }

    fn clear_encrypt_block(pt: [u8; 16], rk: &[[u8; 16]; 11]) -> [u8; 16] {
        let mut s: [u8; 16] = std::array::from_fn(|i| pt[i] ^ rk[0][i]);
        for round in 1..=10 {
            for byte in s.iter_mut() {
                *byte = AES_SBOX[*byte as usize];
            }
            clear_shift_rows(&mut s);
            if round < 10 {
                for col in 0..4 {
                    let a: [u8; 4] = std::array::from_fn(|i| s[col * 4 + i]);
                    let x: [u8; 4] = a.map(clear_xtime);
                    s[col * 4] = x[0] ^ x[1] ^ a[1] ^ a[2] ^ a[3];
                    s[col * 4 + 1] = a[0] ^ x[1] ^ x[2] ^ a[2] ^ a[3];
                    s[col * 4 + 2] = a[0] ^ a[1] ^ x[2] ^ x[3] ^ a[3];
                    s[col * 4 + 3] = x[0] ^ a[0] ^ a[1] ^ a[2] ^ x[3];
                }
            }
            for i in 0..16 {
                s[i] ^= rk[round][i];
            }
        }
        s
    }

    /// Makes the circuit of one block encryption. It has 16 plaintext
    /// inputs, 176 round-key inputs and 16 ciphertext outputs. It uses the
    /// operations that the graph gives to zhc.
    fn build_aes_block(spec: CiphertextSpec) -> Builder {
        use crate::BwKind;
        let b = Builder::new(spec.block_spec());
        let sbox_table: Vec<(u128, u128)> = (0..256u128)
            .map(|i| (i, AES_SBOX[i as usize].into()))
            .collect();
        let xtime_table: Vec<(u128, u128)> = (0..256u128)
            .map(|i| (i, clear_xtime(i as u8).into()))
            .collect();

        let mut state: Vec<Ciphertext> = (0..16).map(|_| b.ciphertext_input(8)).collect();
        let rk: Vec<Vec<Ciphertext>> = (0..11)
            .map(|_| (0..16).map(|_| b.ciphertext_input(8)).collect())
            .collect();

        let xor = |x: &Ciphertext, y: &Ciphertext| b.iop_bitwise(x, y, BwKind::Xor);
        for i in 0..16 {
            state[i] = xor(&state[i], &rk[0][i]);
        }
        for round in 1..=10 {
            for byte in state.iter_mut() {
                *byte = b.iop_match_value(byte, &sbox_table, 8).0;
            }
            let s = state.clone();
            for r in 1..4 {
                for c in 0..4 {
                    state[c * 4 + r] = s[((c + r) % 4) * 4 + r].clone();
                }
            }
            if round < 10 {
                for col in 0..4 {
                    let a: Vec<Ciphertext> = (0..4).map(|i| state[col * 4 + i].clone()).collect();
                    let x: Vec<Ciphertext> = a
                        .iter()
                        .map(|v| b.iop_match_value(v, &xtime_table, 8).0)
                        .collect();
                    let three: Vec<Ciphertext> = (0..4).map(|i| xor(&x[i], &a[i])).collect();
                    state[col * 4] = xor(&xor(&x[0], &three[1]), &xor(&a[2], &a[3]));
                    state[col * 4 + 1] = xor(&xor(&a[0], &x[1]), &xor(&three[2], &a[3]));
                    state[col * 4 + 2] = xor(&xor(&a[0], &a[1]), &xor(&x[2], &three[3]));
                    state[col * 4 + 3] = xor(&xor(&three[0], &a[1]), &xor(&a[2], &x[3]));
                }
            }
            for i in 0..16 {
                state[i] = xor(&state[i], &rk[round][i]);
            }
        }
        for byte in state {
            b.ciphertext_output(byte);
        }
        b
    }

    /// Tests the PBS count of the full block circuit. The test also compares
    /// the result of the interpreter with clear AES-128.
    #[test]
    fn aes_block_dag_pbs_budget() {
        let spec = CiphertextSpec::new(8, 2, 2);
        let builder = build_aes_block(spec);

        let pbs = count_pbs(&builder);
        println!("aes block DAG PBS: {pbs}");
        assert_eq!(pbs, 10_624, "block PBS budget");

        // FIPS-197 appendix C.1 vector.
        let pt: [u8; 16] = std::array::from_fn(|i| (i as u8) * 0x11);
        let key: [u8; 16] = std::array::from_fn(|i| i as u8);
        let rk = clear_key_expansion(key);
        let expected = clear_encrypt_block(pt, &rk);
        assert_eq!(
            expected,
            [
                0x69, 0xc4, 0xe0, 0xd8, 0x6a, 0x7b, 0x04, 0x30, 0xd8, 0xcd, 0xb7, 0x80, 0x70, 0xb4,
                0xc5, 0x5a
            ],
            "clear reference must match FIPS-197"
        );

        let mut inputs: Vec<IopValue> = pt
            .iter()
            .map(|&v| IopValue::Ciphertext(spec.from_int(v.into())))
            .collect();
        for round_key in &rk {
            for &v in round_key {
                inputs.push(IopValue::Ciphertext(spec.from_int(v.into())));
            }
        }
        let outputs = builder.interpret().with_inputs(inputs).get_outputs();
        let expected_out: Vec<IopValue> = expected
            .iter()
            .map(|&v| IopValue::Ciphertext(spec.from_int(v.into())))
            .collect();
        assert_eq!(outputs, expected_out, "encrypted block vs clear AES");
    }

    /// Tests the bypass for xtime. A circuit that supplies the xtime table
    /// must get the bit-level circuit, and not the tree.
    #[test]
    fn match_value_uses_bitsliced_xtime() {
        let spec = CiphertextSpec::new(8, 2, 2);
        let table: Vec<(u128, u128)> = (0..256u128)
            .map(|i| (i, clear_xtime(i as u8).into()))
            .collect();
        let builder = Builder::new(spec.block_spec());
        let src = builder.ciphertext_input(spec.int_size());
        let (out, _flag) = builder.iop_match_value(&src, &table, 8);
        builder.ciphertext_output(out);
        assert_eq!(count_pbs(&builder), 7, "should route to iop_xtime");

        for x in [0u8, 1, 0x7f, 0x80, 0xff] {
            let outputs = builder
                .interpret()
                .with_inputs([IopValue::Ciphertext(spec.from_int(x.into()))])
                .get_outputs();
            assert_eq!(
                outputs,
                vec![IopValue::Ciphertext(spec.from_int(clear_xtime(x).into()))],
                "match_value xtime({x:#04x})"
            );
        }
    }
}
