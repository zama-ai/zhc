use crate::gir::Dialect;

#[derive(Debug, Clone)]
pub struct Ioplang;

impl Dialect for Ioplang {
    type Types = super::types::Types;
    type Operations = super::operations::Operations;
}
