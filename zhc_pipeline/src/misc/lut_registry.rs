use zhc_crypto::integer_semantics::lut::LutRegistry;
use zhc_ir::IR;
use zhc_langs::ioplang::{IopInstructionSet, IopLang};

pub fn extract_lut_registry(ir: &IR<IopLang>) -> LutRegistry {
    ir.walk_ops_linear()
        .filter(|op| op.get_instruction().is_pbs())
        .fold(LutRegistry::empty(), |mut reg, op| {
            use IopInstructionSet::*;
            match op.get_instruction() {
                Pbs { lut, .. } => reg.register_l1(lut),
                Pbs2 { lut, .. } => reg.register_l2(lut),
                _ => unreachable!(),
            };
            reg
        })
}
