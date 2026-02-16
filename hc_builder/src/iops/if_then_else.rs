use hc_crypto::integer_semantics::CiphertextSpec;
use hc_ir::IR;
use hc_langs::ioplang::{IopLang, Lut1Def};
use hc_utils::iter::{CollectInSmallVec, MultiZip};

use crate::builder::Builder;

pub fn if_then_else(spec: CiphertextSpec) -> IR<IopLang> {
    let builder = Builder::new(spec.block_spec());

    let src_a = builder.declare_ciphertext_input(spec.int_size());
    let src_a_blocks = builder.split_ciphertext(&src_a);
    let src_b = builder.declare_ciphertext_input(spec.int_size());
    let src_b_blocks = builder.split_ciphertext(&src_b);
    let cond = builder.declare_ciphertext_input(spec.block_spec().message_size() as u16);
    let cond_blocks = builder.split_ciphertext(&cond);

    let output_blocks = (src_a_blocks.iter(), src_b_blocks.iter())
        .mzip()
        .map(|(a, b)| {
            let cond_a = builder.block_pack(&cond_blocks[0], a);
            let cond_a = builder.block_lookup(&cond_a, Lut1Def::IfFalseZeroed);
            let cond_b = builder.block_pack(&cond_blocks[0], b);
            let cond_b = builder.block_lookup(&cond_b, Lut1Def::IfTrueZeroed);
            builder.block_add(&cond_a, &cond_b)
        })
        .cosvec();

    builder.declare_ciphertext_output(builder.join_ciphertext(output_blocks));

    builder.into_ir()
}

#[cfg(test)]
mod test {
    use super::*;
    use hc_utils::assert_display_is;

