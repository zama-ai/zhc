use std::fmt::Debug;

use zhc_abc as ABC;
use zhc_crypto::integer_semantics::{
    SpecBlock, SpecInteger,
    lut::{Lut1, Lut2, Lut4, Lut8, RawLut},
};
use zhc_utils::{SafeAs, iter::CollectInVec};

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Block {
    pub bits: Vec<ABC::Lit>,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Bool {
    pub block: Block,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Integer {
    pub blocks: Vec<Block>,
}

pub struct Circuit {
    circuit: ABC::Circuit,
    spec: SpecBlock,
}

impl Debug for Circuit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Circuit")
            .field("spec", &self.spec)
            .field("n_inputs", &self.circuit.n_inputs())
            .field("n_outputs", &self.circuit.n_outputs())
            .field("n_ands", &self.circuit.n_ands())
            .finish()
    }
}

impl Circuit {
    pub fn new(spec: SpecBlock) -> Self {
        Self {
            circuit: ABC::Circuit::new(),
            spec,
        }
    }

    pub fn spec(&self) -> SpecBlock {
        self.spec
    }

    pub fn circuit(&self) -> &ABC::Circuit {
        &self.circuit
    }

    pub fn into_circuit(self) -> ABC::Circuit {
        self.circuit
    }

    fn block_len(&self) -> usize {
        self.spec.complete_size() as usize
    }

    fn n_block_values(&self) -> usize {
        1 << self.block_len()
    }

    pub fn const_block(&self, value: u16) -> Block {
        assert!(
            value & !self.spec.complete_mask() == 0,
            "Value {value} exceeds the complete block width"
        );
        let bits = (0..self.block_len())
                .map(|i| self.circuit.constant((value >> i) & 1 == 1))
                .collect();
        Block { bits }
    }

    pub fn input_block(&mut self) -> Block {
        let mut bits = Vec::new();
        for _ in 0..self.spec.message_size() {
            bits.push(self.circuit.input());
        }
        for _ in 0..=self.spec.carry_size() {
            bits.push(self.circuit.constant(false));
        }
        Block { bits }
    }

    pub fn output_block(&mut self, block: &Block) {
        for bit in &block.bits {
            self.circuit.output(*bit);
        }
    }

    pub fn input_integer(&mut self, spec: SpecInteger) -> Integer {
        assert_eq!(spec.block_spec(), self.spec, "Spec mismatch.");
        let mut blocks = Vec::new();
        for _ in 0..spec.block_count() {
            blocks.push(self.input_block());
        }
        Integer { blocks }
    }

    pub fn output_integer(&mut self, ct: &Integer) {
        for block in &ct.blocks {
            self.output_block(block);
        }
    }

    pub fn input_bool(&mut self) -> Bool {
        Bool { block: self.input_block() }
    }

    pub fn output_bool(&mut self, ct: &Bool) {
        self.output_block(&ct.block);
    }

    pub fn add(&mut self, a: &Block, b: &Block) -> Block {
        let zero = self.circuit.constant(false);
        let bits = self.bits_add(&a.bits, &b.bits, zero);
        Block { bits }
    }

    pub fn sub(&mut self, a: &Block, b: &Block) -> Block {
        let bits = self.bits_sub(&a.bits, &b.bits);
        Block { bits }
    }

    pub fn mul(&mut self, a: &Block, b: &Block) -> Block {
        let bits = self.bits_mul(&a.bits, &b.bits);
        Block { bits }
    }

    pub fn shl(&mut self, a: &Block, amount: u8) -> Block {
        let bits = self.bits_shl(&a.bits, amount as usize);
        Block { bits }
    }

    pub fn mac(
        &mut self,
        a: &Block,
        mul: u16,
        b: &Block,
    ) -> Block {
        let zero = self.circuit.constant(false);
        let mul = self.const_block(mul);
        let prod = self.bits_mul(&a.bits, &mul.bits);
        let bits = self.bits_add(&prod, &b.bits, zero);
        Block { bits }
    }

