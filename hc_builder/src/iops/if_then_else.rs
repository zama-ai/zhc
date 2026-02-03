use hc_crypto::integer_semantics::CiphertextSpec;
use hc_ir::IR;
use hc_langs::ioplang::{IopLang, Lut1Def};
use hc_utils::iter::{CollectInSmallVec, MultiZip};

use crate::builder::{Builder, Ciphertext};

pub fn if_then_else(spec: CiphertextSpec) -> IR<IopLang> {
    let builder = Builder::new(spec.block_spec());

    let src_a = builder.eint_input(spec.int_size());
    let src_b = builder.eint_input(spec.int_size());
    let cond = builder.eint_input(spec.block_spec().message_size() as u16);

    let output_blocks = (src_a.blocks().iter(), src_b.blocks().iter())
        .mzip()
        .map(|(a, b)| {
            let cond_a = builder.block_pack_ct(&cond.blocks()[0], a);
            let cond_a = builder.block_pbs(&cond_a, Lut1Def::IfFalseZeroed);
            let cond_b = builder.block_pack_ct(&cond.blocks()[0], b);
            let cond_b = builder.block_pbs(&cond_b, Lut1Def::IfTrueZeroed);
            builder.block_add(&cond_a, &cond_b)
        })
        .cosvec();

    builder.eint_output(Ciphertext::from_blocks(output_blocks));

    builder.into_ir()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_if_then_else() {
        let spec = CiphertextSpec::new(16, 2, 2);
        let ir = if_then_else(spec);
        ir.check_ir(
            "
            %0 : CtInt = input<0, CtInt>();
            %1 : CtInt = input<1, CtInt>();
            %2 : CtInt = input<2, CtInt>();
            %3 : CtInt = zero_ct();
            %4 : CtBlock = extract_ct_block<0>(%0);
            %5 : CtBlock = extract_ct_block<1>(%0);
            %6 : CtBlock = extract_ct_block<2>(%0);
            %7 : CtBlock = extract_ct_block<3>(%0);
            %8 : CtBlock = extract_ct_block<4>(%0);
            %9 : CtBlock = extract_ct_block<5>(%0);
            %10 : CtBlock = extract_ct_block<6>(%0);
            %11 : CtBlock = extract_ct_block<7>(%0);
            %12 : CtBlock = extract_ct_block<0>(%1);
            %13 : CtBlock = extract_ct_block<1>(%1);
            %14 : CtBlock = extract_ct_block<2>(%1);
            %15 : CtBlock = extract_ct_block<3>(%1);
            %16 : CtBlock = extract_ct_block<4>(%1);
            %17 : CtBlock = extract_ct_block<5>(%1);
            %18 : CtBlock = extract_ct_block<6>(%1);
            %19 : CtBlock = extract_ct_block<7>(%1);
            %20 : CtBlock = extract_ct_block<0>(%2);
            %21 : CtBlock = pack_ct<4>(%20, %4);
            %22 : CtBlock = pack_ct<4>(%20, %12);
            %23 : CtBlock = pack_ct<4>(%20, %5);
            %24 : CtBlock = pack_ct<4>(%20, %13);
            %25 : CtBlock = pack_ct<4>(%20, %6);
            %26 : CtBlock = pack_ct<4>(%20, %14);
            %27 : CtBlock = pack_ct<4>(%20, %7);
            %28 : CtBlock = pack_ct<4>(%20, %15);
            %29 : CtBlock = pack_ct<4>(%20, %8);
            %30 : CtBlock = pack_ct<4>(%20, %16);
            %31 : CtBlock = pack_ct<4>(%20, %9);
            %32 : CtBlock = pack_ct<4>(%20, %17);
            %33 : CtBlock = pack_ct<4>(%20, %10);
            %34 : CtBlock = pack_ct<4>(%20, %18);
            %35 : CtBlock = pack_ct<4>(%20, %11);
            %36 : CtBlock = pack_ct<4>(%20, %19);
            %37 : CtBlock = pbs<IfFalseZeroed>(%21);
            %38 : CtBlock = pbs<IfTrueZeroed>(%22);
            %39 : CtBlock = pbs<IfFalseZeroed>(%23);
            %40 : CtBlock = pbs<IfTrueZeroed>(%24);
            %41 : CtBlock = pbs<IfFalseZeroed>(%25);
            %42 : CtBlock = pbs<IfTrueZeroed>(%26);
            %43 : CtBlock = pbs<IfFalseZeroed>(%27);
            %44 : CtBlock = pbs<IfTrueZeroed>(%28);
            %45 : CtBlock = pbs<IfFalseZeroed>(%29);
            %46 : CtBlock = pbs<IfTrueZeroed>(%30);
            %47 : CtBlock = pbs<IfFalseZeroed>(%31);
            %48 : CtBlock = pbs<IfTrueZeroed>(%32);
            %49 : CtBlock = pbs<IfFalseZeroed>(%33);
            %50 : CtBlock = pbs<IfTrueZeroed>(%34);
            %51 : CtBlock = pbs<IfFalseZeroed>(%35);
            %52 : CtBlock = pbs<IfTrueZeroed>(%36);
            %53 : CtBlock = add_ct(%37, %38);
            %54 : CtBlock = add_ct(%39, %40);
            %55 : CtBlock = add_ct(%41, %42);
            %56 : CtBlock = add_ct(%43, %44);
            %57 : CtBlock = add_ct(%45, %46);
            %58 : CtBlock = add_ct(%47, %48);
            %59 : CtBlock = add_ct(%49, %50);
            %60 : CtBlock = add_ct(%51, %52);
            %61 : CtInt = store_ct_block<0>(%53, %3);
            %62 : CtInt = store_ct_block<1>(%54, %61);
            %63 : CtInt = store_ct_block<2>(%55, %62);
            %64 : CtInt = store_ct_block<3>(%56, %63);
            %65 : CtInt = store_ct_block<4>(%57, %64);
            %66 : CtInt = store_ct_block<5>(%58, %65);
            %67 : CtInt = store_ct_block<6>(%59, %66);
            %68 : CtInt = store_ct_block<7>(%60, %67);
            output<0, CtInt>(%68);
        ",
        );
    }
}
