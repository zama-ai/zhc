use hpuc_utils::CollectInSmallVec;

use super::{Dialect, IR, OpIdRaw, OpRef, ValId, val_ref::ValRef};
use std::{collections::HashMap, marker::PhantomData};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Name(pub(super) OpIdRaw);

pub struct Printer<D: Dialect> {
    names: HashMap<ValId, Name>,
    show_erased_ops: bool,
    show_types: bool,
    walker: PrintWalker,
    phantom: PhantomData<D>,
}

pub enum PrintWalker {
    Linear,
    Topo
}

impl<D: Dialect> Printer<D> {
    pub fn from_ir(store: &IR<D>, walker: PrintWalker, show_types: bool, show_erased_ops: bool) -> Printer<D> {
        let names = match walker {
            PrintWalker::Linear => {
                store
                    .raw_walk_ops_linear()
                    .flat_map(|op| op.get_return_valids().iter().cloned().cosvec().into_iter())
                    .enumerate()
                    .map(|(name_id, valid)| {(valid, Name(name_id as u16))})
                    .collect()
            },
            PrintWalker::Topo => {
                store
                    .raw_walk_ops_topo()
                    .flat_map(|op| op.get_return_valids().iter().cloned().cosvec().into_iter())
                    .enumerate()
                    .map(|(name_id, valid)| {(valid, Name(name_id as u16))})
                    .collect()
            },
        };
        Printer {
            names,
            show_erased_ops,
            show_types,
            walker,
            phantom: PhantomData,
        }
    }

    pub fn ir_to_string(&self, store: &IR<D>) -> String {
        struct IRFormatter<'a, D: Dialect> {
            printer: &'a Printer<D>,
            store: &'a IR<D>,
        }

        impl<D: Dialect> std::fmt::Display for IRFormatter<'_, D> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.printer.format_ir(f, self.store)
            }
        }

        format!("{}", IRFormatter { printer: self, store })
    }

    pub fn format_arg(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        valref: ValRef<'_, D>,
    ) -> std::fmt::Result {
        let name = self.names.get(&valref.get_id()).unwrap();
        if valref.is_inactive() {
            write!(f, "%_{}", name.0)
        } else {
            write!(f, "%{}", name.0)
        }
    }

    pub fn format_ret(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        valref: ValRef<'_, D>,
    ) -> std::fmt::Result {
        let name = self.names.get(&valref.get_id()).unwrap();
        if valref.is_inactive() {
            write!(f, "%_{}", name.0)?;
        } else {
            write!(f, "%{}", name.0)?;
        }
        if self.show_types {
            write!(f, " : {}", valref.get_type())?;
        }
        Ok(())
    }

    pub fn format_opref(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        opref: OpRef<'_, D>,
    ) -> std::fmt::Result {
        if opref.is_inactive() && !self.show_erased_ops {
            return Ok(());
        }
        if opref.is_inactive() {
            write!(f, "// ")?;
        }
        let mut rets = opref.raw_get_returns_iter();
        if let Some(ret) = rets.next() {
            self.format_ret(f, ret)?;
        }
        for ret in rets {
            write!(f, ", ")?;
            self.format_ret(f, ret)?;
        }
        if opref.raw_get_returns_iter().next().is_some() {
            write!(f, " = ")?;
        }

        write!(f, "{}(", opref.operation)?;

        let mut args = opref.raw_get_args_iter();
        if let Some(arg) = args.next() {
            self.format_arg(f, arg)?;
        }
        for arg in args {
            write!(f, ", ")?;
            self.format_arg(f, arg)?;
        }
        writeln!(f, ");")
    }

    pub fn format_ir(&self, f: &mut std::fmt::Formatter<'_>, store: &IR<D>) -> std::fmt::Result {
        match self.walker {
            PrintWalker::Linear => {
                for opref in store.raw_walk_ops_linear() {
                    self.format_opref(f, opref)?;
                }
            },
            PrintWalker::Topo => {
                for opref in store.raw_walk_ops_topo() {
                    self.format_opref(f, opref)?;
                }

            },
        }

        Ok(())
    }
}
