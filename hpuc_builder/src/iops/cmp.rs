use hpuc_ir::IR;
use hpuc_langs::ioplang::Ioplang;
use hpuc_utils::{iter::{CollectInSmallVec, MultiZip}, svec};

use crate::builder::{BlockConfig, Builder, EncryptedInteger, Lut1Type};

/// Creates an IR for greater-than comparison between two encrypted integers.
pub fn cmp_gt(width: u8, config: &BlockConfig) -> IR<Ioplang> {
    cmp(width, config, Kind::Gt)
}

/// Creates an IR for greater-than-or-equal comparison between two encrypted integers.
pub fn cmp_gte(width: u8, config: &BlockConfig) -> IR<Ioplang> {
    cmp(width, config, Kind::Gte)
}

/// Creates an IR for less-than comparison between two encrypted integers.
pub fn cmp_lt(width: u8, config: &BlockConfig) -> IR<Ioplang> {
    cmp(width, config, Kind::Lt)
}

/// Creates an IR for less-than-or-equal comparison between two encrypted integers.
pub fn cmp_lte(width: u8, config: &BlockConfig) -> IR<Ioplang> {
    cmp(width, config, Kind::Lte)
}

/// Creates an IR for equality comparison between two encrypted integers.
pub fn cmp_eq(width: u8, config: &BlockConfig) -> IR<Ioplang> {
    cmp(width, config, Kind::Eq)
}

/// Creates an IR for inequality comparison between two encrypted integers.
pub fn cmp_neq(width: u8, config: &BlockConfig) -> IR<Ioplang> {
    cmp(width, config, Kind::Neq)
}

enum Kind {
    Gt,
    Gte,
    Lt,
    Lte,
    Eq,
    Neq,
}

impl Kind {
    fn merge(&self) -> Lut1Type {
        match self {
            Kind::Gt => Lut1Type::CmpGtMrg,
            Kind::Gte => Lut1Type::CmpGteMrg,
            Kind::Lt => Lut1Type::CmpLtMrg,
            Kind::Lte => Lut1Type::CmpLteMrg,
            Kind::Eq => Lut1Type::CmpEqMrg,
            Kind::Neq => Lut1Type::CmpNeqMrg,
        }
    }

    fn compare(&self) -> Lut1Type {
        match self {
            Kind::Gt => Lut1Type::CmpGt,
            Kind::Gte => Lut1Type::CmpGte,
            Kind::Lt => Lut1Type::CmpLt,
            Kind::Lte => Lut1Type::CmpLte,
            Kind::Eq => Lut1Type::CmpEq,
            Kind::Neq => Lut1Type::CmpNeq,
        }
    }
}

fn cmp(width: u8, config: &BlockConfig, kind: Kind) -> IR<Ioplang> {
    let mut builder = Builder::new(config);

    // get input as array of blk
    let src_a = builder.eint_input(width);
    let src_b = builder.eint_input(width);

    // create required luts
    let lut_cmp_sign = builder.lut(Lut1Type::CmpSign);
    let lut_cmp_reduce = builder.lut(Lut1Type::CmpReduce);
    let lut_merge = builder.lut(kind.merge());
    let lut_compare = builder.lut(kind.compare());

    // pack a by pairs
    let packed_a = builder.vector_pack_one_clean(src_a.blocks());
    // pack b by pairs
    let packed_b = builder.vector_pack_one_clean(src_b.blocks());

    // merge a /b and get sign
    let mut merged = (packed_a.iter(), packed_b.iter())
        .mzip()
        .map(|(l, r)| {
            let sub_lr = builder.block_sub(l, r);
            let pbsed = builder.block_pbs(&sub_lr, &lut_cmp_sign);
            let cst = builder.block_constant(1);
            builder.block_adds(&pbsed, &cst)
        })
        .cosvec();

    // reduce (tree-based reduce)
    while merged.len() > 2 {
        let packed = builder.vector_pack_one(merged.as_slice());
        let reduced = packed
            .iter()
            .map(|x| builder.block_pbs(x, &lut_cmp_reduce))
            .cosvec();
        // prepare next iter
        merged = reduced;
    }

    // last reduce and cast based on user required cmp
    let cmp_res = match merged.len() {
        2 => {
            let p = builder.vector_pack_one(merged.as_slice());
            builder.block_pbs(&p[0], &lut_merge)
        }
        1 => builder.block_pbs(&merged[0], &lut_compare),
        _ => unreachable!(),
    };

    // store result in slot 0 of output 0
    let output = EncryptedInteger::from_blocks(1, svec![cmp_res]);
    builder.eint_output(output);

    builder.into_ir()
}

