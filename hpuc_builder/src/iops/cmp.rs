use hpuc_ir::IR;
use hpuc_langs::ioplang::Ioplang;
use hpuc_utils::{svec, CollectInSmallVec, MultiZip};

use crate::builder::{Builder, Config};

pub fn cmp_gt(config: Config) -> IR<Ioplang> {
    cmp(config, Kind::Gt)
}

pub fn cmp_gte(config: Config) -> IR<Ioplang> {
    cmp(config, Kind::Gte)
}

pub fn cmp_lt(config: Config) -> IR<Ioplang> {
    cmp(config, Kind::Lt)
}

pub fn cmp_lte(config: Config) -> IR<Ioplang> {
    cmp(config, Kind::Lte)
}

pub fn cmp_eq(config: Config) -> IR<Ioplang> {
    cmp(config, Kind::Eq)
}

pub fn cmp_neq(config: Config) -> IR<Ioplang> {
    cmp(config, Kind::Neq)
}

enum Kind {
    Gt,
    Gte,
    Lt,
    Lte,
    Eq,
    Neq
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

    fn cmp(&self) -> &'static str {
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

fn cmp(config: Config, kind: Kind) -> IR<Ioplang> {
    let mut b = Builder::new(config);

    // get input as array of blk
    let src_a = b.input_ct();
    let src_b = b.input_ct();

    // create required pbslut
    let cmp_sign = b.decl_lut("CmpSign", |a| a);
    let cmp_reduce = b.decl_lut("CmpReduce", |a| a);
    let user_mrg = b.decl_lut(kind.merge(), |a| a);
    let user_cmp = b.decl_lut(kind.cmp(), |a| a);

    // pack a by pairs
    let packed_a = b.pack(src_a);
    // pack b by pairs
    let packed_b = b.pack(src_b);

    // merge a /b and get sign
    let mut merged = (packed_a.into_iter(), packed_b.into_iter())
        .mzip()
        .map(|(l, r)| {
            let a = b.sub(l, r);
            b.pbs(a, cmp_sign)
        })
        .cosvec();

    // reduce (tree-based reduce)
    while merged.len() > 2 {
        let packed = b.pack(merged);
        let reduced = packed.into_iter().map(|x| b.pbs(x, cmp_reduce)).cosvec();
        // prepare next iter
        merged = reduced;
    }

    // last reduce and cast based on user required cmp
    let cmp_res = match merged.len() {
        2 => {
            let p = b.pack(merged);
            b.pbs(p[0], user_mrg)
        }
        1 => b.pbs(merged[0], user_cmp),
        _ => unreachable!(),
    };

    // store result (boolean) in slot 0 of output 0
    b.output_ct(svec![cmp_res]);

    b.into_ir()
}
