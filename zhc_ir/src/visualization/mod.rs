use crate::{
    AnnIR, Annotation, Dialect, IR, OpMap,
    visualization::{svg::Svg, visual_annotation::VisualAnnotation},
};
use std::path::Path;

mod composition;
mod hierarchy;
mod html;
mod layer_map;
mod layoutlang;
mod placement;
mod svg;
mod visual_annotation;

pub use hierarchy::*;
pub use layer_map::*;
pub use layoutlang::*;

#[cfg(test)]
mod test;

fn draw_ir<D: Dialect>(ir: &IR<D>, hierarchy_ann: OpMap<Hierarchy>) -> Svg {
    let ann_ir = AnnIR::new(ir, hierarchy_ann, ir.filled_valmap(()));
    let layout_ir = generate_layout_ir(&ann_ir);

    let placed_ir = placement::place(&layout_ir);
    let scene = composition::compose(&placed_ir);
    svg::draw(&scene)
}

fn draw_ann_ir<D: Dialect, OpAnn: Annotation + VisualAnnotation, ValAnn: Annotation>(
    ir: &AnnIR<D, OpAnn, ValAnn>,
    hierarchy_ann: OpMap<Hierarchy>,
) -> Svg {
    let ann_ir = AnnIR::new(ir, hierarchy_ann, ir.filled_valmap(()));
    let mut layout_ir = generate_layout_ir(&ann_ir);
    annotate_layout(&mut layout_ir, ir.op_annotations());
    let placed_ir = placement::place(&layout_ir);
    let scene = composition::compose(&placed_ir);
    svg::draw(&scene)
}

/// Draws an IR diagram to an SVG file.
///
/// Takes an IR with hierarchy annotations and produces an SVG visualization
/// showing the compound graph structure with operations, groups, and edges.
pub fn draw_ir_to_svg<D: Dialect>(
    ir: &IR<D>,
    hierarchy_ann: OpMap<Hierarchy>,
    path: impl AsRef<Path>,
) {
    let svg_output = draw_ir(ir, hierarchy_ann);
    let svg_content = format!("{}", svg_output);
    std::fs::write(path, svg_content).expect("Failed to write SVG file");
}

pub fn draw_ann_ir_to_svg<D: Dialect, OpAnn: Annotation + VisualAnnotation, ValAnn: Annotation>(
    ir: &AnnIR<D, OpAnn, ValAnn>,
    hierarchy_ann: OpMap<Hierarchy>,
    path: impl AsRef<Path>,
) {
    let svg_output = draw_ann_ir(ir, hierarchy_ann);
    let svg_content = format!("{}", svg_output);
    std::fs::write(path, svg_content).expect("Failed to write SVG file");
}

/// Draws an IR diagram to an HTML file with interactive zoom/pan.
///
/// Similar to `draw_ir` but outputs an HTML document that embeds the SVG
/// with better viewport handling and transform-based zoom/pan.
pub fn draw_ir_to_html<D: Dialect>(
    ir: &IR<D>,
    hierarchy_ann: OpMap<Hierarchy>,
    path: impl AsRef<Path>,
) {
    let svg_output = draw_ir(ir, hierarchy_ann);
    let html_output = html::wrap_svg(svg_output);
    let html_content = format!("{}", html_output);
    std::fs::write(path, html_content).expect("Failed to write HTML file");
}

pub fn draw_ann_ir_to_html<D: Dialect, OpAnn: Annotation + VisualAnnotation, ValAnn: Annotation>(
    ir: &AnnIR<D, OpAnn, ValAnn>,
    hierarchy_ann: OpMap<Hierarchy>,
    path: impl AsRef<Path>,
) {
    let svg_output = draw_ann_ir(ir, hierarchy_ann);
    let html_output = html::wrap_svg(svg_output);
    let html_content = format!("{}", html_output);
    std::fs::write(path, html_content).expect("Failed to write HTML file");
}
