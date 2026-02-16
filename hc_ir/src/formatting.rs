use std::any::TypeId;
use std::fmt::Write as _;

use hc_utils::iter::{ReconcilerOf2, Separate};

use crate::{Annotation, val_ref::ValRef};

use super::{
    Dialect, IR, OpRef,
    annotation::{AnnIR, AnnOpRef, AnnValRef},
};

/// Specifies the traversal order for printing operations.
pub enum PrintWalker {
    /// Print operations in the order they were added to the IR.
    Linear,
    /// Print operations in topological order (dependencies before users).
    Topo,
}

enum Separated<T> {
    Content(T),
    Separator,
}

/// A formatter for IR structures.
pub struct IRFormatter<'r, D: Dialect> {
    ir: &'r IR<D>,
    show_erased_ops: bool,
    show_types: bool,
    show_opid: bool,
    show_comments: bool,
    walker: PrintWalker,
    indent: usize,
}

impl<'ir, D: Dialect> IRFormatter<'ir, D> {
    /// Creates a new formatter for the given IR.
    pub fn new(ir: &'ir IR<D>) -> Self {
        Self {
            ir,
            show_erased_ops: false,
            show_types: true,
            show_opid: false,
            show_comments: false,
            walker: PrintWalker::Topo,
            indent: 0,
        }
    }

    pub fn with_indent(mut self, indent: usize) -> Self {
        self.indent += indent;
        self
    }

    pub fn show_erased_ops(mut self, show: bool) -> Self {
        self.show_erased_ops = show;
        self
    }

    pub fn show_types(mut self, show: bool) -> Self {
        self.show_types = show;
        self
    }

    pub fn show_opid(mut self, show: bool) -> Self {
        self.show_opid = show;
        self
    }

    pub fn show_comments(mut self, show: bool) -> Self {
        self.show_comments = show;
        self
    }

    pub fn with_walker(mut self, walker: PrintWalker) -> Self {
        self.walker = walker;
        self
    }

    pub fn dump(&self) -> ! {
        println!("{self}");
        panic!("dump");
    }
}

impl<D: Dialect> std::fmt::Display for IRFormatter<'_, D> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let max_comment_len = if self.show_comments {
            self.ir
                .walk_ops_linear()
                .filter_map(|op| op.get_comment().map(|c| c.len()))
                .max()
                .unwrap_or(0)
        } else {
            0
        };
        let opid_width = if self.show_opid {
            self.ir.n_ops().checked_ilog10().map_or(1, |x| x + 1) as usize
        } else {
            0
        };
        format_ir(
            f,
            self.ir,
            &self.walker,
            self.show_erased_ops,
            self.show_types,
            self.show_opid,
            opid_width,
            self.show_comments,
            max_comment_len,
            self.indent,
        )
    }
}

/// A formatter for annotated IR structures.
pub struct AnnIRFormatter<'r, 'ir, D: Dialect, OpAnn: Annotation, ValAnn: Annotation> {
    ann_ir: &'r AnnIR<'ir, D, OpAnn, ValAnn>,
    show_erased_ops: bool,
    show_types: bool,
    show_opid: bool,
    show_comments: bool,
    show_op_ann: bool,
    show_op_ann_alternate: bool,
    show_val_ann: bool,
    show_val_ann_alternate: bool,
    walker: PrintWalker,
    indent: usize,
}

impl<'r, 'ir, D: Dialect, OpAnn: Annotation, ValAnn: Annotation>
    AnnIRFormatter<'r, 'ir, D, OpAnn, ValAnn>
{
    /// Creates a new formatter for the given annotated IR.
    pub fn new(ann_ir: &'r AnnIR<'ir, D, OpAnn, ValAnn>) -> Self {
        Self {
            ann_ir,
            show_erased_ops: false,
            show_types: true,
            show_opid: false,
            show_comments: false,
            show_op_ann: true,
            show_op_ann_alternate: false,
            show_val_ann: true,
            show_val_ann_alternate: false,
            walker: PrintWalker::Topo,
            indent: 0,
        }
    }

    pub fn with_indent(mut self, indent: usize) -> Self {
        self.indent += indent;
        self
    }

    pub fn show_opid(mut self, show: bool) -> Self {
        self.show_opid = show;
        self
    }

    pub fn show_erased_ops(mut self, show: bool) -> Self {
        self.show_erased_ops = show;
        self
    }

    pub fn show_types(mut self, show: bool) -> Self {
        self.show_types = show;
        self
    }

    pub fn show_comments(mut self, show: bool) -> Self {
        self.show_comments = show;
        self
    }

    pub fn show_op_ann(mut self, show: bool) -> Self {
        self.show_op_ann = show;
        self
    }

    pub fn show_op_ann_alternate(mut self, alternate: bool) -> Self {
        self.show_op_ann_alternate = alternate;
        self
    }

    pub fn show_val_ann(mut self, show: bool) -> Self {
        self.show_val_ann = show;
        self
    }

    pub fn show_val_ann_alternate(mut self, alternate: bool) -> Self {
        self.show_val_ann_alternate = alternate;
        self
    }

    pub fn with_walker(mut self, walker: PrintWalker) -> Self {
        self.walker = walker;
        self
    }

    pub fn dump(&self) -> ! {
        println!("{self}");
        panic!("dump");
    }
}

