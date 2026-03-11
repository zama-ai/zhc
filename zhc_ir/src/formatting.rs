use std::any::TypeId;

use zhc_utils::iter::{ReconcilerOf2, Separate};

use crate::{Annotation, val_ref::ValRef};

use super::{
    Dialect, IR, OpRef,
    annotation::{AnnIR, AnnOpRef, AnnValRef},
};

/// Specifies the traversal order for printing operations.
#[derive(Clone, Copy, Debug, Default)]
pub enum PrintWalker {
    /// Print operations in the order they were added to the IR.
    Linear,
    /// Print operations in topological order (dependencies before users).
    #[default]
    Topo,
}

/// Context for formatting IR structures.
#[derive(Clone, Debug)]
pub struct FormatContext {
    pub show_erased_ops: bool,
    pub show_types: bool,
    pub show_opid: bool,
    pub show_comments: bool,
    pub show_op_ann: bool,
    pub show_op_ann_alternate: bool,
    pub show_val_ann: bool,
    pub show_val_ann_alternate: bool,
    pub walker: PrintWalker,
    /// Accumulated prefix strings for nested formatting.
    prefixes: Vec<String>,
    /// Prefix for nested IRs (e.g., "", "a", "b", ...) used for value and op IDs.
    nested_prefix: String,
    /// Precomputed opid column width (for consistent alignment across ops).
    opid_width: Option<usize>,
    /// Precomputed max comment length (for consistent alignment across ops).
    max_comment_len: Option<usize>,
}

impl Default for FormatContext {
    fn default() -> Self {
        Self {
            show_erased_ops: false,
            show_types: true,
            show_opid: false,
            show_comments: false,
            show_op_ann: true,
            show_op_ann_alternate: false,
            show_val_ann: true,
            show_val_ann_alternate: false,
            walker: PrintWalker::default(),
            prefixes: Vec::new(),
            nested_prefix: String::new(),
            opid_width: None,
            max_comment_len: None,
        }
    }
}

impl FormatContext {
    /// Creates a new context with default settings.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns a new context with an additional prefix added.
    pub fn with_prefix(&self, prefix: impl Into<String>) -> Self {
        let mut new_prefixes = self.prefixes.clone();
        new_prefixes.push(prefix.into());
        Self {
            prefixes: new_prefixes,
            ..self.clone()
        }
    }

    /// Returns the concatenated prefix string.
    pub fn prefix(&self) -> String {
        self.prefixes.concat()
    }

    /// Returns the current nested prefix.
    pub fn nested_prefix(&self) -> &str {
        &self.nested_prefix
    }

    /// Returns a new context with the next nested prefix for nested IRs.
    /// Empty -> "a", "a" -> "b", ..., "z" -> "aa", etc.
    pub fn with_next_nested_prefix(&self) -> Self {
        let next = if self.nested_prefix.is_empty() {
            "a".to_string()
        } else {
            // Increment the prefix like a base-26 number
            let mut chars: Vec<char> = self.nested_prefix.chars().collect();
            let mut carry = true;
            for c in chars.iter_mut().rev() {
                if carry {
                    if *c == 'z' {
                        *c = 'a';
                    } else {
                        *c = ((*c as u8) + 1) as char;
                        carry = false;
                    }
                }
            }
            if carry {
                chars.insert(0, 'a');
            }
            chars.into_iter().collect()
        };
        Self {
            nested_prefix: next,
            ..self.clone()
        }
    }

    /// Builder method to set show_erased_ops.
    pub fn show_erased_ops(mut self, show: bool) -> Self {
        self.show_erased_ops = show;
        self
    }

    /// Builder method to set show_types.
    pub fn show_types(mut self, show: bool) -> Self {
        self.show_types = show;
        self
    }

    /// Builder method to set show_opid.
    pub fn show_opid(mut self, show: bool) -> Self {
        self.show_opid = show;
        self
    }

    /// Builder method to set show_comments.
    pub fn show_comments(mut self, show: bool) -> Self {
        self.show_comments = show;
        self
    }

    /// Builder method to set show_op_ann.
    pub fn show_op_ann(mut self, show: bool) -> Self {
        self.show_op_ann = show;
        self
    }

