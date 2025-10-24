use crate::gir::Dialect;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Ioplang;

impl Dialect for Ioplang {
    type Types = super::types::Types;
    type Operations = super::operations::Operations;
}
