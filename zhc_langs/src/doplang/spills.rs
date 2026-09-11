use zhc_ir::IR;

use crate::doplang::{CtMem, DopInstructionSet, DopLang};

pub fn count_spills(ir: &IR<DopLang>) -> usize {
    ir.walk_ops_linear()
        .filter(|op| {
            matches!(
                op.get_instruction(),
                DopInstructionSet::ST {
                    dst: CtMem::Heap(_),
                    ..
                }
            )
        })
        .count()
}
