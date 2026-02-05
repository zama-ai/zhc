use crate::builder::Builder;
use hc_crypto::integer_semantics::CiphertextSpec;
use hc_ir::IR;
use hc_langs::ioplang::{IopLang, Lut1Def};
use hc_utils::iter::{CollectInSmallVec, MultiZip};

/// Creates an IR for greater-than comparison between two encrypted integers.
pub fn cmp_gt(spec: CiphertextSpec) -> IR<IopLang> {
    cmp(spec, Kind::Gt)
}

/// Creates an IR for greater-than-or-equal comparison between two encrypted integers.
pub fn cmp_gte(spec: CiphertextSpec) -> IR<IopLang> {
    cmp(spec, Kind::Gte)
}

/// Creates an IR for less-than comparison between two encrypted integers.
pub fn cmp_lt(spec: CiphertextSpec) -> IR<IopLang> {
    cmp(spec, Kind::Lt)
}

/// Creates an IR for less-than-or-equal comparison between two encrypted integers.
pub fn cmp_lte(spec: CiphertextSpec) -> IR<IopLang> {
    cmp(spec, Kind::Lte)
}

/// Creates an IR for equality comparison between two encrypted integers.
pub fn cmp_eq(spec: CiphertextSpec) -> IR<IopLang> {
    cmp(spec, Kind::Eq)
}

/// Creates an IR for inequality comparison between two encrypted integers.
pub fn cmp_neq(spec: CiphertextSpec) -> IR<IopLang> {
    cmp(spec, Kind::Neq)
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
    fn merge(&self) -> Lut1Def {
        match self {
            Kind::Gt => Lut1Def::CmpGtMrg,
            Kind::Gte => Lut1Def::CmpGteMrg,
            Kind::Lt => Lut1Def::CmpLtMrg,
            Kind::Lte => Lut1Def::CmpLteMrg,
            Kind::Eq => Lut1Def::CmpEqMrg,
            Kind::Neq => Lut1Def::CmpNeqMrg,
        }
    }

    fn compare(&self) -> Lut1Def {
        match self {
            Kind::Gt => Lut1Def::CmpGt,
            Kind::Gte => Lut1Def::CmpGte,
            Kind::Lt => Lut1Def::CmpLt,
            Kind::Lte => Lut1Def::CmpLte,
            Kind::Eq => Lut1Def::CmpEq,
            Kind::Neq => Lut1Def::CmpNeq,
        }
    }
}

fn cmp(spec: CiphertextSpec, kind: Kind) -> IR<IopLang> {
    let builder = Builder::new(spec.block_spec());

    // get input as array of blk
    let src_a = builder.declare_ciphertext_input(spec.int_size());
    let src_a_blocks = builder.split_ciphertext(&src_a);
    let src_b = builder.declare_ciphertext_input(spec.int_size());
    let src_b_blocks = builder.split_ciphertext(&src_b);

    // pack a by pairs
    let packed_a = builder.with_comment("Pack A", || builder.vector_pack_then_clean(src_a_blocks));
    // pack b by pairs
    let packed_b = builder.with_comment("Pack B", || builder.vector_pack_then_clean(src_b_blocks));

    // merge a /b and get sign
    let mut merged = builder.with_comment("Compare blocks", || {
        (packed_a.iter(), packed_b.iter())
            .mzip()
            .enumerate()
            .map(|(i, (l, r))| {
                builder.with_comment(format!("{i}-th"), || {
                    let sub_lr = builder.block_sub(l, r);
                    let pbsed = builder.block_lookup(&sub_lr, Lut1Def::CmpSign);
                    let cst = builder.block_const_plaintext(1);
                    builder.block_add_plaintext(&pbsed, &cst)
                })
            })
            .cosvec()
    });

    // reduce (tree-based reduce)
    builder.with_comment("Reduce comparison", || {
        while merged.len() > 2 {
            let packed = builder.vector_pack(merged.as_slice());
            let reduced = packed
                .iter()
                .map(|x| builder.block_lookup(x, Lut1Def::CmpReduce))
                .cosvec();
            // prepare next iter
            merged = reduced;
        }
    });

    // last reduce and cast based on user required cmp
    let cmp_res = match merged.len() {
        2 => {
            let p = builder.vector_pack(merged.as_slice());
            builder.block_lookup(&p[0], kind.merge())
        }
        1 => builder.block_lookup(&merged[0], kind.compare()),
        _ => unreachable!(),
    };

    // store result in slot 0 of output 0
    let output = builder.join_ciphertext([cmp_res]);

    builder.declare_ciphertext_output(output);

    builder.into_ir()
}

#[cfg(test)]
mod test {
    use hc_crypto::integer_semantics::CiphertextSpec;
    use hc_utils::assert_display_is;

    use super::cmp_eq;

