use std::rc::Rc;

use crate::{
    IR, OpMap, ValMap,
    visualization::{LayoutDialect, LayoutInstructionSet, visual_annotation::VisualAnnotation},
};

/// Attaches visual annotations to every operation node of a layout IR.
///
/// Recursively walks `ir`, including nested groups, and sets each
/// [`Operation`](LayoutInstructionSet::Operation) node's annotation to a copy of the
/// `annotations` entry keyed by the node's original [`OpId`](crate::OpId). Other node kinds are
/// left untouched.
///
/// # Panics
///
/// Panics if `annotations` has no entry for the original operation id of some operation node.
pub fn annotate_layout<OpAnn: VisualAnnotation + Clone>(
    ir: &mut IR<LayoutDialect>,
    annotations: &OpMap<OpAnn>,
) {
    ir.replace_ops_instr_linear(|op| {
        let mut op = op.get_instruction().clone();
        match &mut op {
            LayoutInstructionSet::Operation { opid, op, .. } => {
                op.annotation = Some(Rc::new(annotations.get(*opid).unwrap().clone()));
            }
            LayoutInstructionSet::Group { ir, .. } => {
                annotate_layout(ir, annotations);
            }
            _ => {}
        }
        op
    });
}

/// Attaches visual annotations to every port of every operation node of a layout IR.
///
/// Recursively walks `ir`, including nested groups, and fills each
/// [`Operation`](LayoutInstructionSet::Operation) node's per-port annotations with copies of the
/// `annotations` entries keyed by the ports' original [`ValId`](crate::ValId)s. Other node kinds
/// are left untouched.
///
/// # Panics
///
/// Panics if `annotations` has no entry for the original value id of some port.
pub fn annotate_layout_vals<ValAnn: VisualAnnotation + Clone>(
    ir: &mut IR<LayoutDialect>,
    annotations: &ValMap<ValAnn>,
) {
    ir.replace_ops_instr_linear(|op| {
        let mut op = op.get_instruction().clone();
        match &mut op {
            LayoutInstructionSet::Operation {
                op, args, returns, ..
            } => {
                let annotate = |valid: &crate::ValId| -> Option<Rc<dyn VisualAnnotation>> {
                    Some(Rc::new(annotations.get(*valid).unwrap().clone()))
                };
                op.arg_annotations = args.iter().map(annotate).collect();
                op.return_annotations = returns.iter().map(annotate).collect();
            }
            LayoutInstructionSet::Group { ir, .. } => {
                annotate_layout_vals(ir, annotations);
            }
            _ => {}
        }
        op
    });
}