    /// Builder method to set show_op_ann_alternate.
    pub fn show_op_ann_alternate(mut self, show: bool) -> Self {
        self.show_op_ann_alternate = show;
        self
    }

    /// Builder method to set show_val_ann.
    pub fn show_val_ann(mut self, show: bool) -> Self {
        self.show_val_ann = show;
        self
    }

    /// Builder method to set show_val_ann_alternate.
    pub fn show_val_ann_alternate(mut self, show: bool) -> Self {
        self.show_val_ann_alternate = show;
        self
    }

    /// Builder method to set walker.
    pub fn with_walker(mut self, walker: PrintWalker) -> Self {
        self.walker = walker;
        self
    }

    /// Returns a new context with precomputed metrics for consistent column alignment.
    pub fn with_metrics(&self, opid_width: usize, max_comment_len: usize) -> Self {
        Self {
            opid_width: Some(opid_width),
            max_comment_len: Some(max_comment_len),
            ..self.clone()
        }
    }

    /// Computes the line prefix for operations in an IR, given the IR-level metrics.
    /// This includes the opid column and comments column.
    pub fn compute_line_prefix(&self, opid_width: usize, max_comment_len: usize) -> String {
        let mut line_prefix = String::new();
        let has_comments = self.show_comments && max_comment_len > 0;

        if self.show_opid {
            // Include nested_prefix length in the opid column width
            let prefix_len = self.nested_prefix.len();
            if has_comments {
                // Space for "@" + nested_prefix + opid + padding
                line_prefix.push_str(&" ".repeat(prefix_len + opid_width + 4));
            } else {
                // Space for "@" + nested_prefix + opid + "   |  "
                line_prefix.push_str(&" ".repeat(prefix_len + opid_width + 4));
                line_prefix.push_str("|  ");
            }
        }

        if has_comments {
            // Space for comment column + " | "
            let comment_col_width = max_comment_len + 3;
            line_prefix.push_str(&" ".repeat(comment_col_width + 4));
            line_prefix.push_str("| ");
        }

        line_prefix
    }
}

/// Trait for formatting IR elements with context.
pub trait Format {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>, ctx: &FormatContext) -> std::fmt::Result;
}

/// Wrapper to enable Display for Format types with default context.
pub struct DisplayFormat<'a, T: Format>(pub &'a T);

impl<T: Format> std::fmt::Display for DisplayFormat<'_, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f, &FormatContext::default())
    }
}

/// Wrapper that combines a formattable item with a format context.
/// Implements Display by delegating to Format::fmt.
pub struct Formatted<'a, T: Format> {
    item: &'a T,
    ctx: FormatContext,
}

impl<'a, T: Format> Formatted<'a, T> {
    /// Creates a new Formatted wrapper with default context.
    pub fn new(item: &'a T) -> Self {
        Self {
            item,
            ctx: FormatContext::default(),
        }
    }

    /// Builder method to set show_erased_ops.
    pub fn show_erased_ops(mut self, show: bool) -> Self {
        self.ctx.show_erased_ops = show;
        self
    }

    /// Builder method to set show_types.
    pub fn show_types(mut self, show: bool) -> Self {
        self.ctx.show_types = show;
        self
    }

    /// Builder method to set show_opid.
    pub fn show_opid(mut self, show: bool) -> Self {
        self.ctx.show_opid = show;
        self
    }

    /// Builder method to set show_comments.
    pub fn show_comments(mut self, show: bool) -> Self {
        self.ctx.show_comments = show;
        self
    }

    /// Builder method to set show_op_ann.
    pub fn show_op_ann(mut self, show: bool) -> Self {
        self.ctx.show_op_ann = show;
        self
    }

    /// Builder method to set show_op_ann_alternate.
    pub fn show_op_ann_alternate(mut self, show: bool) -> Self {
        self.ctx.show_op_ann_alternate = show;
        self
    }

    /// Builder method to set show_val_ann.
    pub fn show_val_ann(mut self, show: bool) -> Self {
        self.ctx.show_val_ann = show;
        self
    }

    /// Builder method to set show_val_ann_alternate.
    pub fn show_val_ann_alternate(mut self, show: bool) -> Self {
        self.ctx.show_val_ann_alternate = show;
        self
    }

