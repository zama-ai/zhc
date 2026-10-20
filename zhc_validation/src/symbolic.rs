use std::fmt::Debug;

use zhc_abc::{Circuit, Lit};
use zhc_crypto::integer_semantics::{
    CiphertextBlockSpec, CiphertextSpec, EmulatedCiphertextBlock, PlaintextBlockSpec,
    PlaintextSpec,
    lut::{LookupCheck, Lut1, Lut2, Lut4, Lut8},
};

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SymCiphertextBlock {
    bits: Vec<Lit>,
    spec: CiphertextBlockSpec,
}

impl SymCiphertextBlock {
    fn new(bits: Vec<Lit>, spec: CiphertextBlockSpec) -> Self {
        assert_eq!(bits.len(), spec.complete_size() as usize);
        Self { bits, spec }
    }

    fn check_spec(&self, spec: CiphertextBlockSpec) {
        assert_eq!(self.spec, spec, "Spec mismatch.");
    }

    pub fn spec(&self) -> CiphertextBlockSpec {
        self.spec
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SymPlaintextBlock {
    bits: Vec<Lit>,
    spec: PlaintextBlockSpec,
}

impl SymPlaintextBlock {
    fn new(bits: Vec<Lit>, spec: PlaintextBlockSpec) -> Self {
        assert_eq!(bits.len(), spec.message_size() as usize);
        Self { bits, spec }
    }

    fn check_fits(&self, spec: CiphertextBlockSpec) {
        assert!(
            self.spec.message_size() <= spec.complete_size(),
            "Spec mismatch."
        );
    }

    pub fn spec(&self) -> PlaintextBlockSpec {
        self.spec
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SymCiphertext {
    blocks: Vec<SymCiphertextBlock>,
    spec: CiphertextSpec,
}

impl SymCiphertext {
    fn new(blocks: Vec<SymCiphertextBlock>, spec: CiphertextSpec) -> Self {
        assert_eq!(blocks.len(), spec.block_count() as usize);
        for block in &blocks {
            block.check_spec(spec.block_spec());
        }
        Self { blocks, spec }
    }

    pub fn spec(&self) -> CiphertextSpec {
        self.spec
    }

    pub fn len(&self) -> u8 {
        self.blocks.len() as u8
    }

    pub fn is_empty(&self) -> bool {
        self.blocks.is_empty()
    }

    pub fn get_block(&self, ith: u8) -> SymCiphertextBlock {
        self.blocks[ith as usize].clone()
    }

    pub fn set_block(&mut self, ith: u8, block: SymCiphertextBlock) {
        block.check_spec(self.spec.block_spec());
        self.blocks[ith as usize] = block;
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SymPlaintext {
    blocks: Vec<SymPlaintextBlock>,
    spec: PlaintextSpec,
}

impl SymPlaintext {
    fn new(blocks: Vec<SymPlaintextBlock>, spec: PlaintextSpec) -> Self {
        assert_eq!(blocks.len(), spec.block_count() as usize);
        for block in &blocks {
            assert_eq!(block.spec, spec.block_spec(), "Spec mismatch.");
        }
        Self { blocks, spec }
    }

    pub fn spec(&self) -> PlaintextSpec {
        self.spec
    }

    pub fn len(&self) -> u8 {
        self.blocks.len() as u8
    }

    pub fn is_empty(&self) -> bool {
        self.blocks.is_empty()
    }

    pub fn get_block(&self, ith: u8) -> SymPlaintextBlock {
        self.blocks[ith as usize].clone()
    }
}

pub struct SymCircuit {
    circuit: Circuit,
    spec: CiphertextBlockSpec,
}

impl Debug for SymCircuit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SymCircuit")
            .field("spec", &self.spec)
            .field("n_inputs", &self.circuit.n_inputs())
            .field("n_outputs", &self.circuit.n_outputs())
            .field("n_ands", &self.circuit.n_ands())
            .finish()
    }
}

impl SymCircuit {
    pub fn new(spec: CiphertextBlockSpec) -> Self {
        Self {
            circuit: Circuit::new(),
            spec,
        }
    }

    pub fn spec(&self) -> CiphertextBlockSpec {
        self.spec
    }

    pub fn circuit(&self) -> &Circuit {
        &self.circuit
    }

    pub fn into_circuit(self) -> Circuit {
        self.circuit
    }

    pub fn ciphertext_block_from_complete(&self, value: u16) -> SymCiphertextBlock {
        assert!(
            value & !self.spec.complete_mask() == 0,
            "Value {value} exceeds the complete block width"
        );
        SymCiphertextBlock::new(self.constant_bits(value, self.width()), self.spec)
    }

    pub fn plaintext_block_from_message(
        &self,
        spec: PlaintextBlockSpec,
        value: u16,
    ) -> SymPlaintextBlock {
        let width = spec.message_size() as usize;
        assert!(
            width >= 16 || value >> width == 0,
            "Value {value} exceeds the plaintext block width"
        );
        SymPlaintextBlock::new(self.constant_bits(value, width), spec)
    }

    pub fn input_ciphertext_block(&mut self) -> SymCiphertextBlock {
        let msg = self.spec.message_size() as usize;
        let mut bits = self.constant_bits(0, self.width());
        for bit in bits.iter_mut().take(msg) {
            *bit = self.circuit.input();
        }
        SymCiphertextBlock::new(bits, self.spec)
    }

    pub fn input_plaintext_block(&mut self, spec: PlaintextBlockSpec) -> SymPlaintextBlock {
        let bits = (0..spec.message_size())
            .map(|_| self.circuit.input())
            .collect();
        SymPlaintextBlock::new(bits, spec)
    }

    pub fn input_ciphertext(&mut self, spec: CiphertextSpec) -> SymCiphertext {
        assert_eq!(spec.block_spec(), self.spec, "Spec mismatch.");
        let blocks = (0..spec.block_count())
            .map(|_| self.input_ciphertext_block())
            .collect();
        SymCiphertext::new(blocks, spec)
    }

    pub fn input_plaintext(&mut self, spec: PlaintextSpec) -> SymPlaintext {
        let blocks = (0..spec.block_count())
            .map(|_| self.input_plaintext_block(spec.block_spec()))
            .collect();
        SymPlaintext::new(blocks, spec)
    }

    pub fn declare_ciphertext(&self, spec: CiphertextSpec) -> SymCiphertext {
        assert_eq!(spec.block_spec(), self.spec, "Spec mismatch.");
        let blocks = (0..spec.block_count())
            .map(|_| self.ciphertext_block_from_complete(0))
            .collect();
        SymCiphertext::new(blocks, spec)
    }

    pub fn output_ciphertext(&mut self, ct: &SymCiphertext) {
        let msg = self.spec.message_size() as usize;
        for block in &ct.blocks {
            block.check_spec(self.spec);
            for bit in &block.bits[..msg] {
                self.circuit.output(*bit);
            }
        }
    }

    pub fn add(&mut self, a: &SymCiphertextBlock, b: &SymCiphertextBlock) -> SymCiphertextBlock {
        a.check_spec(self.spec);
        b.check_spec(self.spec);
        let zero = self.circuit.constant(false);
        let bits = self.add_bits(&a.bits, &b.bits, zero);
        SymCiphertextBlock::new(bits, self.spec)
    }

    pub fn sub(&mut self, a: &SymCiphertextBlock, b: &SymCiphertextBlock) -> SymCiphertextBlock {
        a.check_spec(self.spec);
        b.check_spec(self.spec);
        let bits = self.sub_bits(&a.bits, &b.bits);
        SymCiphertextBlock::new(bits, self.spec)
    }

    pub fn add_pt(&mut self, a: &SymCiphertextBlock, b: &SymPlaintextBlock) -> SymCiphertextBlock {
        a.check_spec(self.spec);
        b.check_fits(self.spec);
        let zero = self.circuit.constant(false);
        let bits = self.add_bits(&a.bits, &b.bits, zero);
        SymCiphertextBlock::new(bits, self.spec)
    }

    pub fn sub_pt(&mut self, a: &SymCiphertextBlock, b: &SymPlaintextBlock) -> SymCiphertextBlock {
        a.check_spec(self.spec);
        b.check_fits(self.spec);
        let bits = self.sub_bits(&a.bits, &b.bits);
        SymCiphertextBlock::new(bits, self.spec)
    }

    pub fn pt_sub(&mut self, a: &SymPlaintextBlock, b: &SymCiphertextBlock) -> SymCiphertextBlock {
        a.check_fits(self.spec);
        b.check_spec(self.spec);
        let bits = self.sub_bits(&a.bits, &b.bits);
        SymCiphertextBlock::new(bits, self.spec)
    }

    pub fn mul_pt(&mut self, a: &SymCiphertextBlock, b: &SymPlaintextBlock) -> SymCiphertextBlock {
        a.check_spec(self.spec);
        b.check_fits(self.spec);
        let bits = self.mul_bits(&a.bits, &b.bits);
        SymCiphertextBlock::new(bits, self.spec)
    }

    pub fn add_cst(&mut self, a: &SymCiphertextBlock, value: u16) -> SymCiphertextBlock {
        a.check_spec(self.spec);
        let zero = self.circuit.constant(false);
        let cst = self.constant_bits(value, self.width());
        let bits = self.add_bits(&a.bits, &cst, zero);
        SymCiphertextBlock::new(bits, self.spec)
    }

    pub fn sub_cst(&mut self, a: &SymCiphertextBlock, value: u16) -> SymCiphertextBlock {
        a.check_spec(self.spec);
        let cst = self.constant_bits(value, self.width());
        let bits = self.sub_bits(&a.bits, &cst);
        SymCiphertextBlock::new(bits, self.spec)
    }

    pub fn cst_sub(&mut self, value: u16, a: &SymCiphertextBlock) -> SymCiphertextBlock {
        a.check_spec(self.spec);
        let cst = self.constant_bits(value, self.width());
        let bits = self.sub_bits(&cst, &a.bits);
        SymCiphertextBlock::new(bits, self.spec)
    }

    pub fn mul_cst(&mut self, a: &SymCiphertextBlock, value: u16) -> SymCiphertextBlock {
        a.check_spec(self.spec);
        let bits = self.mul_const_bits(&a.bits, value);
        SymCiphertextBlock::new(bits, self.spec)
    }

    pub fn shl(&mut self, a: &SymCiphertextBlock, amount: u8) -> SymCiphertextBlock {
        a.check_spec(self.spec);
        let bits = self.shl_bits(&a.bits, amount as usize);
        SymCiphertextBlock::new(bits, self.spec)
    }

    pub fn mac(
        &mut self,
        a: &SymCiphertextBlock,
        mul: u8,
        b: &SymCiphertextBlock,
    ) -> SymCiphertextBlock {
        a.check_spec(self.spec);
        b.check_spec(self.spec);
        let zero = self.circuit.constant(false);
        let prod = self.mul_const_bits(&a.bits, mul as u16);
        let bits = self.add_bits(&prod, &b.bits, zero);
        SymCiphertextBlock::new(bits, self.spec)
    }

    pub fn lut1(&mut self, lut: &Lut1, a: &SymCiphertextBlock) -> SymCiphertextBlock {
        let [out] = self.lookup(a, 0, |b| [lut.lookup(b, LookupCheck::AllowBothPadding)]);
        out
    }

    pub fn lut2(&mut self, lut: &Lut2, a: &SymCiphertextBlock) -> [SymCiphertextBlock; 2] {
        self.lookup(a, 2, |b| {
            let r = lut.lookup(b, LookupCheck::AllowOutputPadding);
            [r.0, r.1]
        })
    }

    pub fn lut4(&mut self, lut: &Lut4, a: &SymCiphertextBlock) -> [SymCiphertextBlock; 4] {
        self.lookup(a, 3, |b| {
            let r = lut.lookup(b, LookupCheck::AllowOutputPadding);
            [r.0, r.1, r.2, r.3]
        })
    }

    pub fn lut8(&mut self, lut: &Lut8, a: &SymCiphertextBlock) -> [SymCiphertextBlock; 8] {
        self.lookup(a, 4, |b| {
            let r = lut.lookup(b, LookupCheck::AllowOutputPadding);
            [r.0, r.1, r.2, r.3, r.4, r.5, r.6, r.7]
        })
    }
}

impl SymCircuit {
    fn width(&self) -> usize {
        self.spec.complete_size() as usize
    }

    fn constant_bits(&self, value: u16, width: usize) -> Vec<Lit> {
        (0..width)
            .map(|i| self.circuit.constant((value >> i) & 1 == 1))
            .collect()
    }

    fn extend(&self, bits: &[Lit]) -> Vec<Lit> {
        let width = self.width();
        assert!(bits.len() <= width, "operand wider than a block");
        let mut out = bits.to_vec();
        out.resize(width, self.circuit.constant(false));
        out
    }

    fn add_bits(&mut self, a: &[Lit], b: &[Lit], mut carry: Lit) -> Vec<Lit> {
        let a = self.extend(a);
        let b = self.extend(b);
        let mut out = Vec::with_capacity(self.width());
        for i in 0..self.width() {
            let axb = self.circuit.xor(a[i], b[i]);
            out.push(self.circuit.xor(axb, carry));
            let t = self.circuit.and(a[i], b[i]);
            let u = self.circuit.and(carry, axb);
            carry = self.circuit.or(t, u);
        }
        out
    }

    fn sub_bits(&mut self, a: &[Lit], b: &[Lit]) -> Vec<Lit> {
        let b = self.extend(b);
        let nb: Vec<Lit> = b.iter().map(|l| self.circuit.not(*l)).collect();
        let one = self.circuit.constant(true);
        self.add_bits(a, &nb, one)
    }

    fn shl_bits(&self, a: &[Lit], amount: usize) -> Vec<Lit> {
        let a = self.extend(a);
        (0..self.width())
            .map(|i| {
                if i >= amount {
                    a[i - amount]
                } else {
                    self.circuit.constant(false)
                }
            })
            .collect()
    }

    fn mul_const_bits(&mut self, a: &[Lit], value: u16) -> Vec<Lit> {
        let zero = self.circuit.constant(false);
        let mut acc = self.constant_bits(0, self.width());
        for i in 0..self.width() {
            if (value >> i) & 1 == 1 {
                let shifted = self.shl_bits(a, i);
                acc = self.add_bits(&acc, &shifted, zero);
            }
        }
        acc
    }

    fn mul_bits(&mut self, a: &[Lit], b: &[Lit]) -> Vec<Lit> {
        let zero = self.circuit.constant(false);
        let b = self.extend(b);
        let mut acc = self.constant_bits(0, self.width());
        for i in 0..self.width() {
            let shifted = self.shl_bits(a, i);
            let gated: Vec<Lit> = shifted.iter().map(|l| self.circuit.and(*l, b[i])).collect();
            acc = self.add_bits(&acc, &gated, zero);
        }
        acc
    }

    fn table(&mut self, a: &[Lit], entries: &[u16]) -> Vec<Lit> {
        let width = self.width();
        assert_eq!(
            entries.len(),
            1 << width,
            "table size does not match the block width"
        );
        (0..width)
            .map(|bit| {
                let leaves: Vec<Lit> = entries
                    .iter()
                    .map(|e| self.circuit.constant((e >> bit) & 1 == 1))
                    .collect();
                self.mux_tree(a, &leaves)
            })
            .collect()
    }

    fn mux_tree(&mut self, sel: &[Lit], leaves: &[Lit]) -> Lit {
        if leaves.len() == 1 {
            return leaves[0];
        }
        let half = leaves.len() / 2;
        let top = sel[sel.len() - 1];
        let rest = &sel[..sel.len() - 1];
        let lo = self.mux_tree(rest, &leaves[..half]);
        let hi = self.mux_tree(rest, &leaves[half..]);
        self.circuit.mux(top, hi, lo)
    }

    fn lookup<const N: usize>(
        &mut self,
        a: &SymCiphertextBlock,
        zero_top_bits: usize,
        f: impl Fn(EmulatedCiphertextBlock) -> [EmulatedCiphertextBlock; N],
    ) -> [SymCiphertextBlock; N] {
        a.check_spec(self.spec);
        let tables = enumerate_tables(self.spec, zero_top_bits, f);
        tables.map(|t| {
            let bits = self.table(&a.bits, &t);
            SymCiphertextBlock::new(bits, self.spec)
        })
    }
}

fn enumerate_tables<const N: usize>(
    spec: CiphertextBlockSpec,
    zero_top_bits: usize,
    f: impl Fn(EmulatedCiphertextBlock) -> [EmulatedCiphertextBlock; N],
) -> [Vec<u16>; N] {
    let width = spec.complete_size() as usize;
    let mut tables: [Vec<u16>; N] = std::array::from_fn(|_| Vec::with_capacity(1 << width));
    for v in 0..1u16 << width {
        if (v >> (width - zero_top_bits)) != 0 {
            for t in tables.iter_mut() {
                t.push(0);
            }
            continue;
        }
        let outs = f(spec.from_complete(v));
        for (t, o) in tables.iter_mut().zip(outs) {
            t.push(o.raw_complete_bits());
        }
    }
    tables
}

#[cfg(test)]
mod tests {
    use super::*;
    use zhc_abc::{CheckOptions, Verdict, check_equivalence};

    const SPEC: CiphertextBlockSpec = CiphertextBlockSpec(2, 2);

    fn free_ct(c: &mut SymCircuit) -> SymCiphertextBlock {
        let bits = (0..c.width()).map(|_| c.circuit.input()).collect();
        SymCiphertextBlock::new(bits, SPEC)
    }

    fn free_pt(c: &mut SymCircuit) -> SymPlaintextBlock {
        c.input_plaintext_block(SPEC.matching_plaintext_block_spec())
    }

    fn check(
        gates: impl Fn(&mut SymCircuit, &SymCiphertextBlock, &SymCiphertextBlock) -> SymCiphertextBlock,
        reference: impl Fn(u16, u16) -> u16,
    ) {
        let width = SPEC.complete_size() as usize;
        let mut left = SymCircuit::new(SPEC);
        let a = free_ct(&mut left);
        let b = free_ct(&mut left);
        let out = gates(&mut left, &a, &b);
        for bit in &out.bits {
            left.circuit.output(*bit);
        }

        let mut right = SymCircuit::new(SPEC);
        let a = free_ct(&mut right);
        let b = free_ct(&mut right);
        let mut ab = a.bits.clone();
        ab.extend_from_slice(&b.bits);
        let mask = SPEC.complete_mask();
        for bit in 0..width {
            let leaves: Vec<Lit> = (0..1u32 << (2 * width))
                .map(|v| {
                    let x = (v & mask as u32) as u16;
                    let y = (v >> width) as u16;
                    right.circuit.constant((reference(x, y) >> bit) & 1 == 1)
                })
                .collect();
            let out = right.mux_tree(&ab, &leaves);
            right.circuit.output(out);
        }

        let verdict =
            check_equivalence(left.circuit(), right.circuit(), &CheckOptions::default()).unwrap();
        assert_eq!(verdict, Verdict::Equivalent);
    }

    fn block(v: u16) -> EmulatedCiphertextBlock {
        SPEC.from_complete(v)
    }

    #[test]
    fn add_matches_emulated_blocks() {
        check(
            |c, a, b| c.add(a, b),
            |x, y| block(x).wrapping_add(block(y)).raw_complete_bits(),
        );
    }

    #[test]
    fn sub_matches_emulated_blocks() {
        check(
            |c, a, b| c.sub(a, b),
            |x, y| block(x).wrapping_sub(block(y)).raw_complete_bits(),
        );
    }

    #[test]
    fn shl_matches_emulated_blocks() {
        check(
            |c, a, _| c.shl(a, 2),
            |x, _| block(x).wrapping_shl(2).raw_complete_bits(),
        );
    }

    #[test]
    fn mac_matches_emulated_blocks() {
        check(
            |c, a, b| c.mac(a, 4, b),
            |x, y| block(x).wrapping_mac(4, block(y)).raw_complete_bits(),
        );
    }

    #[test]
    fn cst_sub_matches_emulated_blocks() {
        let pt = SPEC.complete_plaintext_block_spec().from_message(5);
        check(
            |c, a, _| c.cst_sub(5, a),
            |x, _| pt.wrapping_sub_ct(block(x)).raw_complete_bits(),
        );
    }

    #[test]
    fn lut_matches_emulated_lookup() {
        let lut = Lut1::from_fn("clear_carry", SPEC, |b| b.mask_message());
        check(
            |c, a, _| c.lut1(&lut, a),
            |x, _| {
                lut.lookup(block(x), LookupCheck::AllowBothPadding)
                    .raw_complete_bits()
            },
        );
    }

    #[test]
    fn mul_pt_matches_emulated_blocks() {
        let width = SPEC.complete_size() as usize;
        let mut left = SymCircuit::new(SPEC);
        let a = free_ct(&mut left);
        let p = free_pt(&mut left);
        let out = left.mul_pt(&a, &p);
        for bit in &out.bits {
            left.circuit.output(*bit);
        }

        let mut right = SymCircuit::new(SPEC);
        let a = free_ct(&mut right);
        let p = free_pt(&mut right);
        let mut ap = a.bits.clone();
        ap.extend_from_slice(&p.bits);
        let pt_width = p.bits.len();
        for bit in 0..width {
            let leaves: Vec<Lit> = (0..1u32 << (width + pt_width))
                .map(|v| {
                    let x = (v & SPEC.complete_mask() as u32) as u16;
                    let y = (v >> width) as u16;
                    let pt = SPEC.matching_plaintext_block_spec().from_message(y);
                    let r = block(x).wrapping_mul_pt(pt).raw_complete_bits();
                    right.circuit.constant((r >> bit) & 1 == 1)
                })
                .collect();
            let out = right.mux_tree(&ap, &leaves);
            right.circuit.output(out);
        }

        let verdict =
            check_equivalence(left.circuit(), right.circuit(), &CheckOptions::default()).unwrap();
        assert_eq!(verdict, Verdict::Equivalent);
    }
}
