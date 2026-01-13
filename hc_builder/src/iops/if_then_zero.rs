use hc_ir::IR;
use hc_langs::ioplang::Ioplang;
use hc_utils::iter::CollectInSmallVec;

use crate::builder::{BlockConfig, Builder, EncryptedInteger, Lut1Type};

pub fn if_then_zero(width: u8, config: &BlockConfig) -> IR<Ioplang> {
    let mut builder = Builder::new(config);

    let src = builder.eint_input(width);
    let cond = builder.eint_input(1);

    let lut = builder.lut(Lut1Type::IfFalseZeroed);

    let output_blocks = src
        .blocks()
        .iter()
        .map(|b| {
            let cst = builder.block_constant(config.shift());
            let out = builder.block_mac(&cst, &cond.blocks()[0], b);
            builder.block_pbs(&out, &lut)
        })
        .cosvec();

    builder.eint_output(EncryptedInteger::from_blocks(width, output_blocks));

    builder.into_ir()
}

#[cfg(test)]
mod test {
    use crate::{builder::BlockConfig, iops::if_then_zero::if_then_zero};

    #[test]
    fn test_if_then_zero() {
        let config = BlockConfig {
            message_width: 2,
            carry_width: 2,
        };
        let ir = if_then_zero(16, &config);
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
            %11 : Lut1 = gen_lut1<IfFalseZeroed>();
            %12 : PlaintextBlock = constant<4_pt_block>();
            %20 : Ciphertext = let<Ciphertext>();
            %29 : CiphertextBlock = extract_ct_block(%0, %1);
            %30 : CiphertextBlock = extract_ct_block(%0, %2);
            %31 : CiphertextBlock = extract_ct_block(%0, %3);
            %32 : CiphertextBlock = extract_ct_block(%0, %4);
            %33 : CiphertextBlock = extract_ct_block(%0, %5);
            %34 : CiphertextBlock = extract_ct_block(%0, %6);
            %35 : CiphertextBlock = extract_ct_block(%0, %7);
            %36 : CiphertextBlock = extract_ct_block(%0, %8);
            %37 : CiphertextBlock = extract_ct_block(%9, %1);
            %38 : CiphertextBlock = mac(%12, %37, %29);
            %39 : CiphertextBlock = mac(%12, %37, %30);
            %40 : CiphertextBlock = mac(%12, %37, %31);
            %41 : CiphertextBlock = mac(%12, %37, %32);
            %42 : CiphertextBlock = mac(%12, %37, %33);
            %43 : CiphertextBlock = mac(%12, %37, %34);
            %44 : CiphertextBlock = mac(%12, %37, %35);
            %45 : CiphertextBlock = mac(%12, %37, %36);
            %46 : CiphertextBlock = pbs(%38, %11);
            %47 : CiphertextBlock = pbs(%39, %11);
            %48 : CiphertextBlock = pbs(%40, %11);
            %49 : CiphertextBlock = pbs(%41, %11);
            %50 : CiphertextBlock = pbs(%42, %11);
            %51 : CiphertextBlock = pbs(%43, %11);
            %52 : CiphertextBlock = pbs(%44, %11);
            %53 : CiphertextBlock = pbs(%45, %11);
            %54 : Ciphertext = store_ct_block(%46, %20, %1);
            %55 : Ciphertext = store_ct_block(%47, %54, %2);
            %56 : Ciphertext = store_ct_block(%48, %55, %3);
            %57 : Ciphertext = store_ct_block(%49, %56, %4);
            %58 : Ciphertext = store_ct_block(%50, %57, %5);
            %59 : Ciphertext = store_ct_block(%51, %58, %6);
            %60 : Ciphertext = store_ct_block(%52, %59, %7);
            %61 : Ciphertext = store_ct_block(%53, %60, %8);
            output<0, Ciphertext>(%61);
        ",
        );
    }
}