    /// Builder method to set walker.
    pub fn with_walker(mut self, walker: PrintWalker) -> Self {
        self.ctx.walker = walker;
        self
    }

    /// Builder method to add indentation (spaces) to the prefix.
    pub fn with_indent(mut self, indent: usize) -> Self {
        self.ctx.prefixes.push(" ".repeat(indent));
        self
    }

    /// Returns a reference to the format context.
    pub fn context(&self) -> &FormatContext {
        &self.ctx
    }

    /// Returns a mutable reference to the format context.
    pub fn context_mut(&mut self) -> &mut FormatContext {
        &mut self.ctx
    }

    /// Prints this element to stdout and panics.
    ///
    /// # Panics
    ///
    /// Always.
    pub fn dump(&self) -> ! {
        println!("{self}");
        panic!("dump");
    }
}

impl<T: Format> std::fmt::Display for Formatted<'_, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.item.fmt(f, &self.ctx)
    }
}

enum Separated<T> {
    Content(T),
    Separator,
}

impl<D: Dialect> Format for IR<D> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>, ctx: &FormatContext) -> std::fmt::Result {
        // Compute IR-level metrics
        let max_comment_len = if ctx.show_comments {
            self.walk_ops_linear()
                .filter_map(|op| op.get_comment().map(|c| c.len()))
                .max()
                .unwrap_or(0)
        } else {
            0
        };
        let opid_width = if ctx.show_opid {
            self.n_ops().checked_ilog10().map_or(1, |x| x + 1) as usize
        } else {
            0
        };

        let ops_iter = match ctx.walker {
            PrintWalker::Linear => self.raw_walk_ops_linear().reconcile_1_of_2(),
            PrintWalker::Topo => self.raw_walk_ops_topo().reconcile_2_of_2(),
        };

        let ctx_with_metrics = ctx.with_metrics(opid_width, max_comment_len);

        let mut first = true;
        for opref in ops_iter.filter(|opref| opref.is_active() || ctx.show_erased_ops) {
            if !first {
                writeln!(f)?;
            }
            first = false;
            opref.fmt(f, &ctx_with_metrics)?;
        }
        Ok(())
    }
}

impl<D: Dialect, OpAnn: Annotation, ValAnn: Annotation> Format for AnnIR<'_, D, OpAnn, ValAnn> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>, ctx: &FormatContext) -> std::fmt::Result {
        // Compute IR-level metrics
        let max_comment_len = if ctx.show_comments {
            self.walk_ops_linear()
                .filter_map(|op| op.get_comment().map(|c| c.len()))
                .max()
                .unwrap_or(0)
        } else {
            0
        };
        let opid_width = if ctx.show_opid {
            self.n_ops().checked_ilog10().map_or(1, |x| x + 1) as usize
        } else {
            0
        };

        let ops_iter = match ctx.walker {
            PrintWalker::Linear => self.walk_ops_linear().reconcile_1_of_2(),
            PrintWalker::Topo => self.walk_ops_topological().reconcile_2_of_2(),
        };

        let ctx_with_metrics = ctx.with_metrics(opid_width, max_comment_len);

        let mut first = true;
        for opref in ops_iter.filter(|opref| opref.is_active() || ctx.show_erased_ops) {
            if !first {
                writeln!(f)?;
            }
            first = false;
            opref.fmt(f, &ctx_with_metrics)?;
        }
        Ok(())
    }
}

