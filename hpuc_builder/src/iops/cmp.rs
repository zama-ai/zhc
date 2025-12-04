use hpuc_ir::IR;
use hpuc_langs::ioplang::Ioplang;
use hpuc_utils::{svec, CollectInSmallVec, MultiZip};

use crate::builder::{Builder, Config};

pub fn cmp(config: Config) -> IR<Ioplang> {
    let mut b = Builder::new(config);

    // get input as array of blk
    let src_a = b.input_ct();
    let src_b = b.input_ct();

    // create required pbslut
    let cmp_sign = b.decl_lut("cmpsign", |a| a);
    let cmp_reduce = b.decl_lut("cmpreduce", |a| a);
    let user_mrg = b.decl_lut("usermrg", |a| a);
    let user_cmp = b.decl_lut("usercmp", |a| a);

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
