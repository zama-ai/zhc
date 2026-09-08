use zhc_ir::{
    IR, OpIdRaw,
    slack::compute_slack,
    visualization::{StyleModifier, VisualAnnotation, draw_ann_ir_to_html},
};
use zhc_langs::ioplang::IopLang;
use zhc_utils::files::FileHandle;
use zhc_utils::graphics::ColorScale;

#[derive(Debug, Clone, PartialEq, Eq)]
struct RelativeSlack {
    slack: OpIdRaw,
    max_slack: OpIdRaw,
}

impl VisualAnnotation for RelativeSlack {
    fn style_modifier(&self) -> Option<StyleModifier> {
        Some(StyleModifier {
            fill_color: Some(
                ColorScale::INVERSE_TRAFFIC_LIGHT
                    .interpolate(self.slack as f64 / self.max_slack as f64),
            ),
            ..Default::default()
        })
    }
}

pub fn draw_slack(ir: &IR<IopLang>) -> FileHandle {
    let ann_ir = compute_slack(&ir);
    let max_slack = ann_ir
        .walk_ops_linear()
        .map(|a| a.get_annotation().0)
        .max()
        .unwrap();
    let ann_ir = ann_ir.map_opann(|op| RelativeSlack {
        slack: op.get_annotation().0,
        max_slack,
    });
    draw_ann_ir_to_html(&ann_ir.view(), None)
}
