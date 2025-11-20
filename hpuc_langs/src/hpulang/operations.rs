use std::fmt::{Debug, Display};

use hpuc_ir::{DialectOperations, Signature};

use super::types::Types;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Operations {
    Add,
    Sub,
    Mac,
    Adds,
    Subs,
    Muls,
    Ld {},
    St {},
    Pbs {},
    PbsMl2 {},
    PbsMl4 {},
    PbsMl8 {},
}

impl Display for Operations {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl DialectOperations for Operations {
    type Types = Types;

    fn get_signature(&self) -> Signature<Self::Types> {
        todo!()
    }
}