    pub fn lut1(&mut self, lut: &Lut1, a: &Block) -> Block {
        let [out] = self.many_pbs(&lut.0, a);
        out
    }

    pub fn lut2(&mut self, lut: &Lut2, a: &Block) -> [Block; 2] {
        self.many_pbs(&lut.0, a)
    }

    pub fn lut4(&mut self, lut: &Lut4, a: &Block) -> [Block; 4] {
        self.many_pbs(&lut.0, a)
    }

    pub fn lut8(&mut self, lut: &Lut8, a: &Block) -> [Block; 8] {
        self.many_pbs(&lut.0, a)
    }

    fn many_pbs<const N: usize>(&mut self, raw: &RawLut, a: &Block) -> [Block; N] {
        let sub_table_len = raw.lut().len() / N;
        std::array::from_fn(|j| {
            let offset = self.const_block((j * sub_table_len).sas());
            let shifted = self.add(a, &offset);
            self.pbs(raw, &shifted)
        })
    }

    fn pbs(&mut self, raw: &RawLut, a: &Block) -> Block {
        let mut bits = vec![self.circuit.constant(false); self.block_len()];
        for v in 0..self.n_block_values() {
            let out = raw.pbs(self.spec.from_complete(v.sas())).raw_complete_bits();
            let v = self.const_block(v.sas());
            let hit = self.bits_eq(&a.bits, &v.bits);
            for (i, bit) in bits.iter_mut().enumerate() {
                if (out >> i) & 1 == 1 {
                    *bit = self.circuit.or(*bit, hit);
                }
            }
        }
        Block { bits }
    }

}

impl Circuit {

    fn bits_extend(&self, bits: &[ABC::Lit]) -> Vec<ABC::Lit> {
        assert!(bits.len() <= self.block_len(), "operand wider than a block");
        let mut out = bits.to_vec();
        out.resize(self.block_len(), self.circuit.constant(false));
        out
    }

    fn bits_add(&mut self, a: &[ABC::Lit], b: &[ABC::Lit], mut carry: ABC::Lit) -> Vec<ABC::Lit> {
        let a = self.bits_extend(a);
        let b = self.bits_extend(b);
        let mut out = Vec::with_capacity(self.block_len());
        for i in 0..self.block_len() {
            let axb = self.circuit.xor(a[i], b[i]);
            out.push(self.circuit.xor(axb, carry));
            let t = self.circuit.and(a[i], b[i]);
            let u = self.circuit.and(carry, axb);
            carry = self.circuit.or(t, u);
        }
        out
    }

    fn bits_sub(&mut self, a: &[ABC::Lit], b: &[ABC::Lit]) -> Vec<ABC::Lit> {
        let b = self.bits_extend(b);
        let nb = b.iter().map(|l| self.circuit.not(*l)).covec();
        let one = self.circuit.constant(true);
        self.bits_add(a, &nb, one)
    }

    fn bits_shl(&self, a: &[ABC::Lit], amount: usize) -> Vec<ABC::Lit> {
        let a = self.bits_extend(a);
        (0..self.block_len())
            .map(|i| {
                if i >= amount {
                    a[i - amount]
                } else {
                    self.circuit.constant(false)
                }
            })
            .collect()
    }

    fn bits_mul(&mut self, a: &[ABC::Lit], b: &[ABC::Lit]) -> Vec<ABC::Lit> {
        let zero = self.circuit.constant(false);
        let b = self.bits_extend(b);
        let mut acc = self.const_block(0).bits;
        for i in 0..self.block_len() {
            let shifted = self.bits_shl(a, i);
            let gated = shifted.iter().map(|l| self.circuit.and(*l, b[i])).covec();
            acc = self.bits_add(&acc, &gated, zero);
        }
        acc
    }

