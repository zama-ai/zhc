use hpuc_ir::{traversal::OpWalker, ValMap};
use hpuc_langs::hpulang::Hpulang;

pub struct RegInfo {

}


pub fn allocate_registers(ir: &IR<Hpulang>, schedule: impl OpWalker) -> ValMap<RegInfo> {
    todo!()
}
