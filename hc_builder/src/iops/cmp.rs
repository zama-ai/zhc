use hc_crypto::integer_semantics::CiphertextSpec;
use hc_ir::IR;
use hc_langs::ioplang::{Ioplang, Lut1Def};
use hc_utils::{
    iter::{CollectInSmallVec, MultiZip},
    svec,
};

use crate::builder::{Builder, Ciphertext};

/// Creates an IR for greater-than comparison between two encrypted integers.
pub fn cmp_gt(spec: CiphertextSpec) -> IR<Ioplang> {
    cmp(spec, Kind::Gt)
}

/// Creates an IR for greater-than-or-equal comparison between two encrypted integers.
pub fn cmp_gte(spec: CiphertextSpec) -> IR<Ioplang> {
    cmp(spec, Kind::Gte)
}

/// Creates an IR for less-than comparison between two encrypted integers.
pub fn cmp_lt(spec: CiphertextSpec) -> IR<Ioplang> {
    cmp(spec, Kind::Lt)
}

/// Creates an IR for less-than-or-equal comparison between two encrypted integers.
pub fn cmp_lte(spec: CiphertextSpec) -> IR<Ioplang> {
    cmp(spec, Kind::Lte)
}

/// Creates an IR for equality comparison between two encrypted integers.
pub fn cmp_eq(spec: CiphertextSpec) -> IR<Ioplang> {
    cmp(spec, Kind::Eq)
}

/// Creates an IR for inequality comparison between two encrypted integers.
pub fn cmp_neq(spec: CiphertextSpec) -> IR<Ioplang> {
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

fn cmp(spec: CiphertextSpec, kind: Kind) -> IR<Ioplang> {
    let builder = Builder::new(spec.block_spec());

    // get input as array of blk
    let src_a = builder.eint_input(spec.int_size());
    let src_b = builder.eint_input(spec.int_size());

    // pack a by pairs
    let packed_a = builder.vector_pack_one_clean(src_a.blocks());
    // pack b by pairs
    let packed_b = builder.vector_pack_one_clean(src_b.blocks());

    // merge a /b and get sign
    let mut merged = (packed_a.iter(), packed_b.iter())
        .mzip()
        .map(|(l, r)| {
            let sub_lr = builder.block_sub(l, r);
            let pbsed = builder.block_pbs(&sub_lr, Lut1Def::CmpSign);
            let cst = builder.block_constant(1);
            builder.block_adds(&pbsed, &cst)
        })
        .cosvec();

    // reduce (tree-based reduce)
    while merged.len() > 2 {
        let packed = builder.vector_pack_one(merged.as_slice());
        let reduced = packed
            .iter()
            .map(|x| builder.block_pbs(x, Lut1Def::CmpReduce))
            .cosvec();
        // prepare next iter
        merged = reduced;
    }

    // last reduce and cast based on user required cmp
    let cmp_res = match merged.len() {
        2 => {
            let p = builder.vector_pack_one(merged.as_slice());
            builder.block_pbs(&p[0], kind.merge())
        }
        1 => builder.block_pbs(&merged[0], kind.compare()),
        _ => unreachable!(),
    };

    // store result in slot 0 of output 0
    let output = Ciphertext::from_blocks(svec![cmp_res]);
    builder.eint_output(output);

    builder.into_ir()
}

#[cfg(test)]
mod test {

    use hc_crypto::integer_semantics::CiphertextSpec;

    use super::cmp_eq;

    #[test]
    fn test_cmp() {
        let spec = CiphertextSpec::new(16, 2, 2);
        let ir = cmp_eq(spec);
        ir.check_ir(
            "
            %0 : CtInt = input<0, CtInt>();
            %1 : CtInt = input<1, CtInt>();
            %2 : PtBlock = let_pt_block<1>();
            %6 : CtInt = zero_ct();
            %7 : CtBlock = extract_ct_block<0>(%0);
            %8 : CtBlock = extract_ct_block<1>(%0);
            %9 : CtBlock = extract_ct_block<2>(%0);
            %10 : CtBlock = extract_ct_block<3>(%0);
            %11 : CtBlock = extract_ct_block<4>(%0);
            %12 : CtBlock = extract_ct_block<5>(%0);
            %13 : CtBlock = extract_ct_block<6>(%0);
            %14 : CtBlock = extract_ct_block<7>(%0);
            %15 : CtBlock = extract_ct_block<0>(%1);
            %16 : CtBlock = extract_ct_block<1>(%1);
            %17 : CtBlock = extract_ct_block<2>(%1);
            %18 : CtBlock = extract_ct_block<3>(%1);
            %19 : CtBlock = extract_ct_block<4>(%1);
            %20 : CtBlock = extract_ct_block<5>(%1);
            %21 : CtBlock = extract_ct_block<6>(%1);
            %22 : CtBlock = extract_ct_block<7>(%1);
            %23 : CtBlock = pack_ct<4>(%8, %7);
            %24 : CtBlock = pack_ct<4>(%10, %9);
            %25 : CtBlock = pack_ct<4>(%12, %11);
            %26 : CtBlock = pack_ct<4>(%14, %13);
            %27 : CtBlock = pack_ct<4>(%16, %15);
            %28 : CtBlock = pack_ct<4>(%18, %17);
            %29 : CtBlock = pack_ct<4>(%20, %19);
            %30 : CtBlock = pack_ct<4>(%22, %21);
            %31 : CtBlock = pbs<None>(%23);
            %32 : CtBlock = pbs<None>(%24);
            %33 : CtBlock = pbs<None>(%25);
            %34 : CtBlock = pbs<None>(%26);
            %35 : CtBlock = pbs<None>(%27);
            %36 : CtBlock = pbs<None>(%28);
            %37 : CtBlock = pbs<None>(%29);
            %38 : CtBlock = pbs<None>(%30);
            %39 : CtBlock = sub_ct(%31, %35);
            %40 : CtBlock = sub_ct(%32, %36);
            %41 : CtBlock = sub_ct(%33, %37);
            %42 : CtBlock = sub_ct(%34, %38);
            %43 : CtBlock = pbs<CmpSign>(%39);
            %44 : CtBlock = pbs<CmpSign>(%40);
            %45 : CtBlock = pbs<CmpSign>(%41);
            %46 : CtBlock = pbs<CmpSign>(%42);
            %47 : CtBlock = add_pt(%43, %2);
            %48 : CtBlock = add_pt(%44, %2);
            %49 : CtBlock = add_pt(%45, %2);
            %50 : CtBlock = add_pt(%46, %2);
            %51 : CtBlock = pack_ct<4>(%48, %47);
            %52 : CtBlock = pack_ct<4>(%50, %49);
            %53 : CtBlock = pbs<CmpReduce>(%51);
            %54 : CtBlock = pbs<CmpReduce>(%52);
            %55 : CtBlock = pack_ct<4>(%54, %53);
            %56 : CtBlock = pbs<CmpEqMrg>(%55);
            %57 : CtInt = store_ct_block<0>(%56, %6);
            output<0, CtInt>(%57);
        ",
        );
    }
}