impl<D: Dialect> Format for OpRef<'_, D> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>, ctx: &FormatContext) -> std::fmt::Result {
        if self.is_inactive() && !ctx.show_erased_ops {
            return Ok(());
        }

        // Use precomputed metrics if available, otherwise compute for this single op
        let max_comment_len = ctx.max_comment_len.unwrap_or_else(|| {
            if ctx.show_comments {
                self.get_comment().map(|c| c.len()).unwrap_or(0)
            } else {
                0
            }
        });
        let opid_width = ctx.opid_width.unwrap_or_else(|| {
            if ctx.show_opid {
                self.get_id().0.checked_ilog10().map_or(1, |x| x + 1) as usize
            } else {
                0
            }
        });

        let line_prefix = ctx.compute_line_prefix(opid_width, max_comment_len);
        let inner_ctx = ctx.with_prefix(&line_prefix);

        // Write the accumulated prefix from parent levels
        write!(f, "{}", ctx.prefix())?;

        if self.is_inactive() {
            write!(f, "\x1b[9m")?;
        }

        // Write opid column
        let has_comments = ctx.show_comments && max_comment_len > 0;
        if ctx.show_opid {
            let np = ctx.nested_prefix();
            if has_comments {
                write!(f, "@{np}{:<width$}   ", self.id.0, width = opid_width)?;
            } else {
                write!(f, "@{np}{:<width$}   |  ", self.id.0, width = opid_width)?;
            }
        }

        // Write comments column
        if has_comments {
            let comment_col_width = max_comment_len + 3;
            if let Some(comment) = self.get_comment() {
                write!(f, "// {:width$}   | ", comment, width = max_comment_len)?;
            } else {
                write!(f, "{:width$}   | ", "", width = comment_col_width)?;
            }
        }

        // Write return values
        self.raw_get_returns_iter()
            .map(Separated::Content)
            .separate_with(|| Separated::Separator)
            .try_for_each(|v| match v {
                Separated::Content(ret) => {
                    write!(f, "%{}{}", ctx.nested_prefix(), ret.id.0)?;
                    if ctx.show_types {
                        write!(f, " : {}", ret.get_type())?;
                    }
                    Ok(())
                }
                Separated::Separator => write!(f, ", "),
            })?;

        if self.get_return_arity() != 0 {
            write!(f, " = ")?;
        }

        // Write operation using Format trait (allows nested IR formatting with context)
        self.operation.fmt(f, &inner_ctx)?;
        write!(f, "(")?;

        // Write arguments
        self.raw_get_args_iter()
            .map(Separated::Content)
            .separate_with(|| Separated::Separator)
            .try_for_each(|v| match v {
                Separated::Content(arg) => {
                    write!(f, "%{}{}", ctx.nested_prefix(), arg.id.0)?;
                    if ctx.show_types {
                        write!(f, " : {}", arg.get_type())?;
                    }
                    Ok(())
                }
                Separated::Separator => write!(f, ", "),
            })?;

        write!(f, ");")?;

        if self.is_inactive() {
            write!(f, "\x1b[29m")?;
        }

        Ok(())
    }
}

impl<D: Dialect> Format for ValRef<'_, D> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>, ctx: &FormatContext) -> std::fmt::Result {
        write!(f, "{}", self.id)?;
        if ctx.show_types {
            write!(f, " : {}", self.get_type())?;
        }
        Ok(())
    }
}

