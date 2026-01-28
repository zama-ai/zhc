use hc_crypto::integer_semantics::CiphertextSpec;
use hc_ir::IR;
use hc_langs::ioplang::{Ioplang, Lut1Def};
use hc_utils::iter::CollectInSmallVec;

use crate::builder::{Builder, Ciphertext};

pub fn if_then_zero(spec: CiphertextSpec) -> IR<Ioplang> {
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

    builder.into_ir()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_if_then_zero() {
        let spec = CiphertextSpec::new(16, 2, 2);
        let ir = if_then_zero(spec);
        ir.check_ir(
            "
            %0 : CtInt = input<0, CtInt>();
            %1 : CtInt = input<1, CtInt>();
            %2 : CtInt = let_ct();
            %3 : CtBlock = extract_ct_block<0>(%0);
            %4 : CtBlock = extract_ct_block<1>(%0);
            %5 : CtBlock = extract_ct_block<2>(%0);
            %6 : CtBlock = extract_ct_block<3>(%0);
            %7 : CtBlock = extract_ct_block<4>(%0);
            %8 : CtBlock = extract_ct_block<5>(%0);
            %9 : CtBlock = extract_ct_block<6>(%0);
            %10 : CtBlock = extract_ct_block<7>(%0);
            %11 : CtBlock = extract_ct_block<0>(%1);
            %12 : CtBlock = pack_ct<4>(%11, %3);
            %13 : CtBlock = pack_ct<4>(%11, %4);
            %14 : CtBlock = pack_ct<4>(%11, %5);
            %15 : CtBlock = pack_ct<4>(%11, %6);
            %16 : CtBlock = pack_ct<4>(%11, %7);
            %17 : CtBlock = pack_ct<4>(%11, %8);
            %18 : CtBlock = pack_ct<4>(%11, %9);
            %19 : CtBlock = pack_ct<4>(%11, %10);
            %20 : CtBlock = pbs<IfFalseZeroed>(%12);
            %21 : CtBlock = pbs<IfFalseZeroed>(%13);
            %22 : CtBlock = pbs<IfFalseZeroed>(%14);
            %23 : CtBlock = pbs<IfFalseZeroed>(%15);
            %24 : CtBlock = pbs<IfFalseZeroed>(%16);
            %25 : CtBlock = pbs<IfFalseZeroed>(%17);
            %26 : CtBlock = pbs<IfFalseZeroed>(%18);
            %27 : CtBlock = pbs<IfFalseZeroed>(%19);
            %28 : CtInt = store_ct_block<0>(%20, %2);
            %29 : CtInt = store_ct_block<1>(%21, %28);
            %30 : CtInt = store_ct_block<2>(%22, %29);
            %31 : CtInt = store_ct_block<3>(%23, %30);
            %32 : CtInt = store_ct_block<4>(%24, %31);
            %33 : CtInt = store_ct_block<5>(%25, %32);
            %34 : CtInt = store_ct_block<6>(%26, %33);
            %35 : CtInt = store_ct_block<7>(%27, %34);
            output<0, CtInt>(%35);
        ",
        );
    }
}
