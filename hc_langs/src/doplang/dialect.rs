use hc_ir::Dialect;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Doplang;

impl Dialect for Doplang {
    type Types = super::types::Types;
    type Operations = super::operations::Operations;
}
