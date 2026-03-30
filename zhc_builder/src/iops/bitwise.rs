use zhc_crypto::integer_semantics::CiphertextSpec;
use zhc_langs::ioplang::Lut1Def;

use crate::builder::{Builder, Ciphertext};

/// Creates an IR for bitwise AND of two encrypted integers.
///
/// The returned [`Builder`] declares two ciphertext inputs and one ciphertext
/// output, where each output block is the bitwise AND of the corresponding
/// input blocks. The operation is applied independently to every block pair,
/// using a single programmable bootstrapping per block.
///
/// The `spec` parameter describes the integer encoding (bit-width, message
/// bits, carry bits) and determines the number of blocks in the
/// decomposition.
///
/// # Examples
///
/// ```rust,no_run
/// # use zhc_builder::{CiphertextSpec, bitwise_and};
/// # let spec = CiphertextSpec::new(16, 2, 2);
/// let builder = bitwise_and(spec);
/// let ir = builder.into_ir();
/// ```
pub fn bitwise_and(spec: CiphertextSpec) -> Builder {
    let mut builder = Builder::new(spec.block_spec());
    let src_a = builder.ciphertext_input(spec.int_size());
    let src_b = builder.ciphertext_input(spec.int_size());
    let res = builder.iop_bitwise(&src_a, &src_b, BwKind::And);
    builder.ciphertext_output(res);
    builder
}

/// Creates an IR for bitwise OR of two encrypted integers.
///
/// The returned [`Builder`] declares two ciphertext inputs and one ciphertext
/// output, where each output block is the bitwise OR of the corresponding
/// input blocks. The operation is applied independently to every block pair,
/// using a single programmable bootstrapping per block.
///
/// The `spec` parameter describes the integer encoding (bit-width, message
/// bits, carry bits) and determines the number of blocks in the
/// decomposition.
///
/// # Examples
///
/// ```rust,no_run
/// # use zhc_builder::{CiphertextSpec, bitwise_or};
/// # let spec = CiphertextSpec::new(16, 2, 2);
/// let builder = bitwise_or(spec);
/// let ir = builder.into_ir();
/// ```
pub fn bitwise_or(spec: CiphertextSpec) -> Builder {
    let mut builder = Builder::new(spec.block_spec());
    let src_a = builder.ciphertext_input(spec.int_size());
    let src_b = builder.ciphertext_input(spec.int_size());
    let res = builder.iop_bitwise(&src_a, &src_b, BwKind::Or);
    builder.ciphertext_output(res);
    builder
}

