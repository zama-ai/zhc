use hc_ir::IR;
use hc_langs::ioplang::Ioplang;
use hc_utils::iter::{CollectInSmallVec, MultiZip};

use crate::builder::{BlockConfig, Builder, EncryptedInteger, Lut1Type};

pub fn if_then_else(width: u8, config: &BlockConfig) -> IR<Ioplang> {
    let mut builder = Builder::new(config);

    let src_a = builder.eint_input(width);
    let src_b = builder.eint_input(width);
    let cond = builder.eint_input(1);

    let lut_if_true_zeroed = builder.lut(Lut1Type::IfTrueZeroed);
    let lut_if_false_zeroed = builder.lut(Lut1Type::IfFalseZeroed);

    let output_blocks = (src_a.blocks().iter(), src_b.blocks().iter())
        .mzip()
        .map(|(a, b)| {
            let cst = builder.block_constant(config.shift());
            let cond_a = builder.block_mac(&cst, &cond.blocks()[0], a);
            let cond_a = builder.block_pbs(&cond_a, &lut_if_false_zeroed);
            let cond_b = builder.block_mac(&cst, &cond.blocks()[0], b);
            let cond_b = builder.block_pbs(&cond_b, &lut_if_true_zeroed);
            builder.block_add(&cond_a, &cond_b)
        })
        .cosvec();

    builder.eint_output(EncryptedInteger::from_blocks(width, output_blocks));

    builder.into_ir()
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::builder::BlockConfig;

    #[test]
    fn test_if_then_else() {
        let config = BlockConfig {
            message_width: 2,
            carry_width: 2,
        };
        let ir = if_then_else(16, &config);
        ir.check_ir(
            "
            %0 : Ciphertext = input<0, Ciphertext>();
            %1 : Index = constant<0_idx>();
            %2 : Index = constant<1_idx>();
            %3 : Index = constant<2_idx>();
            %4 : Index = constant<3_idx>();
            %5 : Index = constant<4_idx>();
            %6 : Index = constant<5_idx>();
            %7 : Index = constant<6_idx>();
            %8 : Index = constant<7_idx>();
            %9 : Ciphertext = input<1, Ciphertext>();
            %18 : Ciphertext = input<2, Ciphertext>();
            %20 : Lut1 = gen_lut1<IfTrueZeroed>();
            %21 : Lut1 = gen_lut1<IfFalseZeroed>();
            %22 : PlaintextBlock = constant<4_pt_block>();
            %30 : Ciphertext = let<Ciphertext>();
            %39 : CiphertextBlock = extract_ct_block(%0, %1);
            %40 : CiphertextBlock = extract_ct_block(%0, %2);
            %41 : CiphertextBlock = extract_ct_block(%0, %3);
            %42 : CiphertextBlock = extract_ct_block(%0, %4);
            %43 : CiphertextBlock = extract_ct_block(%0, %5);
            %44 : CiphertextBlock = extract_ct_block(%0, %6);
            %45 : CiphertextBlock = extract_ct_block(%0, %7);
            %46 : CiphertextBlock = extract_ct_block(%0, %8);
            %47 : CiphertextBlock = extract_ct_block(%9, %1);
            %48 : CiphertextBlock = extract_ct_block(%9, %2);
            %49 : CiphertextBlock = extract_ct_block(%9, %3);
            %50 : CiphertextBlock = extract_ct_block(%9, %4);
            %51 : CiphertextBlock = extract_ct_block(%9, %5);
            %52 : CiphertextBlock = extract_ct_block(%9, %6);
            %53 : CiphertextBlock = extract_ct_block(%9, %7);
            %54 : CiphertextBlock = extract_ct_block(%9, %8);
            %55 : CiphertextBlock = extract_ct_block(%18, %1);
            %56 : CiphertextBlock = mac(%22, %55, %39);
            %57 : CiphertextBlock = mac(%22, %55, %47);
            %58 : CiphertextBlock = mac(%22, %55, %40);
            %59 : CiphertextBlock = mac(%22, %55, %48);
            %60 : CiphertextBlock = mac(%22, %55, %41);
            %61 : CiphertextBlock = mac(%22, %55, %49);
            %62 : CiphertextBlock = mac(%22, %55, %42);
            %63 : CiphertextBlock = mac(%22, %55, %50);
            %64 : CiphertextBlock = mac(%22, %55, %43);
            %65 : CiphertextBlock = mac(%22, %55, %51);
            %66 : CiphertextBlock = mac(%22, %55, %44);
            %67 : CiphertextBlock = mac(%22, %55, %52);
            %68 : CiphertextBlock = mac(%22, %55, %45);
            %69 : CiphertextBlock = mac(%22, %55, %53);
            %70 : CiphertextBlock = mac(%22, %55, %46);
            %71 : CiphertextBlock = mac(%22, %55, %54);
            %72 : CiphertextBlock = pbs(%56, %21);
            %73 : CiphertextBlock = pbs(%57, %20);
            %74 : CiphertextBlock = pbs(%58, %21);
            %75 : CiphertextBlock = pbs(%59, %20);
            %76 : CiphertextBlock = pbs(%60, %21);
            %77 : CiphertextBlock = pbs(%61, %20);
            %78 : CiphertextBlock = pbs(%62, %21);
            %79 : CiphertextBlock = pbs(%63, %20);
            %80 : CiphertextBlock = pbs(%64, %21);
            %81 : CiphertextBlock = pbs(%65, %20);
            %82 : CiphertextBlock = pbs(%66, %21);
            %83 : CiphertextBlock = pbs(%67, %20);
            %84 : CiphertextBlock = pbs(%68, %21);
            %85 : CiphertextBlock = pbs(%69, %20);
            %86 : CiphertextBlock = pbs(%70, %21);
            %87 : CiphertextBlock = pbs(%71, %20);
            %88 : CiphertextBlock = add_ct(%72, %73);
            %89 : CiphertextBlock = add_ct(%74, %75);
            %90 : CiphertextBlock = add_ct(%76, %77);
            %91 : CiphertextBlock = add_ct(%78, %79);
            %92 : CiphertextBlock = add_ct(%80, %81);
            %93 : CiphertextBlock = add_ct(%82, %83);
            %94 : CiphertextBlock = add_ct(%84, %85);
            %95 : CiphertextBlock = add_ct(%86, %87);
            %96 : Ciphertext = store_ct_block(%88, %30, %1);
            %97 : Ciphertext = store_ct_block(%89, %96, %2);
            %98 : Ciphertext = store_ct_block(%90, %97, %3);
            %99 : Ciphertext = store_ct_block(%91, %98, %4);
            %100 : Ciphertext = store_ct_block(%92, %99, %5);
            %101 : Ciphertext = store_ct_block(%93, %100, %6);
            %102 : Ciphertext = store_ct_block(%94, %101, %7);
            %103 : Ciphertext = store_ct_block(%95, %102, %8);
            output<0, Ciphertext>(%103);
        ",
        );
    }
}
