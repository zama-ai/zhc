use crate::{Dialect, IR};

pub trait Translator {
    type InputDialect: Dialect;
    type OutputDialect: Dialect;

    fn translate(&mut self, input: &IR<Self::InputDialect>) -> IR<Self::OutputDialect>;
}
