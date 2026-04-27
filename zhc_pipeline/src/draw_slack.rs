use std::path::Path;

use zhc_builder::Builder;
use zhc_ir::{
    OpIdRaw,
    slack::compute_slack,
    visualization::{StyleModifier, VisualAnnotation, draw_ann_ir_to_html},
};
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

/// Renders a slack heatmap of the IR as an interactive HTML file.
///
/// Slack measures how much an operation can be delayed without affecting the circuit's
/// critical path. This function computes the slack for every operation, normalizes the
/// values, and renders the IR with a color gradient: red indicates operations on or near
/// the critical path (low slack), while green indicates operations with scheduling
/// flexibility (high slack).
///
/// The resulting HTML file supports interactive features such as zooming and panning.
///
/// # Panics
///
/// Panics if the IR is empty (no operations), or if the file cannot be written to the
/// given path.
///
/// # Examples
///
/// ```rust,no_run
/// # use zhc_pipeline::draw_slack;
/// # use zhc_builder::*;
/// # let builder = Builder::new(CiphertextBlockSpec(2, 2));
/// draw_slack(&builder, "slack_heatmap.html");
/// ```
pub fn draw_slack(builder: &Builder, path: impl AsRef<Path>) {
    let ir = builder.ir();
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
    draw_ann_ir_to_html(&ann_ir, builder.hierarchy(), path);
}
