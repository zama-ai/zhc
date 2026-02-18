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
    let src_a = builder.declare_ciphertext_input(spec.int_size());
    let src_b = builder.declare_ciphertext_input(spec.int_size());
    let res = builder.iop_bitwise(&src_a, &src_b, BwKind::And);
    builder.declare_ciphertext_output(res);
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
    let src_a = builder.declare_ciphertext_input(spec.int_size());
    let src_b = builder.declare_ciphertext_input(spec.int_size());
    let res = builder.iop_bitwise(&src_a, &src_b, BwKind::Or);
    builder.declare_ciphertext_output(res);
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
    let src_a = builder.declare_ciphertext_input(spec.int_size());
    let src_b = builder.declare_ciphertext_input(spec.int_size());
    let res = builder.iop_bitwise(&src_a, &src_b, BwKind::Xor);
    builder.declare_ciphertext_output(res);
    builder
}

/// The kind of bitwise operation to apply block-wise.
pub enum BwKind {
    And,
    Or,
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
    /// # let a = builder.declare_ciphertext_input(spec.int_size());
    /// # let b = builder.declare_ciphertext_input(spec.int_size());
    /// let result = builder.iop_bitwise(&a, &b, BwKind::Xor);
    /// ```
    pub fn iop_bitwise(&mut self, lhs: &Ciphertext, rhs: &Ciphertext, kind: BwKind) -> Ciphertext {
        let lhs_blocks = self.split_ciphertext(lhs);
        let rhs_blocks = self.split_ciphertext(rhs);
        let res = self.vector_zip_then_lookup(
            lhs_blocks,
            rhs_blocks,
            kind.lut(),
            crate::ExtensionBehavior::Panic,
        );
        self.join_ciphertext(res)
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
            %0 : CtInt = input<0, CtInt>();
            %1 : CtInt = input<1, CtInt>();
            %2 : CtBlock = extract_ct_block<0>(%0 : CtInt);
            %3 : CtBlock = extract_ct_block<1>(%0 : CtInt);
            %4 : CtBlock = extract_ct_block<2>(%0 : CtInt);
            %5 : CtBlock = extract_ct_block<3>(%0 : CtInt);
            %6 : CtBlock = extract_ct_block<4>(%0 : CtInt);
            %7 : CtBlock = extract_ct_block<5>(%0 : CtInt);
            %8 : CtBlock = extract_ct_block<6>(%0 : CtInt);
            %9 : CtBlock = extract_ct_block<7>(%0 : CtInt);
            %10 : CtBlock = extract_ct_block<8>(%0 : CtInt);
            %11 : CtBlock = extract_ct_block<9>(%0 : CtInt);
            %12 : CtBlock = extract_ct_block<10>(%0 : CtInt);
            %13 : CtBlock = extract_ct_block<11>(%0 : CtInt);
            %14 : CtBlock = extract_ct_block<12>(%0 : CtInt);
            %15 : CtBlock = extract_ct_block<13>(%0 : CtInt);
            %16 : CtBlock = extract_ct_block<14>(%0 : CtInt);
            %17 : CtBlock = extract_ct_block<15>(%0 : CtInt);
            %18 : CtBlock = extract_ct_block<16>(%0 : CtInt);
            %19 : CtBlock = extract_ct_block<17>(%0 : CtInt);
            %20 : CtBlock = extract_ct_block<18>(%0 : CtInt);
            %21 : CtBlock = extract_ct_block<19>(%0 : CtInt);
            %22 : CtBlock = extract_ct_block<20>(%0 : CtInt);
            %23 : CtBlock = extract_ct_block<21>(%0 : CtInt);
            %24 : CtBlock = extract_ct_block<22>(%0 : CtInt);
            %25 : CtBlock = extract_ct_block<23>(%0 : CtInt);
            %26 : CtBlock = extract_ct_block<24>(%0 : CtInt);
            %27 : CtBlock = extract_ct_block<25>(%0 : CtInt);
            %28 : CtBlock = extract_ct_block<26>(%0 : CtInt);
            %29 : CtBlock = extract_ct_block<27>(%0 : CtInt);
            %30 : CtBlock = extract_ct_block<28>(%0 : CtInt);
            %31 : CtBlock = extract_ct_block<29>(%0 : CtInt);
            %32 : CtBlock = extract_ct_block<30>(%0 : CtInt);
            %33 : CtBlock = extract_ct_block<31>(%0 : CtInt);
            %34 : CtBlock = extract_ct_block<0>(%1 : CtInt);
            %35 : CtBlock = extract_ct_block<1>(%1 : CtInt);
            %36 : CtBlock = extract_ct_block<2>(%1 : CtInt);
            %37 : CtBlock = extract_ct_block<3>(%1 : CtInt);
            %38 : CtBlock = extract_ct_block<4>(%1 : CtInt);
            %39 : CtBlock = extract_ct_block<5>(%1 : CtInt);
            %40 : CtBlock = extract_ct_block<6>(%1 : CtInt);
            %41 : CtBlock = extract_ct_block<7>(%1 : CtInt);
            %42 : CtBlock = extract_ct_block<8>(%1 : CtInt);
            %43 : CtBlock = extract_ct_block<9>(%1 : CtInt);
            %44 : CtBlock = extract_ct_block<10>(%1 : CtInt);
            %45 : CtBlock = extract_ct_block<11>(%1 : CtInt);
            %46 : CtBlock = extract_ct_block<12>(%1 : CtInt);
            %47 : CtBlock = extract_ct_block<13>(%1 : CtInt);
            %48 : CtBlock = extract_ct_block<14>(%1 : CtInt);
            %49 : CtBlock = extract_ct_block<15>(%1 : CtInt);
            %50 : CtBlock = extract_ct_block<16>(%1 : CtInt);
            %51 : CtBlock = extract_ct_block<17>(%1 : CtInt);
            %52 : CtBlock = extract_ct_block<18>(%1 : CtInt);
            %53 : CtBlock = extract_ct_block<19>(%1 : CtInt);
            %54 : CtBlock = extract_ct_block<20>(%1 : CtInt);
            %55 : CtBlock = extract_ct_block<21>(%1 : CtInt);
            %56 : CtBlock = extract_ct_block<22>(%1 : CtInt);
            %57 : CtBlock = extract_ct_block<23>(%1 : CtInt);
            %58 : CtBlock = extract_ct_block<24>(%1 : CtInt);
            %59 : CtBlock = extract_ct_block<25>(%1 : CtInt);
            %60 : CtBlock = extract_ct_block<26>(%1 : CtInt);
            %61 : CtBlock = extract_ct_block<27>(%1 : CtInt);
            %62 : CtBlock = extract_ct_block<28>(%1 : CtInt);
            %63 : CtBlock = extract_ct_block<29>(%1 : CtInt);
            %64 : CtBlock = extract_ct_block<30>(%1 : CtInt);
            %65 : CtBlock = extract_ct_block<31>(%1 : CtInt);
            %66 : CtBlock = pack_ct<4>(%2 : CtBlock, %34 : CtBlock);
            %67 : CtBlock = pbs<BwAnd>(%66 : CtBlock);
            %68 : CtBlock = pack_ct<4>(%3 : CtBlock, %35 : CtBlock);
            %69 : CtBlock = pbs<BwAnd>(%68 : CtBlock);
            %70 : CtBlock = pack_ct<4>(%4 : CtBlock, %36 : CtBlock);
            %71 : CtBlock = pbs<BwAnd>(%70 : CtBlock);
            %72 : CtBlock = pack_ct<4>(%5 : CtBlock, %37 : CtBlock);
            %73 : CtBlock = pbs<BwAnd>(%72 : CtBlock);
            %74 : CtBlock = pack_ct<4>(%6 : CtBlock, %38 : CtBlock);
            %75 : CtBlock = pbs<BwAnd>(%74 : CtBlock);
            %76 : CtBlock = pack_ct<4>(%7 : CtBlock, %39 : CtBlock);
            %77 : CtBlock = pbs<BwAnd>(%76 : CtBlock);
            %78 : CtBlock = pack_ct<4>(%8 : CtBlock, %40 : CtBlock);
            %79 : CtBlock = pbs<BwAnd>(%78 : CtBlock);
            %80 : CtBlock = pack_ct<4>(%9 : CtBlock, %41 : CtBlock);
            %81 : CtBlock = pbs<BwAnd>(%80 : CtBlock);
            %82 : CtBlock = pack_ct<4>(%10 : CtBlock, %42 : CtBlock);
            %83 : CtBlock = pbs<BwAnd>(%82 : CtBlock);
            %84 : CtBlock = pack_ct<4>(%11 : CtBlock, %43 : CtBlock);
            %85 : CtBlock = pbs<BwAnd>(%84 : CtBlock);
            %86 : CtBlock = pack_ct<4>(%12 : CtBlock, %44 : CtBlock);
            %87 : CtBlock = pbs<BwAnd>(%86 : CtBlock);
            %88 : CtBlock = pack_ct<4>(%13 : CtBlock, %45 : CtBlock);
            %89 : CtBlock = pbs<BwAnd>(%88 : CtBlock);
            %90 : CtBlock = pack_ct<4>(%14 : CtBlock, %46 : CtBlock);
            %91 : CtBlock = pbs<BwAnd>(%90 : CtBlock);
            %92 : CtBlock = pack_ct<4>(%15 : CtBlock, %47 : CtBlock);
            %93 : CtBlock = pbs<BwAnd>(%92 : CtBlock);
            %94 : CtBlock = pack_ct<4>(%16 : CtBlock, %48 : CtBlock);
            %95 : CtBlock = pbs<BwAnd>(%94 : CtBlock);
            %96 : CtBlock = pack_ct<4>(%17 : CtBlock, %49 : CtBlock);
            %97 : CtBlock = pbs<BwAnd>(%96 : CtBlock);
            %98 : CtBlock = pack_ct<4>(%18 : CtBlock, %50 : CtBlock);
            %99 : CtBlock = pbs<BwAnd>(%98 : CtBlock);
            %100 : CtBlock = pack_ct<4>(%19 : CtBlock, %51 : CtBlock);
            %101 : CtBlock = pbs<BwAnd>(%100 : CtBlock);
            %102 : CtBlock = pack_ct<4>(%20 : CtBlock, %52 : CtBlock);
            %103 : CtBlock = pbs<BwAnd>(%102 : CtBlock);
            %104 : CtBlock = pack_ct<4>(%21 : CtBlock, %53 : CtBlock);
            %105 : CtBlock = pbs<BwAnd>(%104 : CtBlock);
            %106 : CtBlock = pack_ct<4>(%22 : CtBlock, %54 : CtBlock);
            %107 : CtBlock = pbs<BwAnd>(%106 : CtBlock);
            %108 : CtBlock = pack_ct<4>(%23 : CtBlock, %55 : CtBlock);
            %109 : CtBlock = pbs<BwAnd>(%108 : CtBlock);
            %110 : CtBlock = pack_ct<4>(%24 : CtBlock, %56 : CtBlock);
            %111 : CtBlock = pbs<BwAnd>(%110 : CtBlock);
            %112 : CtBlock = pack_ct<4>(%25 : CtBlock, %57 : CtBlock);
            %113 : CtBlock = pbs<BwAnd>(%112 : CtBlock);
            %114 : CtBlock = pack_ct<4>(%26 : CtBlock, %58 : CtBlock);
            %115 : CtBlock = pbs<BwAnd>(%114 : CtBlock);
            %116 : CtBlock = pack_ct<4>(%27 : CtBlock, %59 : CtBlock);
            %117 : CtBlock = pbs<BwAnd>(%116 : CtBlock);
            %118 : CtBlock = pack_ct<4>(%28 : CtBlock, %60 : CtBlock);
            %119 : CtBlock = pbs<BwAnd>(%118 : CtBlock);
            %120 : CtBlock = pack_ct<4>(%29 : CtBlock, %61 : CtBlock);
            %121 : CtBlock = pbs<BwAnd>(%120 : CtBlock);
            %122 : CtBlock = pack_ct<4>(%30 : CtBlock, %62 : CtBlock);
            %123 : CtBlock = pbs<BwAnd>(%122 : CtBlock);
            %124 : CtBlock = pack_ct<4>(%31 : CtBlock, %63 : CtBlock);
            %125 : CtBlock = pbs<BwAnd>(%124 : CtBlock);
            %126 : CtBlock = pack_ct<4>(%32 : CtBlock, %64 : CtBlock);
            %127 : CtBlock = pbs<BwAnd>(%126 : CtBlock);
            %128 : CtBlock = pack_ct<4>(%33 : CtBlock, %65 : CtBlock);
            %129 : CtBlock = pbs<BwAnd>(%128 : CtBlock);
            %130 : CtInt = decl_ct();
            %131 : CtInt = store_ct_block<0>(%67 : CtBlock, %130 : CtInt);
            %132 : CtInt = store_ct_block<1>(%69 : CtBlock, %131 : CtInt);
            %133 : CtInt = store_ct_block<2>(%71 : CtBlock, %132 : CtInt);
            %134 : CtInt = store_ct_block<3>(%73 : CtBlock, %133 : CtInt);
            %135 : CtInt = store_ct_block<4>(%75 : CtBlock, %134 : CtInt);
            %136 : CtInt = store_ct_block<5>(%77 : CtBlock, %135 : CtInt);
            %137 : CtInt = store_ct_block<6>(%79 : CtBlock, %136 : CtInt);
            %138 : CtInt = store_ct_block<7>(%81 : CtBlock, %137 : CtInt);
            %139 : CtInt = store_ct_block<8>(%83 : CtBlock, %138 : CtInt);
            %140 : CtInt = store_ct_block<9>(%85 : CtBlock, %139 : CtInt);
            %141 : CtInt = store_ct_block<10>(%87 : CtBlock, %140 : CtInt);
            %142 : CtInt = store_ct_block<11>(%89 : CtBlock, %141 : CtInt);
            %143 : CtInt = store_ct_block<12>(%91 : CtBlock, %142 : CtInt);
            %144 : CtInt = store_ct_block<13>(%93 : CtBlock, %143 : CtInt);
            %145 : CtInt = store_ct_block<14>(%95 : CtBlock, %144 : CtInt);
            %146 : CtInt = store_ct_block<15>(%97 : CtBlock, %145 : CtInt);
            %147 : CtInt = store_ct_block<16>(%99 : CtBlock, %146 : CtInt);
            %148 : CtInt = store_ct_block<17>(%101 : CtBlock, %147 : CtInt);
            %149 : CtInt = store_ct_block<18>(%103 : CtBlock, %148 : CtInt);
            %150 : CtInt = store_ct_block<19>(%105 : CtBlock, %149 : CtInt);
            %151 : CtInt = store_ct_block<20>(%107 : CtBlock, %150 : CtInt);
            %152 : CtInt = store_ct_block<21>(%109 : CtBlock, %151 : CtInt);
            %153 : CtInt = store_ct_block<22>(%111 : CtBlock, %152 : CtInt);
            %154 : CtInt = store_ct_block<23>(%113 : CtBlock, %153 : CtInt);
            %155 : CtInt = store_ct_block<24>(%115 : CtBlock, %154 : CtInt);
            %156 : CtInt = store_ct_block<25>(%117 : CtBlock, %155 : CtInt);
            %157 : CtInt = store_ct_block<26>(%119 : CtBlock, %156 : CtInt);
            %158 : CtInt = store_ct_block<27>(%121 : CtBlock, %157 : CtInt);
            %159 : CtInt = store_ct_block<28>(%123 : CtBlock, %158 : CtInt);
            %160 : CtInt = store_ct_block<29>(%125 : CtBlock, %159 : CtInt);
            %161 : CtInt = store_ct_block<30>(%127 : CtBlock, %160 : CtInt);
            %162 : CtInt = store_ct_block<31>(%129 : CtBlock, %161 : CtInt);
            output<0, CtInt>(%162 : CtInt);
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
