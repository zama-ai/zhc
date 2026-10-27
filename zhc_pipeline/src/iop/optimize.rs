use zhc_ir::{IR, cse::eliminate_common_subexpressions, dce::eliminate_dead_code};
use zhc_langs::ioplang::{IopLang, eliminate_aliases, skip_redundant_stores, skip_store_load};

pub fn optimize_iop(ir: &IR<IopLang>) -> IR<IopLang> {
    let mut ir = ir.clone();
    eliminate_aliases(&mut ir);
    skip_store_load(&mut ir);
    eliminate_dead_code(&mut ir);
    skip_redundant_stores(&mut ir);
    eliminate_dead_code(&mut ir);
    eliminate_common_subexpressions(&mut ir);
    ir
}
