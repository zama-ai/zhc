use zhc_ir::Dialect;

/// Dialect tag for the HPU register-level language.
///
/// Unit struct binding [`HpuTypeSystem`](super::HpuTypeSystem) and
/// [`HpuInstructionSet`](super::HpuInstructionSet) into a concrete
/// [`Dialect`] implementation.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HpuLang;

impl Dialect for HpuLang {
    type TypeSystem = super::HpuTypeSystem;
    type InstructionSet = super::HpuInstructionSet;
}
