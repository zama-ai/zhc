use hc_crypto::integer_semantics::CiphertextSpec;
use hc_ir::IR;
use hc_langs::ioplang::{IopLang, Lut1Def};
use hc_utils::iter::CollectInSmallVec;

use crate::builder::{Builder, Ciphertext};

pub fn if_then_zero(spec: CiphertextSpec) -> IR<IopLang> {
    let builder = Builder::new(spec.block_spec());

    let src = builder.eint_input(spec.int_size());
    let cond = builder.eint_input(spec.block_spec().message_size() as u16);

    let output_blocks = src
        .blocks()
        .iter()
        .map(|b| {
            let out = builder.block_pack_ct(&cond.blocks()[0], b);
            builder.block_pbs(&out, Lut1Def::IfFalseZeroed)
        })
        .cosvec();

    builder.eint_output(Ciphertext::from_blocks(output_blocks));

    // builder.dump_eval_panic(svec![IopValue::Ciphertext(spec.from_int(6)),
    // IopValue::Ciphertext(spec.from_int(1))]);

    builder.into_ir()
}

#[cfg(test)]
mod test {
    use super::*;
    use hc_utils::assert_display_is;

    #[test]
    fn test_if_then_zero() {
        let spec = CiphertextSpec::new(16, 2, 2);
        let ir = if_then_zero(spec);
        assert_display_is!(
            ir.format(),
            r#"
            %0 : CtInt = input<0, CtInt>();
            %9 : CtInt = input<1, CtInt>();
            %27 : CtInt = zero_ct();
            %1 : CtBlock = extract_ct_block<0>(%0 : CtInt);
            %2 : CtBlock = extract_ct_block<1>(%0 : CtInt);
            %3 : CtBlock = extract_ct_block<2>(%0 : CtInt);
            %4 : CtBlock = extract_ct_block<3>(%0 : CtInt);
            %5 : CtBlock = extract_ct_block<4>(%0 : CtInt);
            %6 : CtBlock = extract_ct_block<5>(%0 : CtInt);
            %7 : CtBlock = extract_ct_block<6>(%0 : CtInt);
            %8 : CtBlock = extract_ct_block<7>(%0 : CtInt);
            %10 : CtBlock = extract_ct_block<0>(%9 : CtInt);
            %11 : CtBlock = pack_ct<4>(%10 : CtBlock, %1 : CtBlock);
            %13 : CtBlock = pack_ct<4>(%10 : CtBlock, %2 : CtBlock);
            %15 : CtBlock = pack_ct<4>(%10 : CtBlock, %3 : CtBlock);
            %17 : CtBlock = pack_ct<4>(%10 : CtBlock, %4 : CtBlock);
            %19 : CtBlock = pack_ct<4>(%10 : CtBlock, %5 : CtBlock);
            %21 : CtBlock = pack_ct<4>(%10 : CtBlock, %6 : CtBlock);
            %23 : CtBlock = pack_ct<4>(%10 : CtBlock, %7 : CtBlock);
            %25 : CtBlock = pack_ct<4>(%10 : CtBlock, %8 : CtBlock);
            %12 : CtBlock = pbs<IfFalseZeroed>(%11 : CtBlock);
            %14 : CtBlock = pbs<IfFalseZeroed>(%13 : CtBlock);
            %16 : CtBlock = pbs<IfFalseZeroed>(%15 : CtBlock);
            %18 : CtBlock = pbs<IfFalseZeroed>(%17 : CtBlock);
            %20 : CtBlock = pbs<IfFalseZeroed>(%19 : CtBlock);
            %22 : CtBlock = pbs<IfFalseZeroed>(%21 : CtBlock);
            %24 : CtBlock = pbs<IfFalseZeroed>(%23 : CtBlock);
            %26 : CtBlock = pbs<IfFalseZeroed>(%25 : CtBlock);
            %28 : CtInt = store_ct_block<0>(%12 : CtBlock, %27 : CtInt);
            %29 : CtInt = store_ct_block<1>(%14 : CtBlock, %28 : CtInt);
            %30 : CtInt = store_ct_block<2>(%16 : CtBlock, %29 : CtInt);
            %31 : CtInt = store_ct_block<3>(%18 : CtBlock, %30 : CtInt);
            %32 : CtInt = store_ct_block<4>(%20 : CtBlock, %31 : CtInt);
            %33 : CtInt = store_ct_block<5>(%22 : CtBlock, %32 : CtInt);
            %34 : CtInt = store_ct_block<6>(%24 : CtBlock, %33 : CtInt);
            %35 : CtInt = store_ct_block<7>(%26 : CtBlock, %34 : CtInt);
            output<0, CtInt>(%35 : CtInt);
        "#
        );
    }
}