impl<D: Dialect, OpAnn: Annotation, ValAnn: Annotation> std::fmt::Display
    for AnnIRFormatter<'_, '_, D, OpAnn, ValAnn>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let max_comment_len = if self.show_comments {
            self.ann_ir
                .walk_ops_linear()
                .filter_map(|op| op.get_comment().map(|c| c.len()))
                .max()
                .unwrap_or(0)
        } else {
            0
        };
        let opid_width = if self.show_opid {
            self.ann_ir.n_ops().checked_ilog10().map_or(1, |x| x + 1) as usize
        } else {
            0
        };
        format_ann_ir(
            f,
            self.ann_ir,
            &self.walker,
            self.show_erased_ops,
            self.show_types,
            self.show_opid,
            opid_width,
            self.show_comments,
            max_comment_len,
            self.show_op_ann,
            self.show_op_ann_alternate,
            self.show_val_ann,
            self.show_val_ann_alternate,
            self.indent,
        )
    }
}

/// A formatter for a single operation reference.
pub struct OpRefFormatter<'r, 'ir, D: Dialect> {
    opref: &'r OpRef<'ir, D>,
    show_erased: bool,
    show_types: bool,
    show_opid: bool,
    show_comments: bool,
    indent: usize,
}

impl<'r, 'ir, D: Dialect> OpRefFormatter<'r, 'ir, D> {
    pub fn new(opref: &'r OpRef<'ir, D>) -> Self {
        Self {
            opref,
            show_erased: false,
            show_types: true,
            show_opid: false,
            show_comments: false,
            indent: 0,
        }
    }

    pub fn with_indent(mut self, indent: usize) -> Self {
        self.indent += indent;
        self
    }

    pub fn show_erased(mut self, show: bool) -> Self {
        self.show_erased = show;
        self
    }

    pub fn show_types(mut self, show: bool) -> Self {
        self.show_types = show;
        self
    }

    pub fn show_opid(mut self, show: bool) -> Self {
        self.show_opid = show;
        self
    }

    pub fn show_comments(mut self, show: bool) -> Self {
        self.show_comments = show;
        self
    }

    pub fn dump(&self) -> ! {
        println!("{self}");
        panic!("dump");
    }
}

impl<D: Dialect> std::fmt::Display for OpRefFormatter<'_, '_, D> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let max_comment_len = if self.show_comments {
            self.opref.get_comment().map(|c| c.len()).unwrap_or(0)
        } else {
            0
        };
        let opid_width = if self.show_opid {
            self.opref.get_id().0.checked_ilog10().map_or(1, |x| x + 1) as usize
        } else {
            0
        };
        format_opref(
            f,
            self.opref,
            self.show_erased,
            self.show_types,
            self.show_opid,
            opid_width,
            self.show_comments,
            max_comment_len,
            self.indent,
        )
    }
}

/// A formatter for a single value reference.
pub struct ValRefFormatter<'r, 'ir, D: Dialect> {
    valref: &'r ValRef<'ir, D>,
    show_type: bool,
}

impl<'r, 'ir, D: Dialect> ValRefFormatter<'r, 'ir, D> {
    pub fn new(valref: &'r ValRef<'ir, D>) -> Self {
        Self {
            valref,
            show_type: true,
        }
    }

    pub fn show_type(mut self, show: bool) -> Self {
        self.show_type = show;
        self
    }

    pub fn dump(&self) -> ! {
        println!("{self}");
        panic!("dump");
    }
}

impl<D: Dialect> std::fmt::Display for ValRefFormatter<'_, '_, D> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        format_valref(f, self.valref, self.show_type)
    }
}

/// A formatter for a single annotated operation reference.
pub struct AnnOpRefFormatter<'r, 'ir, 'ann, D: Dialect, OpAnn: Annotation, ValAnn: Annotation> {
    opref: &'r AnnOpRef<'ir, 'ann, D, OpAnn, ValAnn>,
    show_erased: bool,
    show_types: bool,
    show_opid: bool,
    show_comments: bool,
    show_op_ann: bool,
    show_op_ann_alternate: bool,
    show_val_ann: bool,
    show_val_ann_alternate: bool,
    indent: usize,
}

