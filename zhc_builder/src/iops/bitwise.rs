use zhc_crypto::integer_semantics::CiphertextSpec;
use zhc_langs::ioplang::Lut1Def;

use crate::builder::{Builder, Ciphertext};

/// Creates an IR for bitwise AND of two encrypted integers.
///
/// Convenience wrapper that calls [`Builder::iop_bitwise`] with [`BwKind::And`].
/// See that method for details.
pub fn bitwise_and(spec: CiphertextSpec) -> Builder {
    let builder = Builder::new(spec.block_spec());
    let src_a = builder.ciphertext_input(spec.int_size());
    let src_b = builder.ciphertext_input(spec.int_size());
    let res = builder.iop_bitwise(&src_a, &src_b, BwKind::And);
    builder.ciphertext_output(res);
    builder
}

/// Creates an IR for bitwise OR of two encrypted integers.
///
/// Convenience wrapper that calls [`Builder::iop_bitwise`] with [`BwKind::Or`].
/// See that method for details.
pub fn bitwise_or(spec: CiphertextSpec) -> Builder {
    let builder = Builder::new(spec.block_spec());
    let src_a = builder.ciphertext_input(spec.int_size());
    let src_b = builder.ciphertext_input(spec.int_size());
    let res = builder.iop_bitwise(&src_a, &src_b, BwKind::Or);
    builder.ciphertext_output(res);
    builder
}

/// Creates an IR for bitwise XOR of two encrypted integers.
///
/// Convenience wrapper that calls [`Builder::iop_bitwise`] with [`BwKind::Xor`].
/// See that method for details.
///
/// # Examples
///
/// ```rust,no_run
/// # use zhc_builder::{CiphertextSpec, bitwise_xor};
/// # let spec = CiphertextSpec::new(16, 2, 2);
/// let builder = bitwise_xor(spec);
/// let ir = builder.optimize_ir();
/// ```
pub fn bitwise_xor(spec: CiphertextSpec) -> Builder {
    let builder = Builder::new(spec.block_spec());
    let src_a = builder.ciphertext_input(spec.int_size());
    let src_b = builder.ciphertext_input(spec.int_size());
    let res = builder.iop_bitwise(&src_a, &src_b, BwKind::Xor);
    builder.ciphertext_output(res);
    builder
}

/// Creates an IR for bitwise NOT of an encrypted integer.
///
/// Convenience wrapper that calls [`Builder::iop_bitwise_inv`].
/// See that method for details.
///
/// # Examples
///
/// ```rust,no_run
/// # use zhc_builder::{CiphertextSpec, bitwise_inv};
/// # let spec = CiphertextSpec::new(16, 2, 2);
/// let builder = bitwise_inv(spec);
/// let ir = builder.optimize_ir();
/// ```
pub fn bitwise_inv(spec: CiphertextSpec) -> Builder {
    let builder = Builder::new(spec.block_spec());
    let src_ct = builder.ciphertext_input(spec.int_size());
    let res = builder.iop_bitwise_inv(&src_ct);
    builder.ciphertext_output(res);
    builder
}

/// The kind of bitwise operation to apply block-wise.
pub enum BwKind {
    /// Bitwise AND — each output block is `a & b`.
    And,
    /// Bitwise OR — each output block is `a | b`.
    Or,
    /// Bitwise XOR — each output block is `a ^ b`.
    Xor,
}

impl BwKind {
    fn lut(&self) -> Lut1Def {
        match self {
            BwKind::And => Lut1Def::BwAnd,
            BwKind::Or => Lut1Def::BwOr,
            BwKind::Xor => Lut1Def::BwXor,
        }
    }
}

impl Builder {
    /// Applies a block-wise bitwise operation on two encrypted integers.
    ///
    /// Both operands must have the same block decomposition; the builder
    /// panics if their lengths differ.
    ///
    /// # Panics
    ///
    /// Panics if `lhs` and `rhs` have different numbers of blocks.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_builder::{CiphertextSpec, Builder, BwKind};
    /// # let spec = CiphertextSpec::new(16, 2, 2);
    /// # let builder = Builder::new(spec.block_spec());
    /// # let a = builder.ciphertext_input(spec.int_size());
    /// # let b = builder.ciphertext_input(spec.int_size());
    /// let result = builder.iop_bitwise(&a, &b, BwKind::Xor);
    /// ```
    pub fn iop_bitwise(&self, lhs: &Ciphertext, rhs: &Ciphertext, kind: BwKind) -> Ciphertext {
        // If both operands are bits, XOR is PBS-free: sums stay pending until a
        // consumer reduces them.
        if matches!(kind, BwKind::Xor)
            && lhs.spec().int_size() == rhs.spec().int_size()
            && (self.registered_bits(lhs).is_some() || self.registered_bits(rhs).is_some())
        {
            let out: Vec<_> = self
                .bits_of(lhs)
                .into_iter()
                .zip(self.bits_of(rhs))
                .map(|(a, b)| self.bit_xor(a, b))
                .collect();
            let joined = self.bits_join(&out, lhs.spec().int_size());
            self.register_bits(&joined, &out);
            return joined;
        }
        let lhs_blocks = self.ciphertext_split(lhs);
        let rhs_blocks = self.ciphertext_split(rhs);
        let res = self.vector_zip_then_lookup(
            lhs_blocks,
            rhs_blocks,
            kind.lut(),
            crate::ExtensionBehavior::Panic,
        );
        self.ciphertext_join(res, None)
    }

