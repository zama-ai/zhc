use hpuc_ir::IR;
use hpuc_langs::ioplang::Ioplang;
use hpuc_utils::{CollectInSmallVec, MultiZip, svec};

use crate::builder::{Builder, IntegerConfig};

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
    fn merge(&self) -> &'static str {
        match self {
            Kind::Gt => "CmpGtMrg",
            Kind::Gte => "CmpGteMrg",
            Kind::Lt => "CmpLtMrg",
            Kind::Lte => "CmpLteMrg",
            Kind::Eq => "CmpEqMrg",
            Kind::Neq => "CmpNeqMrg",
        }
    }

    fn compare(&self) -> &'static str {
        match self {
            Kind::Gt => "CmpGt",
            Kind::Gte => "CmpGte",
            Kind::Lt => "CmpLt",
            Kind::Lte => "CmpLte",
            Kind::Eq => "CmpEq",
            Kind::Neq => "CmpNeq",
        }
    }
}

fn cmp(config: &IntegerConfig, kind: Kind) -> IR<Ioplang> {
    let mut b = Builder::new(config);

    // get input as array of blk
    let src_a = b.input_ct();
    let src_b = b.input_ct();

    // create required luts
    let lut_cmp_sign = b.decl_lut("CmpSign", |a| a);
    let lut_cmp_reduce = b.decl_lut("CmpReduce", |a| a);
    let lut_merge = b.decl_lut(kind.merge(), |a| a);
    let lut_compare = b.decl_lut(kind.compare(), |a| a);

    // pack a by pairs
    let packed_a = b.pack(src_a);
    // pack b by pairs
    let packed_b = b.pack(src_b);

    // merge a /b and get sign
    let mut merged = (packed_a.into_iter(), packed_b.into_iter())
        .mzip()
        .map(|(l, r)| {
            let a = b.sub(l, r);
            let a = b.pbs(a, lut_cmp_sign);
            let cst = b.constant(1);
            b.adds(a, cst)
        })
        .cosvec();

    // reduce (tree-based reduce)
    while merged.len() > 2 {
        let packed = b.pack(merged);
        let reduced = packed
            .into_iter()
            .map(|x| b.pbs(x, lut_cmp_reduce))
            .cosvec();
        // prepare next iter
        merged = reduced;
    }

    // last reduce and cast based on user required cmp
    let cmp_res = match merged.len() {
        2 => {
            let p = b.pack(merged);
            b.pbs(p[0], lut_merge)
        }
        1 => b.pbs(merged[0], lut_compare),
        _ => unreachable!(),
    };

    // store result (boolean) in slot 0 of output 0
    b.output_ct(svec![cmp_res]);

    b.into_ir()
}