impl<'r, 'ir, 'ann, D: Dialect, OpAnn: Annotation, ValAnn: Annotation>
    AnnOpRefFormatter<'r, 'ir, 'ann, D, OpAnn, ValAnn>
{
    pub fn new(opref: &'r AnnOpRef<'ir, 'ann, D, OpAnn, ValAnn>) -> Self {
        Self {
            opref,
            show_erased: false,
            show_types: true,
            show_opid: false,
            show_comments: false,
            show_op_ann: true,
            show_op_ann_alternate: false,
            show_val_ann: true,
            show_val_ann_alternate: false,
            indent: 0,
        }
    }

    pub fn with_indent(mut self, indent: usize) -> Self {
        self.indent += indent;
        self
    }

    pub fn show_erased(mut self, show: bool) -> Self {
        self.show_erased = show;
        self
    }

    pub fn show_types(mut self, show: bool) -> Self {
        self.show_types = show;
        self
    }

    pub fn show_opid(mut self, show: bool) -> Self {
        self.show_opid = show;
        self
    }

    pub fn show_comments(mut self, show: bool) -> Self {
        self.show_comments = show;
        self
    }

    pub fn show_op_ann(mut self, show: bool) -> Self {
        self.show_op_ann = show;
        self
    }

    pub fn show_op_ann_alternate(mut self, alternate: bool) -> Self {
        self.show_op_ann_alternate = alternate;
        self
    }

    pub fn show_val_ann(mut self, show: bool) -> Self {
        self.show_val_ann = show;
        self
    }

    pub fn show_val_ann_alternate(mut self, alternate: bool) -> Self {
        self.show_val_ann_alternate = alternate;
        self
    }

    pub fn dump(&self) -> ! {
        println!("{self}");
        panic!("dump");
    }
}

impl<D: Dialect, OpAnn: Annotation, ValAnn: Annotation> std::fmt::Display
    for AnnOpRefFormatter<'_, '_, '_, D, OpAnn, ValAnn>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let max_comment_len = if self.show_comments {
            self.opref.get_comment().map(|c| c.len()).unwrap_or(0)
        } else {
            0
        };
        let opid_width = if self.show_opid {
            self.opref.get_id().0.checked_ilog10().map_or(1, |x| x + 1) as usize
        } else {
            0
        };
        format_ann_opref(
            f,
            self.opref,
            self.show_erased,
            self.show_types,
            self.show_opid,
            opid_width,
            self.show_comments,
            max_comment_len,
            self.show_op_ann,
            self.show_op_ann_alternate,
            self.show_val_ann,
            self.show_val_ann_alternate,
            self.indent,
        )
    }
}

/// A formatter for a single annotated value reference.
pub struct AnnValRefFormatter<'r, 'ir, 'ann, D: Dialect, OpAnn: Annotation, ValAnn: Annotation> {
    valref: &'r AnnValRef<'ir, 'ann, D, OpAnn, ValAnn>,
    show_type: bool,
    show_ann: bool,
    show_ann_alternate: bool,
}

impl<'r, 'ir, 'ann, D: Dialect, OpAnn: Annotation, ValAnn: Annotation>
    AnnValRefFormatter<'r, 'ir, 'ann, D, OpAnn, ValAnn>
{
    pub fn new(valref: &'r AnnValRef<'ir, 'ann, D, OpAnn, ValAnn>) -> Self {
        Self {
            valref,
            show_type: true,
            show_ann: true,
            show_ann_alternate: false,
        }
    }

    pub fn show_type(mut self, show: bool) -> Self {
        self.show_type = show;
        self
    }

    pub fn show_ann(mut self, show: bool) -> Self {
        self.show_ann = show;
        self
    }

    pub fn show_ann_alternate(mut self, alternate: bool) -> Self {
        self.show_ann_alternate = alternate;
        self
    }

    pub fn dump(&self) -> ! {
        println!("{self}");
        panic!("dump");
    }
}

impl<D: Dialect, OpAnn: Annotation, ValAnn: Annotation> std::fmt::Display
    for AnnValRefFormatter<'_, '_, '_, D, OpAnn, ValAnn>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        format_ann_valref(
            f,
            self.valref,
            self.show_type,
            self.show_ann,
            self.show_ann_alternate,
        )
    }
}