/// Creates an IR for bitwise XOR of two encrypted integers.
///
/// The returned [`Builder`] declares two ciphertext inputs and one ciphertext
/// output, where each output block is the bitwise XOR of the corresponding
/// input blocks. The operation is applied independently to every block pair,
/// using a single programmable bootstrapping per block.
///
/// The `spec` parameter describes the integer encoding (bit-width, message
/// bits, carry bits) and determines the number of blocks in the
/// decomposition.
///
/// # Examples
///
/// ```rust,no_run
/// # use zhc_builder::{CiphertextSpec, bitwise_xor};
/// # let spec = CiphertextSpec::new(16, 2, 2);
/// let builder = bitwise_xor(spec);
/// let ir = builder.into_ir();
/// ```
pub fn bitwise_xor(spec: CiphertextSpec) -> Builder {
    let mut builder = Builder::new(spec.block_spec());
    let src_a = builder.ciphertext_input(spec.int_size());
    let src_b = builder.ciphertext_input(spec.int_size());
    let res = builder.iop_bitwise(&src_a, &src_b, BwKind::Xor);
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
    /// # let mut builder = Builder::new(spec.block_spec());
    /// # let a = builder.ciphertext_input(spec.int_size());
    /// # let b = builder.ciphertext_input(spec.int_size());
    /// let result = builder.iop_bitwise(&a, &b, BwKind::Xor);
    /// ```
    pub fn iop_bitwise(&mut self, lhs: &Ciphertext, rhs: &Ciphertext, kind: BwKind) -> Ciphertext {
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
}

#[cfg(test)]
mod test {
    use super::*;
    use zhc_langs::ioplang::IopValue;
    use zhc_utils::assert_display_is;

    #[test]
    fn test_bw_and() {
        let spec = CiphertextSpec::new(64, 2, 2);
        let ir = bitwise_and(spec).into_ir();
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
                %67 = pbs<Protect, BwAnd>(%66);
                %68 = pack_ct<4>(%3, %35);
                %69 = pbs<Protect, BwAnd>(%68);
                %70 = pack_ct<4>(%4, %36);
                %71 = pbs<Protect, BwAnd>(%70);
                %72 = pack_ct<4>(%5, %37);
                %73 = pbs<Protect, BwAnd>(%72);
                %74 = pack_ct<4>(%6, %38);
                %75 = pbs<Protect, BwAnd>(%74);
                %76 = pack_ct<4>(%7, %39);
                %77 = pbs<Protect, BwAnd>(%76);
                %78 = pack_ct<4>(%8, %40);
                %79 = pbs<Protect, BwAnd>(%78);
                %80 = pack_ct<4>(%9, %41);
                %81 = pbs<Protect, BwAnd>(%80);
                %82 = pack_ct<4>(%10, %42);
                %83 = pbs<Protect, BwAnd>(%82);
                %84 = pack_ct<4>(%11, %43);
                %85 = pbs<Protect, BwAnd>(%84);
                %86 = pack_ct<4>(%12, %44);
                %87 = pbs<Protect, BwAnd>(%86);
                %88 = pack_ct<4>(%13, %45);
                %89 = pbs<Protect, BwAnd>(%88);
                %90 = pack_ct<4>(%14, %46);
                %91 = pbs<Protect, BwAnd>(%90);
                %92 = pack_ct<4>(%15, %47);
                %93 = pbs<Protect, BwAnd>(%92);
                %94 = pack_ct<4>(%16, %48);
                %95 = pbs<Protect, BwAnd>(%94);
                %96 = pack_ct<4>(%17, %49);
                %97 = pbs<Protect, BwAnd>(%96);
                %98 = pack_ct<4>(%18, %50);
                %99 = pbs<Protect, BwAnd>(%98);
                %100 = pack_ct<4>(%19, %51);
                %101 = pbs<Protect, BwAnd>(%100);
                %102 = pack_ct<4>(%20, %52);
                %103 = pbs<Protect, BwAnd>(%102);
                %104 = pack_ct<4>(%21, %53);
                %105 = pbs<Protect, BwAnd>(%104);
                %106 = pack_ct<4>(%22, %54);
                %107 = pbs<Protect, BwAnd>(%106);
                %108 = pack_ct<4>(%23, %55);
                %109 = pbs<Protect, BwAnd>(%108);
                %110 = pack_ct<4>(%24, %56);
                %111 = pbs<Protect, BwAnd>(%110);
                %112 = pack_ct<4>(%25, %57);
                %113 = pbs<Protect, BwAnd>(%112);
                %114 = pack_ct<4>(%26, %58);
                %115 = pbs<Protect, BwAnd>(%114);
                %116 = pack_ct<4>(%27, %59);
                %117 = pbs<Protect, BwAnd>(%116);
                %118 = pack_ct<4>(%28, %60);
                %119 = pbs<Protect, BwAnd>(%118);
                %120 = pack_ct<4>(%29, %61);
                %121 = pbs<Protect, BwAnd>(%120);
                %122 = pack_ct<4>(%30, %62);
                %123 = pbs<Protect, BwAnd>(%122);
                %124 = pack_ct<4>(%31, %63);
                %125 = pbs<Protect, BwAnd>(%124);
                %126 = pack_ct<4>(%32, %64);
                %127 = pbs<Protect, BwAnd>(%126);
                %128 = pack_ct<4>(%33, %65);
                %129 = pbs<Protect, BwAnd>(%128);
                %130 = decl_ct<64>();
                %131 = store_ct_block<0>(%67, %130);
                %132 = store_ct_block<1>(%69, %131);
                %133 = store_ct_block<2>(%71, %132);
                %134 = store_ct_block<3>(%73, %133);
                %135 = store_ct_block<4>(%75, %134);
                %136 = store_ct_block<5>(%77, %135);
                %137 = store_ct_block<6>(%79, %136);
                %138 = store_ct_block<7>(%81, %137);
                %139 = store_ct_block<8>(%83, %138);
                %140 = store_ct_block<9>(%85, %139);
                %141 = store_ct_block<10>(%87, %140);
                %142 = store_ct_block<11>(%89, %141);
                %143 = store_ct_block<12>(%91, %142);
                %144 = store_ct_block<13>(%93, %143);
                %145 = store_ct_block<14>(%95, %144);
                %146 = store_ct_block<15>(%97, %145);
                %147 = store_ct_block<16>(%99, %146);
                %148 = store_ct_block<17>(%101, %147);
                %149 = store_ct_block<18>(%103, %148);
                %150 = store_ct_block<19>(%105, %149);
                %151 = store_ct_block<20>(%107, %150);
                %152 = store_ct_block<21>(%109, %151);
                %153 = store_ct_block<22>(%111, %152);
                %154 = store_ct_block<23>(%113, %153);
                %155 = store_ct_block<24>(%115, %154);
                %156 = store_ct_block<25>(%117, %155);
                %157 = store_ct_block<26>(%119, %156);
                %158 = store_ct_block<27>(%121, %157);
                %159 = store_ct_block<28>(%123, %158);
                %160 = store_ct_block<29>(%125, %159);
                %161 = store_ct_block<30>(%127, %160);
                %162 = store_ct_block<31>(%129, %161);
                output<0>(%162);
            "#
        );
    }

    #[test]
    fn correctness_and() {
        fn semantic(inp: &[IopValue]) -> Vec<IopValue> {
            let [IopValue::Ciphertext(lhs), IopValue::Ciphertext(rhs)] = inp else {
                unreachable!()
            };
            vec![IopValue::Ciphertext(lhs.bitwise_and(*rhs))]
        }
        for size in (2..128).step_by(2) {
            bitwise_and(CiphertextSpec::new(size, 2, 2)).test_random(100, semantic);
        }
    }

    #[test]
    fn correctness_or() {
        fn semantic(inp: &[IopValue]) -> Vec<IopValue> {
            let [IopValue::Ciphertext(lhs), IopValue::Ciphertext(rhs)] = inp else {
                unreachable!()
            };
            vec![IopValue::Ciphertext(lhs.bitwise_or(*rhs))]
        }
        for size in (2..128).step_by(2) {
            bitwise_or(CiphertextSpec::new(size, 2, 2)).test_random(100, semantic);
        }
    }

    #[test]
    fn correctness_xor() {
        fn semantic(inp: &[IopValue]) -> Vec<IopValue> {
            let [IopValue::Ciphertext(lhs), IopValue::Ciphertext(rhs)] = inp else {
                unreachable!()
            };
            vec![IopValue::Ciphertext(lhs.bitwise_xor(*rhs))]
        }
        for size in (2..128).step_by(2) {
            bitwise_xor(CiphertextSpec::new(size, 2, 2)).test_random(100, semantic);
        }
    }
}
