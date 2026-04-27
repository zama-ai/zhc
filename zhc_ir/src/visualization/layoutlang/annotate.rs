use std::rc::Rc;

use crate::{
    IR, OpMap,
    visualization::{LayoutDialect, LayoutInstructionSet, visual_annotation::VisualAnnotation},
};

pub fn annotate_layout<OpAnn: VisualAnnotation + Clone>(
    ir: &mut IR<LayoutDialect>,
    annotations: &OpMap<OpAnn>,
) {
    ir.mutate_ops_linear(|op| match op {
        LayoutInstructionSet::Operation { opid, op, .. } => {
            op.annotation = Some(Rc::new(annotations.get(opid).unwrap().clone()));
        }
        LayoutInstructionSet::Group { ir, .. } => {
            annotate_layout(ir, annotations);
        }
        _ => {}
    });
}