fn format_ir<D: Dialect>(
    f: &mut std::fmt::Formatter<'_>,
    ir: &IR<D>,
    walker: &PrintWalker,
    show_erased_ops: bool,
    show_types: bool,
    show_opid: bool,
    opid_width: usize,
    show_comments: bool,
    max_comment_len: usize,
    indent: usize,
) -> std::fmt::Result {
    let ops_iter = match walker {
        PrintWalker::Linear => ir.raw_walk_ops_linear().reconcile_1_of_2(),
        PrintWalker::Topo => ir.raw_walk_ops_topo().reconcile_2_of_2(),
    };
    ops_iter
        .filter(|opref| opref.is_active() || show_erased_ops)
        .map(Separated::Content)
        .separate_with(|| Separated::Separator)
        .try_for_each(|a| match a {
            Separated::Content(opref) => format_opref(
                f,
                &opref,
                show_erased_ops,
                show_types,
                show_opid,
                opid_width,
                show_comments,
                max_comment_len,
                indent,
            ),
            Separated::Separator => writeln!(f),
        })
}

fn format_ann_ir<D: Dialect, OpAnn: Annotation, ValAnn: Annotation>(
    f: &mut std::fmt::Formatter<'_>,
    ann_ir: &AnnIR<'_, D, OpAnn, ValAnn>,
    walker: &PrintWalker,
    show_erased_ops: bool,
    show_types: bool,
    show_opid: bool,
    opid_width: usize,
    show_comments: bool,
    max_comment_len: usize,
    show_op_ann: bool,
    show_op_ann_alternate: bool,
    show_val_ann: bool,
    show_val_ann_alternate: bool,
    indent: usize,
) -> std::fmt::Result {
    let ops_iter = match walker {
        PrintWalker::Linear => ann_ir.walk_ops_linear().reconcile_1_of_2(),
        PrintWalker::Topo => ann_ir.walk_ops_topological().reconcile_2_of_2(),
    };

    ops_iter
        .filter(|opref| opref.is_active() || show_erased_ops)
        .map(Separated::Content)
        .separate_with(|| Separated::Separator)
        .try_for_each(|a| match a {
            Separated::Content(opref) => format_ann_opref(
                f,
                &opref,
                show_erased_ops,
                show_types,
                show_opid,
                opid_width,
                show_comments,
                max_comment_len,
                show_op_ann,
                show_op_ann_alternate,
                show_val_ann,
                show_val_ann_alternate,
                indent,
            ),
            Separated::Separator => writeln!(f),
        })
}

fn format_valref<D: Dialect>(
    f: &mut std::fmt::Formatter<'_>,
    valref: &ValRef<'_, D>,
    show_type: bool,
) -> std::fmt::Result {
    write!(f, "{}", valref.id)?;
    if show_type {
        write!(f, " : {}", valref.get_type())?;
    }
    Ok(())
}

fn format_opref<D: Dialect>(
    f: &mut std::fmt::Formatter<'_>,
    opref: &super::OpRef<'_, D>,
    show_erased_ops: bool,
    show_types: bool,
    show_opid: bool,
    opid_width: usize,
    show_comments: bool,
    max_comment_len: usize,
    indent: usize,
) -> std::fmt::Result {
    if opref.is_inactive() && !show_erased_ops {
        return Ok(());
    }

    write!(f, "{:indent$}", "", indent = indent)?;

    if opref.is_inactive() {
        write!(f, "\x1b[9m")?;
    }

    let has_comments = show_comments && max_comment_len > 0;
    if show_opid {
        if has_comments {
            write!(f, "{:0width$}   ", opref.id, width = opid_width)?;
        } else {
            write!(f, "{:0width$}   |  ", opref.id, width = opid_width)?;
        }
    }

    if has_comments {
        let comment_col_width = max_comment_len + 3;
        if let Some(comment) = opref.get_comment() {
            write!(f, "// {:width$}   | ", comment, width = max_comment_len)?;
        } else {
            write!(f, "{:width$}   | ", "", width = comment_col_width)?;
        }
    }

    opref
        .raw_get_returns_iter()
        .map(Separated::Content)
        .separate_with(|| Separated::Separator)
        .try_for_each(|v| match v {
            Separated::Content(ret) => format_valref(f, &ret, show_types),
            Separated::Separator => write!(f, ", "),
        })?;
    if opref.get_return_arity() != 0 {
        write!(f, " = ")?;
    }

    write!(f, "{}(", opref.operation)?;

    opref
        .raw_get_args_iter()
        .map(Separated::Content)
        .separate_with(|| Separated::Separator)
        .try_for_each(|v| match v {
            Separated::Content(arg) => format_valref(f, &arg, show_types),
            Separated::Separator => write!(f, ", "),
        })?;
    write!(f, ");")?;

    if opref.is_inactive() {
        write!(f, "\x1b[29m")?;
    }

    Ok(())
}