    #[test]
    fn test_cmp() {
        let spec = CiphertextSpec::new(16, 2, 2);
        let ir = cmp_eq(spec);
        assert_display_is!(
            ir.format().show_comments(true).show_opid(true),
            r#"
                @00                              | %0 : CtInt = input<0, CtInt>();
                @09                              | %9 : CtInt = input<1, CtInt>();
                @36   // Compare blocks / 0-th   | %36 : PtBlock = let_pt_block<1>();
                @56                              | %56 : CtInt = zero_ct();
                @01                              | %1 : CtBlock = extract_ct_block<0>(%0 : CtInt);
                @02                              | %2 : CtBlock = extract_ct_block<1>(%0 : CtInt);
                @03                              | %3 : CtBlock = extract_ct_block<2>(%0 : CtInt);
                @04                              | %4 : CtBlock = extract_ct_block<3>(%0 : CtInt);
                @05                              | %5 : CtBlock = extract_ct_block<4>(%0 : CtInt);
                @06                              | %6 : CtBlock = extract_ct_block<5>(%0 : CtInt);
                @07                              | %7 : CtBlock = extract_ct_block<6>(%0 : CtInt);
                @08                              | %8 : CtBlock = extract_ct_block<7>(%0 : CtInt);
                @10                              | %10 : CtBlock = extract_ct_block<0>(%9 : CtInt);
                @11                              | %11 : CtBlock = extract_ct_block<1>(%9 : CtInt);
                @12                              | %12 : CtBlock = extract_ct_block<2>(%9 : CtInt);
                @13                              | %13 : CtBlock = extract_ct_block<3>(%9 : CtInt);
                @14                              | %14 : CtBlock = extract_ct_block<4>(%9 : CtInt);
                @15                              | %15 : CtBlock = extract_ct_block<5>(%9 : CtInt);
                @16                              | %16 : CtBlock = extract_ct_block<6>(%9 : CtInt);
                @17                              | %17 : CtBlock = extract_ct_block<7>(%9 : CtInt);
                @18   // Pack A                  | %18 : CtBlock = pack_ct<4>(%2 : CtBlock, %1 : CtBlock);
                @20   // Pack A                  | %20 : CtBlock = pack_ct<4>(%4 : CtBlock, %3 : CtBlock);
                @22   // Pack A                  | %22 : CtBlock = pack_ct<4>(%6 : CtBlock, %5 : CtBlock);
                @24   // Pack A                  | %24 : CtBlock = pack_ct<4>(%8 : CtBlock, %7 : CtBlock);
                @26   // Pack B                  | %26 : CtBlock = pack_ct<4>(%11 : CtBlock, %10 : CtBlock);
                @28   // Pack B                  | %28 : CtBlock = pack_ct<4>(%13 : CtBlock, %12 : CtBlock);
                @30   // Pack B                  | %30 : CtBlock = pack_ct<4>(%15 : CtBlock, %14 : CtBlock);
                @32   // Pack B                  | %32 : CtBlock = pack_ct<4>(%17 : CtBlock, %16 : CtBlock);
                @19   // Pack A                  | %19 : CtBlock = pbs<None>(%18 : CtBlock);
                @21   // Pack A                  | %21 : CtBlock = pbs<None>(%20 : CtBlock);
                @23   // Pack A                  | %23 : CtBlock = pbs<None>(%22 : CtBlock);
                @25   // Pack A                  | %25 : CtBlock = pbs<None>(%24 : CtBlock);
                @27   // Pack B                  | %27 : CtBlock = pbs<None>(%26 : CtBlock);
                @29   // Pack B                  | %29 : CtBlock = pbs<None>(%28 : CtBlock);
                @31   // Pack B                  | %31 : CtBlock = pbs<None>(%30 : CtBlock);
                @33   // Pack B                  | %33 : CtBlock = pbs<None>(%32 : CtBlock);
                @34   // Compare blocks / 0-th   | %34 : CtBlock = sub_ct(%19 : CtBlock, %27 : CtBlock);
                @38   // Compare blocks / 1-th   | %38 : CtBlock = sub_ct(%21 : CtBlock, %29 : CtBlock);
                @42   // Compare blocks / 2-th   | %42 : CtBlock = sub_ct(%23 : CtBlock, %31 : CtBlock);
                @46   // Compare blocks / 3-th   | %46 : CtBlock = sub_ct(%25 : CtBlock, %33 : CtBlock);
                @35   // Compare blocks / 0-th   | %35 : CtBlock = pbs<CmpSign>(%34 : CtBlock);
                @39   // Compare blocks / 1-th   | %39 : CtBlock = pbs<CmpSign>(%38 : CtBlock);
                @43   // Compare blocks / 2-th   | %43 : CtBlock = pbs<CmpSign>(%42 : CtBlock);
                @47   // Compare blocks / 3-th   | %47 : CtBlock = pbs<CmpSign>(%46 : CtBlock);
                @37   // Compare blocks / 0-th   | %37 : CtBlock = add_pt(%35 : CtBlock, %36 : PtBlock);
                @41   // Compare blocks / 1-th   | %41 : CtBlock = add_pt(%39 : CtBlock, %36 : PtBlock);
                @45   // Compare blocks / 2-th   | %45 : CtBlock = add_pt(%43 : CtBlock, %36 : PtBlock);
                @49   // Compare blocks / 3-th   | %49 : CtBlock = add_pt(%47 : CtBlock, %36 : PtBlock);
                @50   // Reduce comparison       | %50 : CtBlock = pack_ct<4>(%41 : CtBlock, %37 : CtBlock);
                @51   // Reduce comparison       | %51 : CtBlock = pack_ct<4>(%49 : CtBlock, %45 : CtBlock);
                @52   // Reduce comparison       | %52 : CtBlock = pbs<CmpReduce>(%50 : CtBlock);
                @53   // Reduce comparison       | %53 : CtBlock = pbs<CmpReduce>(%51 : CtBlock);
                @54                              | %54 : CtBlock = pack_ct<4>(%53 : CtBlock, %52 : CtBlock);
                @55                              | %55 : CtBlock = pbs<CmpEqMrg>(%54 : CtBlock);
                @57                              | %57 : CtInt = store_ct_block<0>(%55 : CtBlock, %56 : CtInt);
                @58                              | output<0, CtInt>(%57 : CtInt);
            "#
        );
    }
}
