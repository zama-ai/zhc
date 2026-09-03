use zhc_crypto::integer_semantics::lut::LutRegistry;
use zhc_ir::IR;
use zhc_langs::ioplang::{IopInstructionSet, IopLang};

/// Builds a [`LutRegistry`] from every lookup table used by the PBS operations of an IR.
///
/// Walks the active operations of `ir` in linear order and registers the table of each
/// single-output and two-output PBS instruction, in that order. Identical tables are registered
/// once, so identifiers are assigned in order of first appearance. Non-PBS operations and inactive
/// operations are ignored. An IR with no PBS operation yields an empty registry.
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