fn write_multiline(
    f: &mut std::fmt::Formatter<'_>,
    content: &str,
    continuation_prefix: &str,
) -> std::fmt::Result {
    let mut lines = content.lines();
    if let Some(first) = lines.next() {
        write!(f, "{first}")?;
        for line in lines {
            write!(f, "\n{continuation_prefix}{line}")?;
        }
    }
    Ok(())
}

fn compute_ann_line_prefix(
    indent: usize,
    show_opid: bool,
    opid_width: usize,
    show_comments: bool,
    max_comment_len: usize,
) -> String {
    let mut prefix = String::new();
    write!(&mut prefix, "{:indent$}", "", indent = indent).unwrap();
    if show_opid {
        let has_comments = show_comments && max_comment_len > 0;
        if has_comments {
            write!(&mut prefix, "{:width$}   ", "", width = opid_width).unwrap();
        } else {
            write!(&mut prefix, "{:width$}   |  ", "", width = opid_width).unwrap();
        }
    }
    if show_comments && max_comment_len > 0 {
        let comment_col_width = max_comment_len + 3;
        write!(&mut prefix, "{:width$}   | ", "", width = comment_col_width).unwrap();
    }
    prefix
}

fn format_ann_opref<D: Dialect, OpAnn: Annotation, ValAnn: Annotation>(
    f: &mut std::fmt::Formatter<'_>,
    opref: &AnnOpRef<'_, '_, D, OpAnn, ValAnn>,
    show_erased_ops: bool,
    show_types: bool,
    show_opid: bool,
    opid_width: usize,
    show_comments: bool,
    max_comment_len: usize,
    show_op_ann: bool,
    show_op_ann_alternate: bool,
    show_val_ann: bool,
    show_val_ann_alternate: bool,
    indent: usize,
) -> std::fmt::Result {
    format_opref(
        f,
        opref,
        show_erased_ops,
        show_types,
        show_opid,
        opid_width,
        show_comments,
        max_comment_len,
        indent,
    )?;
    let line_prefix = compute_ann_line_prefix(
        indent,
        show_opid,
        opid_width,
        show_comments,
        max_comment_len,
    );
    if show_op_ann && TypeId::of::<OpAnn>() != TypeId::of::<()>() {
        writeln!(f)?;
        write!(f, "{line_prefix}")?;

        let ann_str = if show_op_ann_alternate {
            format!("{:#?}", opref.get_annotation())
        } else {
            format!("{:?}", opref.get_annotation())
        };
        let continuation_prefix = format!("{line_prefix}    operation -> ");
        write!(f, "    operation -> ")?;
        write_multiline(f, &ann_str, &continuation_prefix)?;
    }

    if show_val_ann && TypeId::of::<ValAnn>() != TypeId::of::<()>() {
        for ret in opref.get_returns_iter() {
            writeln!(f)?;
            let id = ret.get_id().0;
            let ann = ret.get_annotation();

            write!(f, "{line_prefix}")?;

            let (val_prefix, ann_str) = if ret.is_inactive() {
                let prefix = format!("    %_{id} -> ");
                let ann_str = if show_val_ann_alternate {
                    format!("{ann:#?}")
                } else {
                    format!("{ann:?}")
                };
                (prefix, ann_str)
            } else {
                let prefix = format!("    %{id} -> ");
                let ann_str = if show_val_ann_alternate {
                    format!("{ann:#?}")
                } else {
                    format!("{ann:?}")
                };
                (prefix, ann_str)
            };

            let continuation_prefix =
                format!("{line_prefix}{:width$}", "", width = val_prefix.len());
            write!(f, "{val_prefix}")?;
            write_multiline(f, &ann_str, &continuation_prefix)?;
        }
    }

    Ok(())
}

fn format_ann_valref<D: Dialect, OpAnn: Annotation, ValAnn: Annotation>(
    f: &mut std::fmt::Formatter<'_>,
    valref: &AnnValRef<'_, '_, D, OpAnn, ValAnn>,
    show_type: bool,
    show_ann: bool,
    show_ann_alternate: bool,
) -> std::fmt::Result {
    write!(f, "{}", valref.get_id().0)?;
    if show_type {
        write!(f, " : {}", valref.get_type())?;
    }
    if show_ann && TypeId::of::<ValAnn>() != TypeId::of::<()>() {
        if show_ann_alternate {
            write!(f, " -> {:#?}", valref.get_annotation())?;
        } else {
            write!(f, " -> {:?}", valref.get_annotation())?;
        }
    }
    Ok(())
}
