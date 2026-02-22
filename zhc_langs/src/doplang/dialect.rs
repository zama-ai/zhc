use zhc_ir::Dialect;

/// Dialect tag for the DOP hardware ISA.
///
/// Binds [`DopTypeSystem`](super::DopTypeSystem) and
/// [`DopInstructionSet`](super::DopInstructionSet) into a concrete
/// [`Dialect`] implementation. DOP is the lowest dialect in the
/// compilation pipeline: its instructions correspond directly to HPU
/// hardware opcodes.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DopLang;

impl Dialect for DopLang {
    type TypeSystem = super::DopTypeSystem;
    type InstructionSet = super::DopInstructionSet;
}