    #[test]
    fn test_if_then_else() {
        let spec = CiphertextSpec::new(16, 2, 2);
        let ir = if_then_else(spec);
        assert_display_is!(
            ir.format(),
            r#"
                %0 : CtInt = input<0, CtInt>();
                %9 : CtInt = input<1, CtInt>();
                %18 : CtInt = input<2, CtInt>();
                %60 : CtInt = decl_ct();
                %1 : CtBlock = extract_ct_block<0>(%0 : CtInt);
                %2 : CtBlock = extract_ct_block<1>(%0 : CtInt);
                %3 : CtBlock = extract_ct_block<2>(%0 : CtInt);
                %4 : CtBlock = extract_ct_block<3>(%0 : CtInt);
                %5 : CtBlock = extract_ct_block<4>(%0 : CtInt);
                %6 : CtBlock = extract_ct_block<5>(%0 : CtInt);
                %7 : CtBlock = extract_ct_block<6>(%0 : CtInt);
                %8 : CtBlock = extract_ct_block<7>(%0 : CtInt);
                %10 : CtBlock = extract_ct_block<0>(%9 : CtInt);
                %11 : CtBlock = extract_ct_block<1>(%9 : CtInt);
                %12 : CtBlock = extract_ct_block<2>(%9 : CtInt);
                %13 : CtBlock = extract_ct_block<3>(%9 : CtInt);
                %14 : CtBlock = extract_ct_block<4>(%9 : CtInt);
                %15 : CtBlock = extract_ct_block<5>(%9 : CtInt);
                %16 : CtBlock = extract_ct_block<6>(%9 : CtInt);
                %17 : CtBlock = extract_ct_block<7>(%9 : CtInt);
                %19 : CtBlock = extract_ct_block<0>(%18 : CtInt);
                %20 : CtBlock = pack_ct<4>(%19 : CtBlock, %1 : CtBlock);
                %22 : CtBlock = pack_ct<4>(%19 : CtBlock, %10 : CtBlock);
                %25 : CtBlock = pack_ct<4>(%19 : CtBlock, %2 : CtBlock);
                %27 : CtBlock = pack_ct<4>(%19 : CtBlock, %11 : CtBlock);
                %30 : CtBlock = pack_ct<4>(%19 : CtBlock, %3 : CtBlock);
                %32 : CtBlock = pack_ct<4>(%19 : CtBlock, %12 : CtBlock);
                %35 : CtBlock = pack_ct<4>(%19 : CtBlock, %4 : CtBlock);
                %37 : CtBlock = pack_ct<4>(%19 : CtBlock, %13 : CtBlock);
                %40 : CtBlock = pack_ct<4>(%19 : CtBlock, %5 : CtBlock);
                %42 : CtBlock = pack_ct<4>(%19 : CtBlock, %14 : CtBlock);
                %45 : CtBlock = pack_ct<4>(%19 : CtBlock, %6 : CtBlock);
                %47 : CtBlock = pack_ct<4>(%19 : CtBlock, %15 : CtBlock);
                %50 : CtBlock = pack_ct<4>(%19 : CtBlock, %7 : CtBlock);
                %52 : CtBlock = pack_ct<4>(%19 : CtBlock, %16 : CtBlock);
                %55 : CtBlock = pack_ct<4>(%19 : CtBlock, %8 : CtBlock);
                %57 : CtBlock = pack_ct<4>(%19 : CtBlock, %17 : CtBlock);
                %21 : CtBlock = pbs<IfFalseZeroed>(%20 : CtBlock);
                %23 : CtBlock = pbs<IfTrueZeroed>(%22 : CtBlock);
                %26 : CtBlock = pbs<IfFalseZeroed>(%25 : CtBlock);
                %28 : CtBlock = pbs<IfTrueZeroed>(%27 : CtBlock);
                %31 : CtBlock = pbs<IfFalseZeroed>(%30 : CtBlock);
                %33 : CtBlock = pbs<IfTrueZeroed>(%32 : CtBlock);
                %36 : CtBlock = pbs<IfFalseZeroed>(%35 : CtBlock);
                %38 : CtBlock = pbs<IfTrueZeroed>(%37 : CtBlock);
                %41 : CtBlock = pbs<IfFalseZeroed>(%40 : CtBlock);
                %43 : CtBlock = pbs<IfTrueZeroed>(%42 : CtBlock);
                %46 : CtBlock = pbs<IfFalseZeroed>(%45 : CtBlock);
                %48 : CtBlock = pbs<IfTrueZeroed>(%47 : CtBlock);
                %51 : CtBlock = pbs<IfFalseZeroed>(%50 : CtBlock);
                %53 : CtBlock = pbs<IfTrueZeroed>(%52 : CtBlock);
                %56 : CtBlock = pbs<IfFalseZeroed>(%55 : CtBlock);
                %58 : CtBlock = pbs<IfTrueZeroed>(%57 : CtBlock);
                %24 : CtBlock = add_ct(%21 : CtBlock, %23 : CtBlock);
                %29 : CtBlock = add_ct(%26 : CtBlock, %28 : CtBlock);
                %34 : CtBlock = add_ct(%31 : CtBlock, %33 : CtBlock);
                %39 : CtBlock = add_ct(%36 : CtBlock, %38 : CtBlock);
                %44 : CtBlock = add_ct(%41 : CtBlock, %43 : CtBlock);
                %49 : CtBlock = add_ct(%46 : CtBlock, %48 : CtBlock);
                %54 : CtBlock = add_ct(%51 : CtBlock, %53 : CtBlock);
                %59 : CtBlock = add_ct(%56 : CtBlock, %58 : CtBlock);
                %61 : CtInt = store_ct_block<0>(%24 : CtBlock, %60 : CtInt);
                %62 : CtInt = store_ct_block<1>(%29 : CtBlock, %61 : CtInt);
                %63 : CtInt = store_ct_block<2>(%34 : CtBlock, %62 : CtInt);
                %64 : CtInt = store_ct_block<3>(%39 : CtBlock, %63 : CtInt);
                %65 : CtInt = store_ct_block<4>(%44 : CtBlock, %64 : CtInt);
                %66 : CtInt = store_ct_block<5>(%49 : CtBlock, %65 : CtInt);
                %67 : CtInt = store_ct_block<6>(%54 : CtBlock, %66 : CtInt);
                %68 : CtInt = store_ct_block<7>(%59 : CtBlock, %67 : CtInt);
                output<0, CtInt>(%68 : CtInt);
            "#
        );
    }
}