    fn bits_eq(&mut self, a: &[ABC::Lit], b: &[ABC::Lit]) -> ABC::Lit {
        let a = self.bits_extend(a);
        let b = self.bits_extend(b);
        let mut acc = self.circuit.constant(true);
        for i in 0..self.block_len() {
            let differ = self.circuit.xor(a[i], b[i]);
            let same = self.circuit.not(differ);
            acc = self.circuit.and(acc, same);
        }
        acc
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use zhc_abc::{CheckOptions, Verdict, check_equivalence};
    use zhc_crypto::integer_semantics::{EmulatedPlaintextBlock, lut::LookupCheck};

    #[test]
    fn input_block_has_free_message_bits_and_false_upper_bits() {
        let spec = SpecBlock(2, 2);
        let mut c = Circuit::new(spec);
        let a = c.input_block();
        assert_eq!(a.bits.len(), spec.complete_size() as usize);
        assert_eq!(c.circuit().n_inputs(), spec.message_size() as usize);
        for (i, bit) in a.bits.iter().enumerate() {
            if i < spec.message_size() as usize {
                assert!(!bit.is_constant(), "bit {i} must be an input");
            } else {
                assert_eq!(*bit, ABC::Lit::FALSE, "bit {i} must be false");
            }
        }
    }

    #[test]
    fn const_block_holds_the_value() {
        let spec = SpecBlock(2, 2);
        let c = Circuit::new(spec);
        for x in 0..1u16 << spec.complete_size() {
            let a = c.const_block(x);
            assert_eq!(a.bits.len(), spec.complete_size() as usize);
            for (i, bit) in a.bits.iter().enumerate() {
                let want = if (x >> i) & 1 == 1 {
                    ABC::Lit::TRUE
                } else {
                    ABC::Lit::FALSE
                };
                assert_eq!(*bit, want, "x={x} bit={i}");
            }
        }
    }

    #[test]
    fn add_matches_emulated_blocks() {
        let spec = SpecBlock(2, 2);
        let mut c = Circuit::new(spec);
        for x in 0..1u16 << spec.complete_size() {
            for y in 0..1u16 << spec.complete_size() {
                let a = c.const_block(x);
                let b = c.const_block(y);
                let out = c.add(&a, &b);
                let expected = spec
                    .from_complete(x)
                    .wrapping_add(spec.from_complete(y))
                    .raw_complete_bits();
                assert_eq!(out.bits.len(), spec.complete_size() as usize);
                for (i, bit) in out.bits.iter().enumerate() {
                    let want = if (expected >> i) & 1 == 1 {
                        ABC::Lit::TRUE
                    } else {
                        ABC::Lit::FALSE
                    };
                    assert_eq!(*bit, want, "x={x} y={y} bit={i}");
                }
            }
        }
    }

    #[test]
    fn sub_matches_emulated_blocks() {
        let spec = SpecBlock(2, 2);
        let mut c = Circuit::new(spec);
        for x in 0..1u16 << spec.complete_size() {
            for y in 0..1u16 << spec.complete_size() {
                let a = c.const_block(x);
                let b = c.const_block(y);
                let out = c.sub(&a, &b);
                let expected = spec
                    .from_complete(x)
                    .wrapping_sub(spec.from_complete(y))
                    .raw_complete_bits();
                assert_eq!(out.bits.len(), spec.complete_size() as usize);
                for (i, bit) in out.bits.iter().enumerate() {
                    let want = if (expected >> i) & 1 == 1 {
                        ABC::Lit::TRUE
                    } else {
                        ABC::Lit::FALSE
                    };
                    assert_eq!(*bit, want, "x={x} y={y} bit={i}");
                }
            }
        }
    }

    #[test]
    fn mul_matches_emulated_blocks() {
        let spec = SpecBlock(2, 2);
        let mut c = Circuit::new(spec);
        for x in 0..1u16 << spec.complete_size() {
            for y in 0..1u16 << spec.complete_size() {
                let a = c.const_block(x);
                let b = c.const_block(y);
                let out = c.mul(&a, &b);
                let expected = spec
                    .from_complete(x)
                    .wrapping_mul_pt(EmulatedPlaintextBlock::from_complete(spec, y))
                    .raw_complete_bits();
                assert_eq!(out.bits.len(), spec.complete_size() as usize);
                for (i, bit) in out.bits.iter().enumerate() {
                    let want = if (expected >> i) & 1 == 1 {
                        ABC::Lit::TRUE
                    } else {
                        ABC::Lit::FALSE
                    };
                    assert_eq!(*bit, want, "x={x} y={y} bit={i}");
                }
            }
        }
    }

    #[test]
    fn shl_matches_emulated_blocks() {
        let spec = SpecBlock(2, 2);
        let mut c = Circuit::new(spec);
        for x in 0..1u16 << spec.complete_size() {
            for amount in 0..=spec.complete_size() {
                let a = c.const_block(x);
                let out = c.shl(&a, amount);
                let expected = spec
                    .from_complete(x)
                    .wrapping_shl(amount)
                    .raw_complete_bits();
                assert_eq!(out.bits.len(), spec.complete_size() as usize);
                for (i, bit) in out.bits.iter().enumerate() {
                    let want = if (expected >> i) & 1 == 1 {
                        ABC::Lit::TRUE
                    } else {
                        ABC::Lit::FALSE
                    };
                    assert_eq!(*bit, want, "x={x} amount={amount} bit={i}");
                }
            }
        }
    }

    #[test]
    fn mac_matches_emulated_blocks() {
        let spec = SpecBlock(2, 2);
        let mut c = Circuit::new(spec);
        for mul in [1u8, 2, 4, 8] {
            for x in 0..1u16 << spec.complete_size() {
                for y in 0..1u16 << spec.complete_size() {
                    let a = c.const_block(x);
                    let b = c.const_block(y);
                    let out = c.mac(&a, mul as u16, &b);
                    let expected = spec
                        .from_complete(x)
                        .wrapping_mac(mul, spec.from_complete(y))
                        .raw_complete_bits();
                    assert_eq!(out.bits.len(), spec.complete_size() as usize);
                    for (i, bit) in out.bits.iter().enumerate() {
                        let want = if (expected >> i) & 1 == 1 {
                            ABC::Lit::TRUE
                        } else {
                            ABC::Lit::FALSE
                        };
                        assert_eq!(*bit, want, "mul={mul} x={x} y={y} bit={i}");
                    }
                }
            }
        }
    }

    #[test]
    fn lut1_matches_emulated_lookup() {
        let spec = SpecBlock(2, 2);
        let lut = Lut1::from_fn("clear_carry", spec, |b| b.mask_message());
        let mut c = Circuit::new(spec);
        for x in 0..1u16 << spec.complete_size() {
            let a = c.const_block(x);
            let out = c.lut1(&lut, &a);
            let expected = lut
                .lookup(spec.from_complete(x), LookupCheck::Permissive)
                .raw_complete_bits();
            assert_eq!(out.bits.len(), spec.complete_size() as usize);
            for (i, bit) in out.bits.iter().enumerate() {
                let want = if (expected >> i) & 1 == 1 {
                    ABC::Lit::TRUE
                } else {
                    ABC::Lit::FALSE
                };
                assert_eq!(*bit, want, "x={x} bit={i}");
            }
        }
    }

    #[test]
    fn lut2_matches_emulated_lookup() {
        let spec = SpecBlock(2, 2);
        let lut = Lut2::from_fn(
            "message_and_carry",
            spec,
            |b| b.mask_message(),
            |b| b.move_carry_to_message(),
        );
        let mut c = Circuit::new(spec);
        for x in 0..1u16 << spec.complete_size() {
            let a = c.const_block(x);
            let outs = c.lut2(&lut, &a);
            let (e0, e1) = lut.lookup(spec.from_complete(x), LookupCheck::Permissive);
            let expected = [e0.raw_complete_bits(), e1.raw_complete_bits()];
            for (j, out) in outs.iter().enumerate() {
                assert_eq!(out.bits.len(), spec.complete_size() as usize);
                for (i, bit) in out.bits.iter().enumerate() {
                    let want = if (expected[j] >> i) & 1 == 1 {
                        ABC::Lit::TRUE
                    } else {
                        ABC::Lit::FALSE
                    };
                    assert_eq!(*bit, want, "x={x} output={j} bit={i}");
                }
            }
        }
    }

    #[test]
    fn lut4_matches_emulated_lookup() {
        let spec = SpecBlock(2, 2);
        let lut = Lut4::from_fn(
            "four",
            spec,
            |b| b.mask_message(),
            |b| b.move_carry_to_message(),
            |b| b.mask_carry(),
            |b| b,
        );
        let mut c = Circuit::new(spec);
        for x in 0..1u16 << spec.complete_size() {
            let a = c.const_block(x);
            let outs = c.lut4(&lut, &a);
            let (e0, e1, e2, e3) = lut.lookup(spec.from_complete(x), LookupCheck::Permissive);
            let expected = [
                e0.raw_complete_bits(),
                e1.raw_complete_bits(),
                e2.raw_complete_bits(),
                e3.raw_complete_bits(),
            ];
            for (j, out) in outs.iter().enumerate() {
                assert_eq!(out.bits.len(), spec.complete_size() as usize);
                for (i, bit) in out.bits.iter().enumerate() {
                    let want = if (expected[j] >> i) & 1 == 1 {
                        ABC::Lit::TRUE
                    } else {
                        ABC::Lit::FALSE
                    };
                    assert_eq!(*bit, want, "x={x} output={j} bit={i}");
                }
            }
        }
    }

    #[test]
    fn lut8_matches_emulated_lookup() {
        let spec = SpecBlock(2, 2);
        let lut = Lut8::from_fn(
            "eight",
            spec,
            |b| b.mask_message(),
            |b| b.move_carry_to_message(),
            |b| b.mask_carry(),
            |b| b,
            |b| b.mask_message(),
            |b| b,
            |b| b.mask_carry(),
            |b| b.move_carry_to_message(),
        );
        let mut c = Circuit::new(spec);
        for x in 0..1u16 << spec.complete_size() {
            let a = c.const_block(x);
            let outs = c.lut8(&lut, &a);
            let (e0, e1, e2, e3, e4, e5, e6, e7) =
                lut.lookup(spec.from_complete(x), LookupCheck::Permissive);
            let expected = [
                e0.raw_complete_bits(),
                e1.raw_complete_bits(),
                e2.raw_complete_bits(),
                e3.raw_complete_bits(),
                e4.raw_complete_bits(),
                e5.raw_complete_bits(),
                e6.raw_complete_bits(),
                e7.raw_complete_bits(),
            ];
            for (j, out) in outs.iter().enumerate() {
                assert_eq!(out.bits.len(), spec.complete_size() as usize);
                for (i, bit) in out.bits.iter().enumerate() {
                    let want = if (expected[j] >> i) & 1 == 1 {
                        ABC::Lit::TRUE
                    } else {
                        ABC::Lit::FALSE
                    };
                    assert_eq!(*bit, want, "x={x} output={j} bit={i}");
                }
            }
        }
    }

    #[test]
    fn add_of_inputs_is_proven_commutative() {
        let spec = SpecBlock(2, 2);

        let mut left = Circuit::new(spec);
        let a = left.input_block();
        let b = left.input_block();
        let out = left.add(&a, &b);
        left.output_block(&out);

        let mut right = Circuit::new(spec);
        let a = right.input_block();
        let b = right.input_block();
        let out = right.add(&b, &a);
        right.output_block(&out);

        let verdict =
            check_equivalence(left.circuit(), right.circuit(), &CheckOptions::default()).unwrap();
        assert_eq!(verdict, Verdict::Equivalent);
    }

    #[test]
    fn add_and_sub_of_inputs_are_proven_different() {
        let spec = SpecBlock(2, 2);

        let mut left = Circuit::new(spec);
        let a = left.input_block();
        let b = left.input_block();
        let out = left.add(&a, &b);
        left.output_block(&out);

        let mut right = Circuit::new(spec);
        let a = right.input_block();
        let b = right.input_block();
        let out = right.sub(&a, &b);
        right.output_block(&out);

        let verdict =
            check_equivalence(left.circuit(), right.circuit(), &CheckOptions::default()).unwrap();
        assert!(matches!(verdict, Verdict::NotEquivalent(_)));
    }
}
