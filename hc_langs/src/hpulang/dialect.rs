use hc_ir::Dialect;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Hpulang;

impl Dialect for Hpulang {
    type Types = super::types::Types;
    type Operations = super::operations::Operations;
}
