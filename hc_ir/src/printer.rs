use hc_utils::iter::{CollectInSmallVec, Separate};

use crate::{AnnValRef, Annotation};

use super::{
    Dialect, IR, OpIdRaw, OpRef, ValId,
    annotation::{AnnIR, AnnOpRef},
    val_ref::ValRef,
};
use std::{collections::HashMap, fmt::Debug, marker::PhantomData};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Name(pub(super) OpIdRaw);

/// A utility for formatting IR structures into human-readable text.
///
/// The printer assigns unique names to values and formats operations
/// according to configurable display options. Different traversal
/// orders can be used to control the output organization.
pub struct Printer<D: Dialect> {
    names: HashMap<ValId, Name>,
    show_erased_ops: bool,
    show_types: bool,
    show_op_ann: bool,
    show_val_ann: bool,
    walker: PrintWalker,
    phantom: PhantomData<D>,
}

/// Specifies the traversal order for printing operations.
pub enum PrintWalker {
    /// Print operations in the order they were added to the IR.
    Linear,
    /// Print operations in topological order (dependencies before users).
    Topo,
}

enum NewLined<T> {
    Line(T),
    NewLine,
}

impl<D: Dialect> Printer<D> {
    /// Creates a new printer configured for the specified IR.
    ///
    /// The `walker` determines traversal order, `show_types` controls whether
    /// type annotations are included, and `show_erased_ops` determines whether
    /// inactive operations are displayed.
    pub fn from_ir(
        store: &IR<D>,
        walker: PrintWalker,
        show_types: bool,
        show_erased_ops: bool,
    ) -> Printer<D> {
        let names = match walker {
            PrintWalker::Linear => store
                .raw_walk_ops_linear()
                .flat_map(|op| op.get_return_valids().iter().cloned().cosvec().into_iter())
                .enumerate()
                .map(|(name_id, valid)| (valid, Name(name_id as u16)))
                .collect(),
            PrintWalker::Topo => store
                .raw_walk_ops_topo()
                .flat_map(|op| op.get_return_valids().iter().cloned().cosvec().into_iter())
                .enumerate()
                .map(|(name_id, valid)| (valid, Name(name_id as u16)))
                .collect(),
        };
        Printer {
            names,
            show_erased_ops,
            show_types,
            show_op_ann: false,
            show_val_ann: false,
            walker,
            phantom: PhantomData,
        }
    }

    /// Creates a new printer configured for the specified annotated IR.
    ///
    /// The `walker` determines traversal order, `show_types` controls whether
    /// type annotations are included, and `show_erased_ops` determines whether
    /// inactive operations are displayed.
    pub fn from_ann_ir<OpAnn: Annotation, ValAnn: Annotation>(
        ann_ir: &AnnIR<D, OpAnn, ValAnn>,
        walker: PrintWalker,
        show_types: bool,
        show_erased_ops: bool,
        show_op_ann: bool,
        show_val_ann: bool,
    ) -> Printer<D> {
        let names = match walker {
            PrintWalker::Linear => ann_ir
                .raw_walk_ops_linear()
                .flat_map(|op| op.get_return_valids().iter().cloned().cosvec().into_iter())
                .enumerate()
                .map(|(name_id, valid)| (valid, Name(name_id as u16)))
                .collect(),
            PrintWalker::Topo => ann_ir
                .raw_walk_ops_topo()
                .flat_map(|op| op.get_return_valids().iter().cloned().cosvec().into_iter())
                .enumerate()
                .map(|(name_id, valid)| (valid, Name(name_id as u16)))
                .collect(),
        };
        Printer {
            names,
            show_erased_ops,
            show_types,
            show_op_ann,
            show_val_ann,
            walker,
            phantom: PhantomData,
        }
    }

    /// Formats the entire IR as a string.
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

