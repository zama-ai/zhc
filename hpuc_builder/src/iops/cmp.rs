use hpuc_ir::IR;
use hpuc_langs::ioplang::Ioplang;
use hpuc_utils::{CollectInSmallVec, MultiZip, svec};

use crate::builder::{Builder, IntegerConfig, LutType};

/// Creates an IR for greater-than comparison between two encrypted integers.
pub fn cmp_gt(config: &IntegerConfig) -> IR<Ioplang> {
    cmp(config, Kind::Gt)
}

/// Creates an IR for greater-than-or-equal comparison between two encrypted integers.
pub fn cmp_gte(config: &IntegerConfig) -> IR<Ioplang> {
    cmp(config, Kind::Gte)
}

/// Creates an IR for less-than comparison between two encrypted integers.
pub fn cmp_lt(config: &IntegerConfig) -> IR<Ioplang> {
    cmp(config, Kind::Lt)
}

/// Creates an IR for less-than-or-equal comparison between two encrypted integers.
pub fn cmp_lte(config: &IntegerConfig) -> IR<Ioplang> {
    cmp(config, Kind::Lte)
}

/// Creates an IR for equality comparison between two encrypted integers.
pub fn cmp_eq(config: &IntegerConfig) -> IR<Ioplang> {
    cmp(config, Kind::Eq)
}

/// Creates an IR for inequality comparison between two encrypted integers.
pub fn cmp_neq(config: &IntegerConfig) -> IR<Ioplang> {
    cmp(config, Kind::Neq)
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
    fn merge(&self) -> LutType {
        match self {
            Kind::Gt => LutType::CmpGtMrg,
            Kind::Gte => LutType::CmpGteMrg,
            Kind::Lt => LutType::CmpLtMrg,
            Kind::Lte => LutType::CmpLteMrg,
            Kind::Eq => LutType::CmpEqMrg,
            Kind::Neq => LutType::CmpNeqMrg,
        }
    }

    fn compare(&self) -> LutType {
        match self {
            Kind::Gt => LutType::CmpGt,
            Kind::Gte => LutType::CmpGte,
            Kind::Lt => LutType::CmpLt,
            Kind::Lte => LutType::CmpLte,
            Kind::Eq => LutType::CmpEq,
            Kind::Neq => LutType::CmpNeq,
        }
    }
}

fn cmp(config: &IntegerConfig, kind: Kind) -> IR<Ioplang> {

    // -> .cosvec() = .collect::<SmallVec>()
    // -> .covec() = .collect::<Vec>()
    // -> (a, b, c, ...).mzip() -> Multizip flattened. Can call `.map(|(a_i, b_i, c_i, ...)| ...)`
    // -> some_iterator.chunk(n) -> chunks on iterators (no need to allocate an intermediate).

    let mut builder = Builder::new(config);

    // get input as array of blk
    let src_a = builder.input_ct();
    let src_b = builder.input_ct();

    // create required luts
    let lut_cmp_sign = builder.get_lut(LutType::CmpSign);
    let lut_cmp_reduce = builder.get_lut(LutType::CmpReduce);
    let lut_merge = builder.get_lut(kind.merge());
    let lut_compare = builder.get_lut(kind.compare());


    // pack a by pairs
    let packed_a = builder.pack(src_a);
    // pack b by pairs
    let packed_b = builder.pack(src_b);


    // merge a /b and get sign
    let mut merged = (packed_a.into_iter(), packed_b.into_iter())
        .mzip()
        .map(|(l, r)| {
            let sub_lr = builder.sub(l, r);
            let pbsed = builder.pbs(sub_lr, lut_cmp_sign);
            let cst = builder.constant(1);
            builder.adds(pbsed, cst)
        })
        .cosvec();

    // reduce (tree-based reduce)
    while merged.len() > 2 {
        let packed = builder.pack(merged);
        let reduced = packed
            .into_iter()
            .map(|x| builder.pbs(x, lut_cmp_reduce))
            .cosvec();
        // prepare next iter
        merged = reduced;
    }

    // last reduce and cast based on user required cmp
    let cmp_res = match merged.len() {
        2 => {
            let p = builder.pack(merged);
            builder.pbs(p[0], lut_merge)
        }
        1 => builder.pbs(merged[0], lut_compare),
        _ => unreachable!(),
    };

    // store result (boolean) in slot 0 of output 0
    builder.output_ct(svec![cmp_res]);

    builder.into_ir()
}