impl<D: Dialect, OpAnn: Annotation, ValAnn: Annotation> Format
    for AnnOpRef<'_, '_, D, OpAnn, ValAnn>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>, ctx: &FormatContext) -> std::fmt::Result {
        if self.is_inactive() && !ctx.show_erased_ops {
            return Ok(());
        }

        // Use precomputed metrics if available, otherwise compute for this single op
        let max_comment_len = ctx.max_comment_len.unwrap_or_else(|| {
            if ctx.show_comments {
                self.get_comment().map(|c| c.len()).unwrap_or(0)
            } else {
                0
            }
        });
        let opid_width = ctx.opid_width.unwrap_or_else(|| {
            if ctx.show_opid {
                self.get_id().0.checked_ilog10().map_or(1, |x| x + 1) as usize
            } else {
                0
            }
        });

        let line_prefix = ctx.compute_line_prefix(opid_width, max_comment_len);
        let inner_ctx = ctx.with_prefix(&line_prefix);

        // Write the accumulated prefix from parent levels
        write!(f, "{}", ctx.prefix())?;

        if self.is_inactive() {
            write!(f, "\x1b[9m")?;
        }

        // Write opid column
        let has_comments = ctx.show_comments && max_comment_len > 0;
        if ctx.show_opid {
            let np = ctx.nested_prefix();
            if has_comments {
                write!(f, "@{np}{:<width$}   ", self.get_id().0, width = opid_width)?;
            } else {
                write!(
                    f,
                    "@{np}{:<width$}   |  ",
                    self.get_id().0,
                    width = opid_width
                )?;
            }
        }

        // Write comments column
        if has_comments {
            let comment_col_width = max_comment_len + 3;
            if let Some(comment) = self.get_comment() {
                write!(f, "// {:width$}   | ", comment, width = max_comment_len)?;
            } else {
                write!(f, "{:width$}   | ", "", width = comment_col_width)?;
            }
        }

        // Write return values
        self.get_returns_iter()
            .map(Separated::Content)
            .separate_with(|| Separated::Separator)
            .try_for_each(|v| match v {
                Separated::Content(ret) => {
                    write!(f, "%{}{}", ctx.nested_prefix(), ret.get_id().0)?;
                    if ctx.show_types {
                        write!(f, " : {}", ret.get_type())?;
                    }
                    Ok(())
                }
                Separated::Separator => write!(f, ", "),
            })?;

        if self.get_return_arity() != 0 {
            write!(f, " = ")?;
        }

        // Write operation using Format trait (allows nested IR formatting with context)
        self.operation.fmt(f, &inner_ctx)?;
        write!(f, "(")?;

        // Write arguments
        self.get_args_iter()
            .map(Separated::Content)
            .separate_with(|| Separated::Separator)
            .try_for_each(|v| match v {
                Separated::Content(arg) => {
                    write!(f, "%{}{}", ctx.nested_prefix(), arg.get_id().0)?;
                    if ctx.show_types {
                        write!(f, " : {}", arg.get_type())?;
                    }
                    Ok(())
                }
                Separated::Separator => write!(f, ", "),
            })?;

        write!(f, ");")?;

        if self.is_inactive() {
            write!(f, "\x1b[29m")?;
        }

        // Write annotations
        let ann_line_prefix = format!(
            "{}{}",
            ctx.prefix(),
            ctx.compute_line_prefix(opid_width, max_comment_len)
        );

        if ctx.show_op_ann && TypeId::of::<OpAnn>() != TypeId::of::<()>() {
            writeln!(f)?;
            write!(f, "{ann_line_prefix}")?;

            let ann_str = if ctx.show_op_ann_alternate {
                format!("{:#?}", self.get_annotation())
            } else {
                format!("{:?}", self.get_annotation())
            };
            let continuation_prefix = format!("{ann_line_prefix}    operation -> ");
            write!(f, "    operation -> ")?;
            write_multiline(f, &ann_str, &continuation_prefix)?;
        }

        if ctx.show_val_ann && TypeId::of::<ValAnn>() != TypeId::of::<()>() {
            for ret in self.get_returns_iter() {
                writeln!(f)?;
                let id = ret.get_id().0;
                let ann = ret.get_annotation();
                let vp = ctx.nested_prefix();

                write!(f, "{ann_line_prefix}")?;

                let (ann_prefix, ann_str) = if ret.is_inactive() {
                    let prefix = format!("    %_{vp}{id} -> ");
                    let ann_str = if ctx.show_val_ann_alternate {
                        format!("{ann:#?}")
                    } else {
                        format!("{ann:?}")
                    };
                    (prefix, ann_str)
                } else {
                    let prefix = format!("    %{vp}{id} -> ");
                    let ann_str = if ctx.show_val_ann_alternate {
                        format!("{ann:#?}")
                    } else {
                        format!("{ann:?}")
                    };
                    (prefix, ann_str)
                };

                let continuation_prefix =
                    format!("{ann_line_prefix}{:width$}", "", width = ann_prefix.len());
                write!(f, "{ann_prefix}")?;
                write_multiline(f, &ann_str, &continuation_prefix)?;
            }
        }

        Ok(())
    }
}

impl<D: Dialect, OpAnn: Annotation, ValAnn: Annotation> Format
    for AnnValRef<'_, '_, D, OpAnn, ValAnn>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>, ctx: &FormatContext) -> std::fmt::Result {
        write!(f, "{}", self.get_id().0)?;
        if ctx.show_types {
            write!(f, " : {}", self.get_type())?;
        }
        if ctx.show_val_ann && TypeId::of::<ValAnn>() != TypeId::of::<()>() {
            if ctx.show_val_ann_alternate {
                write!(f, " -> {:#?}", self.get_annotation())?;
            } else {
                write!(f, " -> {:?}", self.get_annotation())?;
            }
        }
        Ok(())
    }
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