        format!(
            "{}",
            IRFormatter {
                printer: self,
                store
            }
        )
    }

    /// Formats a value reference as an argument in an operation.
    pub fn format_arg(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        valref: &ValRef<'_, D>,
    ) -> std::fmt::Result {
        let name = self.names.get(&valref.get_id()).unwrap();
        if valref.is_inactive() {
            write!(f, "%_{}", name.0)
        } else {
            write!(f, "%{}", name.0)
        }
    }

    /// Formats a value reference as a return value with optional type annotation.
    pub fn format_ret(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        valref: &ValRef<'_, D>,
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

    /// Formats a complete operation with its arguments and return values.
    pub fn format_opref(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        opref: &OpRef<'_, D>,
    ) -> std::fmt::Result {
        if opref.is_inactive() && !self.show_erased_ops {
            return Ok(());
        }
        if opref.is_inactive() {
            write!(f, "// ")?;
        }
        let mut rets = opref.raw_get_returns_iter();
        if let Some(ret) = rets.next() {
            self.format_ret(f, &ret)?;
        }
        for ret in rets {
            write!(f, ", ")?;
            self.format_ret(f, &ret)?;
        }
        if opref.raw_get_returns_iter().next().is_some() {
            write!(f, " = ")?;
        }

        write!(f, "{}(", opref.operation)?;

        let mut args = opref.raw_get_args_iter();
        if let Some(arg) = args.next() {
            self.format_arg(f, &arg)?;
        }
        for arg in args {
            write!(f, ", ")?;
            self.format_arg(f, &arg)?;
        }
        write!(f, ");")
    }

    /// Formats the entire IR.
    pub fn format_ir(&self, f: &mut std::fmt::Formatter<'_>, store: &IR<D>) -> std::fmt::Result {
        match self.walker {
            PrintWalker::Linear => {
                store
                    .raw_walk_ops_linear()
                    .map(|opref| NewLined::Line(opref))
                    .separate_with(|| NewLined::NewLine)
                    .for_each(|a| match a {
                        NewLined::Line(opref) => {
                            self.format_opref(f, &opref).unwrap();
                        }
                        NewLined::NewLine => {
                            writeln!(f).unwrap();
                        }
                    });
            }
            PrintWalker::Topo => {
                store
                    .raw_walk_ops_topo()
                    .filter(|opref|opref.is_active() || self.show_erased_ops )
                    .map(|opref| NewLined::Line(opref))
                    .separate_with(|| NewLined::NewLine)
                    .for_each(|a| match a {
                        NewLined::Line(opref) => {
                            self.format_opref(f, &opref).unwrap();
                        }
                        NewLined::NewLine => {
                            writeln!(f).unwrap();
                        }
                    });
            }
        }

        Ok(())
    }

    /// Formats the entire annotated IR as a string.
    pub fn ann_ir_to_string<'ir, OpAnn: Annotation, ValAnn: Annotation>(
        &self,
        ann_ir: &AnnIR<'ir, D, OpAnn, ValAnn>,
    ) -> String {
        struct AnnIRFormatter<'ir, 'a, D: Dialect, OpAnn: Annotation, ValAnn: Annotation> {
            printer: &'a Printer<D>,
            ann_ir: &'a AnnIR<'ir, D, OpAnn, ValAnn>,
        }

        impl<'ir, D: Dialect, OpAnn: Annotation, ValAnn: Annotation> std::fmt::Display
            for AnnIRFormatter<'ir, '_, D, OpAnn, ValAnn>
        {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.printer.format_ann_ir(f, self.ann_ir)
            }
        }

        format!(
            "{}",
            AnnIRFormatter {
                printer: self,
                ann_ir
            }
        )
    }

    /// Formats a complete annotated operation with its arguments, return values, and annotations.
    pub fn format_ann_valref<OpAnn: Annotation, ValAnn: Annotation>(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        valref: &AnnValRef<'_, '_, D, OpAnn, ValAnn>,
    ) -> std::fmt::Result {

        let name = self.names.get(&valref.get_id()).unwrap();
        if valref.is_inactive() {
            write!(f, "%_{} -> {:?}", name.0, valref.get_annotation())
        } else {
            write!(f, "%{} -> {:?}", name.0, valref.get_annotation())
        }


    }

    /// Formats a complete annotated operation with its arguments, return values, and annotations.
    pub fn format_ann_opref<OpAnn: Annotation, ValAnn: Annotation>(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        opref: &AnnOpRef<'_, '_, D, OpAnn, ValAnn>,
    ) -> std::fmt::Result {

        self.format_opref(f, opref)?;

        // Add operation annotation
        if self.show_op_ann {
            writeln!(f)?;
            write!(f, "    operation -> {:?}", opref.get_annotation())?;
        }

        // Add value annotations for return values
        if self.show_val_ann {
            for ret in opref.get_returns_iter() {
                writeln!(f)?;
                let name = self.names.get(&ret.get_id()).unwrap();
                if ret.is_inactive() {
                    write!(f, "    %_{} -> {:?}", name.0, ret.get_annotation())?;
                } else {
                    write!(f, "    %{} -> {:?}", name.0, ret.get_annotation())?;
                }
            }
        }

        Ok(())
    }

    /// Formats the entire annotated IR.
    pub fn format_ann_ir<OpAnn: Annotation, ValAnn: Annotation>(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        ann_ir: &AnnIR<D, OpAnn, ValAnn>,
    ) -> std::fmt::Result {

        match self.walker {
            PrintWalker::Linear => {
                ann_ir.walk_ops_linear().cosvec().into_iter()
            }
            PrintWalker::Topo => {
                ann_ir.walk_ops_topological().cosvec().into_iter()
            }
        }
        .map(|opref| NewLined::Line(opref))
        .separate_with(|| NewLined::NewLine)
        .for_each(|a| match a {
            NewLined::Line(opref) => {
                self.format_ann_opref(f, &opref).unwrap();
            }
            NewLined::NewLine => {
                writeln!(f).unwrap();
            }
        });

        Ok(())

    }
}