#[cfg(test)]
mod test {
    use crate::builder::IntegerConfig;

    use super::cmp_eq;


    #[test]
    fn test_cmp() {
        let config = IntegerConfig{ integer_width: 16, message_width: 2, carry_width: 2, nu_msg: 0, nu_bool: 0 };
        let ir = cmp_eq(&config);
        ir.check_ir("
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
            %22 : PlaintextBlock = constant<4_pt_block>();
            %23 : Lut1 = gen_lut1<None>();
            %25 : Lut1 = gen_lut1<None>();
            %26 : PlaintextBlock = constant<1_pt_block>();
            %31 : Lut1 = gen_lut1<None>();
            %33 : Lut1 = gen_lut1<None>();
            %34 : Ciphertext = let<Ciphertext>();
            %36 : CiphertextBlock = extract_ct_block(%0, %1);
            %37 : CiphertextBlock = extract_ct_block(%0, %2);
            %38 : CiphertextBlock = extract_ct_block(%0, %3);
            %39 : CiphertextBlock = extract_ct_block(%0, %4);
            %40 : CiphertextBlock = extract_ct_block(%0, %5);
            %41 : CiphertextBlock = extract_ct_block(%0, %6);
            %42 : CiphertextBlock = extract_ct_block(%0, %7);
            %43 : CiphertextBlock = extract_ct_block(%0, %8);
            %44 : CiphertextBlock = extract_ct_block(%9, %1);
            %45 : CiphertextBlock = extract_ct_block(%9, %2);
            %46 : CiphertextBlock = extract_ct_block(%9, %3);
            %47 : CiphertextBlock = extract_ct_block(%9, %4);
            %48 : CiphertextBlock = extract_ct_block(%9, %5);
            %49 : CiphertextBlock = extract_ct_block(%9, %6);
            %50 : CiphertextBlock = extract_ct_block(%9, %7);
            %51 : CiphertextBlock = extract_ct_block(%9, %8);
            %52 : CiphertextBlock = mac(%22, %37, %36);
            %53 : CiphertextBlock = mac(%22, %39, %38);
            %54 : CiphertextBlock = mac(%22, %41, %40);
            %55 : CiphertextBlock = mac(%22, %43, %42);
            %56 : CiphertextBlock = mac(%22, %45, %44);
            %57 : CiphertextBlock = mac(%22, %47, %46);
            %58 : CiphertextBlock = mac(%22, %49, %48);
            %59 : CiphertextBlock = mac(%22, %51, %50);
            %60 : CiphertextBlock = pbs(%52, %23);
            %61 : CiphertextBlock = pbs(%53, %23);
            %62 : CiphertextBlock = pbs(%54, %23);
            %63 : CiphertextBlock = pbs(%55, %23);
            %64 : CiphertextBlock = pbs(%56, %25);
            %65 : CiphertextBlock = pbs(%57, %25);
            %66 : CiphertextBlock = pbs(%58, %25);
            %67 : CiphertextBlock = pbs(%59, %25);
            %68 : CiphertextBlock = sub_ct(%60, %64);
            %69 : CiphertextBlock = sub_ct(%61, %65);
            %70 : CiphertextBlock = sub_ct(%62, %66);
            %71 : CiphertextBlock = sub_ct(%63, %67);
            %72 : CiphertextBlock = pbs(%68, %18);
            %73 : CiphertextBlock = pbs(%69, %18);
            %74 : CiphertextBlock = pbs(%70, %18);
            %75 : CiphertextBlock = pbs(%71, %18);
            %76 : CiphertextBlock = add_pt(%72, %26);
            %77 : CiphertextBlock = add_pt(%73, %26);
            %78 : CiphertextBlock = add_pt(%74, %26);
            %79 : CiphertextBlock = add_pt(%75, %26);
            %80 : CiphertextBlock = mac(%22, %77, %76);
            %81 : CiphertextBlock = mac(%22, %79, %78);
            %82 : CiphertextBlock = pbs(%80, %31);
            %83 : CiphertextBlock = pbs(%81, %31);
            %84 : CiphertextBlock = pbs(%82, %19);
            %85 : CiphertextBlock = pbs(%83, %19);
            %86 : CiphertextBlock = mac(%22, %85, %84);
            %87 : CiphertextBlock = pbs(%86, %33);
            %88 : CiphertextBlock = pbs(%87, %20);
            %89 : Ciphertext = store_ct_block(%88, %34, %1);
            output<0, Ciphertext>(%89);
        ");
    }
}
