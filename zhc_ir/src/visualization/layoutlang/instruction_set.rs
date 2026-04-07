use std::{hash::Hash, rc::Rc};

use zhc_utils::{small::SmallVec, svec};

use crate::{
    Dialect, DialectInstructionSet, Format, FormatContext, IR, OpId, OpRef, Signature, ValId, sig,
    visualization::{
        layoutlang::{LayoutDialect, LayoutTypeSystem},
        visual_annotation::VisualAnnotation,
    },
};

#[derive(Debug)]
pub struct OpContent {
    pub args: SmallVec<String>,
    pub returns: SmallVec<String>,
    pub call: String,
    pub comment: Option<String>,
    pub annotation: Option<Rc<dyn VisualAnnotation>>,
}

impl Clone for OpContent {
    fn clone(&self) -> Self {
        Self {
            args: self.args.clone(),
            returns: self.returns.clone(),
            call: self.call.clone(),
            comment: self.comment.clone(),
            annotation: self.annotation.clone(),
        }
    }
}

impl PartialEq for OpContent {
    fn eq(&self, other: &Self) -> bool {
        // Annotation is intentionally excluded from equality
        self.args == other.args
            && self.returns == other.returns
            && self.call == other.call
            && self.comment == other.comment
    }
}

impl Eq for OpContent {}

impl Hash for OpContent {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // Annotation is intentionally excluded from hashing
        self.args.hash(state);
        self.returns.hash(state);
        self.call.hash(state);
        self.comment.hash(state);
    }
}

impl OpContent {
    pub fn from_op<'ir, D: Dialect>(opref: &OpRef<'ir, D>, ctx: &FormatContext) -> Self {
        OpContent {
            args: opref
                .get_args_iter()
                .map(|a| a.fmt_to_string(ctx))
                .collect(),
            returns: opref
                .get_returns_iter()
                .map(|a| a.fmt_to_string(ctx))
                .collect(),
            call: opref.fmt_to_string(&ctx.clone().show_comments(false).show_types(false)),
            comment: None,
            annotation: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum LayoutInstructionSet {
    Operation {
        opid: OpId,
        op: OpContent,
        args: SmallVec<ValId>,
        returns: SmallVec<ValId>,
    },
    Dummy {
        valid: ValId,
    },
    Group {
        ir: IR<LayoutDialect>,
        name: String,
    },
    GroupInput {
        pos: u16,
        valid: ValId,
    },
    GroupOutput {
        pos: u16,
        valid: ValId,
    },
}

impl Format for LayoutInstructionSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>, ctx: &crate::FormatContext) -> std::fmt::Result {
        match self {
            LayoutInstructionSet::Operation { opid, .. } => write!(f, "operation<{opid}>"),
            LayoutInstructionSet::Dummy { valid, .. } => write!(f, "dummy<{valid}>"),
            LayoutInstructionSet::Group { ir, name, .. } => {
                let inner_ctx = ctx.with_prefix("    ").with_next_nested_prefix();
                writeln!(f, "group<\"{}\"> {{", name)?;
                Format::fmt(ir, f, &inner_ctx)?;
                write!(f, "\n{}}}", ctx.prefix())
            }
            LayoutInstructionSet::GroupInput { pos, .. } => write!(f, "group_input<{pos}>"),
            LayoutInstructionSet::GroupOutput { pos, .. } => write!(f, "group_output<{pos}>"),
        }
    }
}

impl DialectInstructionSet for LayoutInstructionSet {
    type TypeSystem = LayoutTypeSystem;

    fn get_signature(&self) -> crate::Signature<Self::TypeSystem> {
        match self {
            LayoutInstructionSet::Operation { op, .. } => Signature(
                svec![LayoutTypeSystem::Value; op.args.len()],
                svec![LayoutTypeSystem::Value; op.returns.len()],
            ),
            LayoutInstructionSet::Dummy { .. } => {
                sig![(LayoutTypeSystem::Value) -> (LayoutTypeSystem::Value)]
            }
            LayoutInstructionSet::Group { ir, .. } => {
                let n_inputs = ir
                    .walk_ops_linear()
                    .filter(|op| {
                        matches!(
                            op.get_instruction(),
                            LayoutInstructionSet::GroupInput { .. }
                        )
                    })
                    .count();
                let n_outputs = ir
                    .walk_ops_linear()
                    .filter(|op| {
                        matches!(
                            op.get_instruction(),
                            LayoutInstructionSet::GroupOutput { .. }
                        )
                    })
                    .count();
                Signature(
                    svec![LayoutTypeSystem::Value; n_inputs],
                    svec![LayoutTypeSystem::Value; n_outputs],
                )
            }
            LayoutInstructionSet::GroupInput { .. } => sig![() -> (LayoutTypeSystem::Value)],
            LayoutInstructionSet::GroupOutput { .. } => sig![(LayoutTypeSystem::Value) -> ()],
        }
    }
}