    /// Applies a bitwise NOT operation on an encrypted integer.
    ///
    /// Computes the complement by subtracting each block from a plaintext mask of all ones.
    /// This avoids PBS entirely, making it a cheap operation suitable for use in subtraction
    /// (two's complement) and other bitwise pipelines.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_builder::{CiphertextSpec, Builder};
    /// # let spec = CiphertextSpec::new(16, 2, 2);
    /// # let builder = Builder::new(spec.block_spec());
    /// # let a = builder.ciphertext_input(spec.int_size());
    /// let result = builder.iop_bitwise_inv(&a);
    /// ```
    pub fn iop_bitwise_inv(&self, ct: &Ciphertext) -> Ciphertext {
        let ct_blocks = self.ciphertext_split(ct);
        // create a message full of 1
        let allone = self.block_let_plaintext((1 << self.spec().message_size()) - 1);
        let res = ct_blocks
            .iter()
            .map(|m| self.block_plaintext_sub(allone, m))
            .collect::<Vec<_>>();
        self.ciphertext_join(res, None)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use zhc_langs::ioplang::IopValue;
    use zhc_utils::assert_display_is;

    #[test]
    fn test_bw_and() {
        let spec = CiphertextSpec::new(64, 2, 2);
        let ir = bitwise_and(spec).optimize_ir();
        assert_display_is!(
            ir.format().with_walker(zhc_ir::PrintWalker::Linear),
            r#"
                %0 = input_ciphertext<0, 64>();
                %1 = input_ciphertext<1, 64>();
                %2 = extract_ct_block<0>(%0);
                %3 = extract_ct_block<1>(%0);
                %4 = extract_ct_block<2>(%0);
                %5 = extract_ct_block<3>(%0);
                %6 = extract_ct_block<4>(%0);
                %7 = extract_ct_block<5>(%0);
                %8 = extract_ct_block<6>(%0);
                %9 = extract_ct_block<7>(%0);
                %10 = extract_ct_block<8>(%0);
                %11 = extract_ct_block<9>(%0);
                %12 = extract_ct_block<10>(%0);
                %13 = extract_ct_block<11>(%0);
                %14 = extract_ct_block<12>(%0);
                %15 = extract_ct_block<13>(%0);
                %16 = extract_ct_block<14>(%0);
                %17 = extract_ct_block<15>(%0);
                %18 = extract_ct_block<16>(%0);
                %19 = extract_ct_block<17>(%0);
                %20 = extract_ct_block<18>(%0);
                %21 = extract_ct_block<19>(%0);
                %22 = extract_ct_block<20>(%0);
                %23 = extract_ct_block<21>(%0);
                %24 = extract_ct_block<22>(%0);
                %25 = extract_ct_block<23>(%0);
                %26 = extract_ct_block<24>(%0);
                %27 = extract_ct_block<25>(%0);
                %28 = extract_ct_block<26>(%0);
                %29 = extract_ct_block<27>(%0);
                %30 = extract_ct_block<28>(%0);
                %31 = extract_ct_block<29>(%0);
                %32 = extract_ct_block<30>(%0);
                %33 = extract_ct_block<31>(%0);
                %34 = extract_ct_block<0>(%1);
                %35 = extract_ct_block<1>(%1);
                %36 = extract_ct_block<2>(%1);
                %37 = extract_ct_block<3>(%1);
                %38 = extract_ct_block<4>(%1);
                %39 = extract_ct_block<5>(%1);
                %40 = extract_ct_block<6>(%1);
                %41 = extract_ct_block<7>(%1);
                %42 = extract_ct_block<8>(%1);
                %43 = extract_ct_block<9>(%1);
                %44 = extract_ct_block<10>(%1);
                %45 = extract_ct_block<11>(%1);
                %46 = extract_ct_block<12>(%1);
                %47 = extract_ct_block<13>(%1);
                %48 = extract_ct_block<14>(%1);
                %49 = extract_ct_block<15>(%1);
                %50 = extract_ct_block<16>(%1);
                %51 = extract_ct_block<17>(%1);
                %52 = extract_ct_block<18>(%1);
                %53 = extract_ct_block<19>(%1);
                %54 = extract_ct_block<20>(%1);
                %55 = extract_ct_block<21>(%1);
                %56 = extract_ct_block<22>(%1);
                %57 = extract_ct_block<23>(%1);
                %58 = extract_ct_block<24>(%1);
                %59 = extract_ct_block<25>(%1);
                %60 = extract_ct_block<26>(%1);
                %61 = extract_ct_block<27>(%1);
                %62 = extract_ct_block<28>(%1);
                %63 = extract_ct_block<29>(%1);
                %64 = extract_ct_block<30>(%1);
                %65 = extract_ct_block<31>(%1);
                %66 = pack_ct<4>(%2, %34);
                %67 = pbs<Protect, Lut1("BwAnd")>(%66);
                %68 = pack_ct<4>(%3, %35);
                %69 = pbs<Protect, Lut1("BwAnd")>(%68);
                %70 = pack_ct<4>(%4, %36);
                %71 = pbs<Protect, Lut1("BwAnd")>(%70);
                %72 = pack_ct<4>(%5, %37);
                %73 = pbs<Protect, Lut1("BwAnd")>(%72);
                %74 = pack_ct<4>(%6, %38);
                %75 = pbs<Protect, Lut1("BwAnd")>(%74);
                %76 = pack_ct<4>(%7, %39);
                %77 = pbs<Protect, Lut1("BwAnd")>(%76);
                %78 = pack_ct<4>(%8, %40);
                %79 = pbs<Protect, Lut1("BwAnd")>(%78);
                %80 = pack_ct<4>(%9, %41);
                %81 = pbs<Protect, Lut1("BwAnd")>(%80);
                %82 = pack_ct<4>(%10, %42);
                %83 = pbs<Protect, Lut1("BwAnd")>(%82);
                %84 = pack_ct<4>(%11, %43);
                %85 = pbs<Protect, Lut1("BwAnd")>(%84);
                %86 = pack_ct<4>(%12, %44);
                %87 = pbs<Protect, Lut1("BwAnd")>(%86);
                %88 = pack_ct<4>(%13, %45);
                %89 = pbs<Protect, Lut1("BwAnd")>(%88);
                %90 = pack_ct<4>(%14, %46);
                %91 = pbs<Protect, Lut1("BwAnd")>(%90);
                %92 = pack_ct<4>(%15, %47);
                %93 = pbs<Protect, Lut1("BwAnd")>(%92);
                %94 = pack_ct<4>(%16, %48);
                %95 = pbs<Protect, Lut1("BwAnd")>(%94);
                %96 = pack_ct<4>(%17, %49);
                %97 = pbs<Protect, Lut1("BwAnd")>(%96);
                %98 = pack_ct<4>(%18, %50);
                %99 = pbs<Protect, Lut1("BwAnd")>(%98);
                %100 = pack_ct<4>(%19, %51);
                %101 = pbs<Protect, Lut1("BwAnd")>(%100);
                %102 = pack_ct<4>(%20, %52);
                %103 = pbs<Protect, Lut1("BwAnd")>(%102);
                %104 = pack_ct<4>(%21, %53);
                %105 = pbs<Protect, Lut1("BwAnd")>(%104);
                %106 = pack_ct<4>(%22, %54);
                %107 = pbs<Protect, Lut1("BwAnd")>(%106);
                %108 = pack_ct<4>(%23, %55);
                %109 = pbs<Protect, Lut1("BwAnd")>(%108);
                %110 = pack_ct<4>(%24, %56);
                %111 = pbs<Protect, Lut1("BwAnd")>(%110);
                %112 = pack_ct<4>(%25, %57);
                %113 = pbs<Protect, Lut1("BwAnd")>(%112);
                %114 = pack_ct<4>(%26, %58);
                %115 = pbs<Protect, Lut1("BwAnd")>(%114);
                %116 = pack_ct<4>(%27, %59);
                %117 = pbs<Protect, Lut1("BwAnd")>(%116);
                %118 = pack_ct<4>(%28, %60);
                %119 = pbs<Protect, Lut1("BwAnd")>(%118);
                %120 = pack_ct<4>(%29, %61);
                %121 = pbs<Protect, Lut1("BwAnd")>(%120);
                %122 = pack_ct<4>(%30, %62);
                %123 = pbs<Protect, Lut1("BwAnd")>(%122);
                %124 = pack_ct<4>(%31, %63);
                %125 = pbs<Protect, Lut1("BwAnd")>(%124);
                %126 = pack_ct<4>(%32, %64);
                %127 = pbs<Protect, Lut1("BwAnd")>(%126);
                %128 = pack_ct<4>(%33, %65);
                %129 = pbs<Protect, Lut1("BwAnd")>(%128);
                %130 = decl_ct<64>();
                %164 = store_ct_block<0>(%67, %130);
                %165 = store_ct_block<1>(%69, %164);
                %166 = store_ct_block<2>(%71, %165);
                %167 = store_ct_block<3>(%73, %166);
                %168 = store_ct_block<4>(%75, %167);
                %169 = store_ct_block<5>(%77, %168);
                %170 = store_ct_block<6>(%79, %169);
                %171 = store_ct_block<7>(%81, %170);
                %172 = store_ct_block<8>(%83, %171);
                %173 = store_ct_block<9>(%85, %172);
                %174 = store_ct_block<10>(%87, %173);
                %175 = store_ct_block<11>(%89, %174);
                %176 = store_ct_block<12>(%91, %175);
                %177 = store_ct_block<13>(%93, %176);
                %178 = store_ct_block<14>(%95, %177);
                %179 = store_ct_block<15>(%97, %178);
                %180 = store_ct_block<16>(%99, %179);
                %181 = store_ct_block<17>(%101, %180);
                %182 = store_ct_block<18>(%103, %181);
                %183 = store_ct_block<19>(%105, %182);
                %184 = store_ct_block<20>(%107, %183);
                %185 = store_ct_block<21>(%109, %184);
                %186 = store_ct_block<22>(%111, %185);
                %187 = store_ct_block<23>(%113, %186);
                %188 = store_ct_block<24>(%115, %187);
                %189 = store_ct_block<25>(%117, %188);
                %190 = store_ct_block<26>(%119, %189);
                %191 = store_ct_block<27>(%121, %190);
                %192 = store_ct_block<28>(%123, %191);
                %193 = store_ct_block<29>(%125, %192);
                %194 = store_ct_block<30>(%127, %193);
                %195 = store_ct_block<31>(%129, %194);
                output<0>(%195);
            "#
        );
    }

    #[test]
    fn correctness_and() {
        fn semantic(inp: &[IopValue]) -> Option<Vec<IopValue>> {
            let [IopValue::Ciphertext(lhs), IopValue::Ciphertext(rhs)] = inp else {
                unreachable!()
            };
            Some(vec![IopValue::Ciphertext(lhs.bitwise_and(*rhs))])
        }
        for size in (2..128).step_by(2) {
            bitwise_and(CiphertextSpec::new(size, 2, 2)).test_random(100, semantic);
        }
    }

    #[test]
    fn correctness_or() {
        fn semantic(inp: &[IopValue]) -> Option<Vec<IopValue>> {
            let [IopValue::Ciphertext(lhs), IopValue::Ciphertext(rhs)] = inp else {
                unreachable!()
            };
            Some(vec![IopValue::Ciphertext(lhs.bitwise_or(*rhs))])
        }
        for size in (2..128).step_by(2) {
            bitwise_or(CiphertextSpec::new(size, 2, 2)).test_random(100, semantic);
        }
    }

    #[test]
    fn correctness_xor() {
        fn semantic(inp: &[IopValue]) -> Option<Vec<IopValue>> {
            let [IopValue::Ciphertext(lhs), IopValue::Ciphertext(rhs)] = inp else {
                unreachable!()
            };
            Some(vec![IopValue::Ciphertext(lhs.bitwise_xor(*rhs))])
        }
        for size in (2..128).step_by(2) {
            bitwise_xor(CiphertextSpec::new(size, 2, 2)).test_random(100, semantic);
        }
    }

    #[test]
    fn correctness_inv() {
        fn semantic(inp: &[IopValue]) -> Option<Vec<IopValue>> {
            let [IopValue::Ciphertext(ct)] = inp else {
                unreachable!()
            };
            Some(vec![IopValue::Ciphertext(ct.bitwise_not())])
        }
        for size in (2..128).step_by(2) {
            bitwise_inv(CiphertextSpec::new(size, 2, 2)).test_random(100, semantic);
        }
    }
}