#[cfg(test)]
mod test {
    use crate::builder::BlockConfig;

    use super::cmp_eq;

    #[test]
    fn test_cmp() {
        let config = BlockConfig {
            message_width: 2,
            carry_width: 2,
        };
        let ir = cmp_eq(16, &config);
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
            %18 : Lut1 = gen_lut1<CmpSign>();
            %19 : Lut1 = gen_lut1<CmpReduce>();
            %20 : Lut1 = gen_lut1<CmpEqMrg>();
            %22 : Lut1 = gen_lut1<None>();
            %23 : PlaintextBlock = constant<4_pt_block>();
            %24 : Lut1 = gen_lut1<None>();
            %26 : PlaintextBlock = constant<1_pt_block>();
            %32 : Ciphertext = let<Ciphertext>();
            %34 : CiphertextBlock = extract_ct_block(%0, %1);
            %35 : CiphertextBlock = extract_ct_block(%0, %2);
            %36 : CiphertextBlock = extract_ct_block(%0, %3);
            %37 : CiphertextBlock = extract_ct_block(%0, %4);
            %38 : CiphertextBlock = extract_ct_block(%0, %5);
            %39 : CiphertextBlock = extract_ct_block(%0, %6);
            %40 : CiphertextBlock = extract_ct_block(%0, %7);
            %41 : CiphertextBlock = extract_ct_block(%0, %8);
            %42 : CiphertextBlock = extract_ct_block(%9, %1);
            %43 : CiphertextBlock = extract_ct_block(%9, %2);
            %44 : CiphertextBlock = extract_ct_block(%9, %3);
            %45 : CiphertextBlock = extract_ct_block(%9, %4);
            %46 : CiphertextBlock = extract_ct_block(%9, %5);
            %47 : CiphertextBlock = extract_ct_block(%9, %6);
            %48 : CiphertextBlock = extract_ct_block(%9, %7);
            %49 : CiphertextBlock = extract_ct_block(%9, %8);
            %50 : CiphertextBlock = mac(%23, %35, %34);
            %51 : CiphertextBlock = mac(%23, %37, %36);
            %52 : CiphertextBlock = mac(%23, %39, %38);
            %53 : CiphertextBlock = mac(%23, %41, %40);
            %54 : CiphertextBlock = mac(%23, %43, %42);
            %55 : CiphertextBlock = mac(%23, %45, %44);
            %56 : CiphertextBlock = mac(%23, %47, %46);
            %57 : CiphertextBlock = mac(%23, %49, %48);
            %58 : CiphertextBlock = pbs(%50, %22);
            %59 : CiphertextBlock = pbs(%51, %22);
            %60 : CiphertextBlock = pbs(%52, %22);
            %61 : CiphertextBlock = pbs(%53, %22);
            %62 : CiphertextBlock = pbs(%54, %24);
            %63 : CiphertextBlock = pbs(%55, %24);
            %64 : CiphertextBlock = pbs(%56, %24);
            %65 : CiphertextBlock = pbs(%57, %24);
            %66 : CiphertextBlock = sub_ct(%58, %62);
            %67 : CiphertextBlock = sub_ct(%59, %63);
            %68 : CiphertextBlock = sub_ct(%60, %64);
            %69 : CiphertextBlock = sub_ct(%61, %65);
            %70 : CiphertextBlock = pbs(%66, %18);
            %71 : CiphertextBlock = pbs(%67, %18);
            %72 : CiphertextBlock = pbs(%68, %18);
            %73 : CiphertextBlock = pbs(%69, %18);
            %74 : CiphertextBlock = add_pt(%70, %26);
            %75 : CiphertextBlock = add_pt(%71, %26);
            %76 : CiphertextBlock = add_pt(%72, %26);
            %77 : CiphertextBlock = add_pt(%73, %26);
            %78 : CiphertextBlock = mac(%23, %75, %74);
            %79 : CiphertextBlock = mac(%23, %77, %76);
            %80 : CiphertextBlock = pbs(%78, %19);
            %81 : CiphertextBlock = pbs(%79, %19);
            %82 : CiphertextBlock = mac(%23, %81, %80);
            %83 : CiphertextBlock = pbs(%82, %20);
            %84 : Ciphertext = store_ct_block(%83, %32, %1);
            output<0, Ciphertext>(%84);
        ",
        );
    }
}
