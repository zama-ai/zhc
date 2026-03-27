use zhc_utils::{
    graphics::{Frame, Height, Position, Size, Width, X, Y},
    iter::MultiZip,
};

use crate::{
    AnnIR, AnnIRView, AnnValRef, AnnValUseRef, ValId,
    visualization::{
        LayoutDialect, LayoutInstructionSet, OpContent,
        composition::{
            CompositionSolution, EffectOpClass, GroupClass, InputOpClass, LinkClass, OpBodyClass,
            OpClass, OpCommentClass, OpInputPortClass, OpOutputPortClass, Style, StyleSheet,
        },
    },
};
use zhc_utils::small::SmallVec;

use super::syntax_tree::*;

/// Renders a composed layout IR to SVG.
pub fn draw<'ir>(
    ir: &AnnIR<'ir, LayoutDialect, CompositionSolution, ()>,
    stylesheet: &StyleSheet,
) -> Svg {
    let view = ir.view();
    let bounding_box = extract_bounding_box(&view);

    let groups = gen_groups(&view, stylesheet);
    let operations = gen_operations(&view, stylesheet);
    let links = gen_links(&view, stylesheet);

    Svg {
        width: bounding_box.size.width.0.0,
        height: bounding_box.size.height.0.0,
        elements: vec![groups, operations, links],
    }
}

fn extract_bounding_box<'ir, 'ann>(
    view: &AnnIRView<'ir, 'ann, LayoutDialect, CompositionSolution, ()>,
) -> Frame {
    let mut min_x = f64::MAX;
    let mut min_y = f64::MAX;
    let mut max_x = f64::MIN;
    let mut max_y = f64::MIN;

    for op in view.walk_ops_linear() {
        let frame = op.get_annotation().get_frame();
        min_x = min_x.min(frame.position.x.0);
        min_y = min_y.min(frame.position.y.0);
        max_x = max_x.max(frame.position.x.0 + frame.size.width.0.0);
        max_y = max_y.max(frame.position.y.0 + frame.size.height.0.0);
    }

    Frame {
        position: Position {
            x: X::new(0.0),
            y: Y::new(0.0),
        },
        size: Size {
            width: Width::new(max_x),
            height: Height::new(max_y),
        },
    }
}

fn gen_groups<'ir, 'ann>(
    view: &AnnIRView<'ir, 'ann, LayoutDialect, CompositionSolution, ()>,
    stylesheet: &StyleSheet,
) -> SvgElement {
    let mut elements = Vec::new();
    gen_groups_recursive(view, stylesheet, &mut elements);
    SvgElement::Group {
        elements,
        transform: None,
        id: Some("groups".into()),
        class: None,
    }
}

fn gen_groups_recursive<'ir, 'ann>(
    view: &AnnIRView<'ir, 'ann, LayoutDialect, CompositionSolution, ()>,
    stylesheet: &StyleSheet,
    elements: &mut Vec<SvgElement>,
) {
    for op in view.walk_ops_linear() {
        if let LayoutInstructionSet::Group { ir, name } = op.get_instruction() {
            let frame = op.get_annotation().get_frame();
            let style = stylesheet.get::<GroupClass>();

            // Draw group background
            elements.push(SvgElement::Rect {
                x: frame.position.x.0,
                y: frame.position.y.0,
                width: frame.size.width.0.0,
                height: frame.size.height.0.0,
                fill: Some(style.fill_color.to_string()),
                stroke: Some(style.border_color.to_string()),
                stroke_width: Some(style.border_width.0),
                class: Some("group".into()),
                id: None,
                data_val: None,
            });

            // Draw group title
            elements.push(SvgElement::Text {
                x: frame.position.x.0 + style.padding.0,
                y: frame.position.y.0 + style.padding.0 + style.font_size.0,
                content: name.clone(),
                font_size: style.font_size.0,
                font_family: Some(style.font.0.to_string()),
                fill: Some(style.font_color.to_string()),
                text_anchor: TextAnchor::Start,
                dominant_baseline: DominantBaseline::Auto,
                class: Some("group-title".into()),
                id: None,
            });

            // Recurse into nested IR
            let CompositionSolution::Group { maps, .. } = op.get_annotation() else {
                unreachable!()
            };
            let nested_view = AnnIRView::new(&ir, &maps.0, &maps.1);
            gen_groups_recursive(&nested_view, stylesheet, elements);
        }
    }
}

fn gen_operations<'ir, 'ann>(
    view: &AnnIRView<'ir, 'ann, LayoutDialect, CompositionSolution, ()>,
    stylesheet: &StyleSheet,
) -> SvgElement {
    let mut elements = Vec::new();
    gen_operations_recursive(view, stylesheet, &mut elements);
    SvgElement::Group {
        elements,
        transform: None,
        id: Some("operations".into()),
        class: None,
    }
}

fn gen_operations_recursive<'ir, 'ann>(
    view: &AnnIRView<'ir, 'ann, LayoutDialect, CompositionSolution, ()>,
    stylesheet: &StyleSheet,
    elements: &mut Vec<SvgElement>,
) {
    for op in view.walk_ops_linear() {
        match op.get_instruction() {
            LayoutInstructionSet::Operation {
                op: ref orig_op,
                args: ref arg_valids,
                returns: ref ret_valids,
                ..
            } => {
                let ann = op.get_annotation();
                elements.push(gen_operation_box(
                    orig_op, arg_valids, ret_valids, ann, stylesheet,
                ));
            }
            LayoutInstructionSet::Group { ir, .. } => {
                // Recurse into nested IR
                if let CompositionSolution::Group { maps, .. } = op.get_annotation() {
                    let nested_view = AnnIRView::new(&ir, &maps.0, &maps.1);
                    gen_operations_recursive(&nested_view, stylesheet, elements);
                }
            }
            // Dummy, GroupInput, GroupOutput are invisible
            _ => {}
        }
    }
}

fn gen_operation_box(
    orig_op: &OpContent,
    arg_valids: &SmallVec<ValId>,
    ret_valids: &SmallVec<ValId>,
    ann: &CompositionSolution,
    stylesheet: &StyleSheet,
) -> SvgElement {
    let CompositionSolution::Op {
        sol,
        args,
        rets,
        body,
        comment,
    } = ann
    else {
        unreachable!("Expected CompositionSolution::Op");
    };

    let frame = sol.frame.clone();
    let is_input = orig_op.args.len() == 0;
    let is_effect = orig_op.returns.len() == 0;

    let (style, class_name): (&Style, &str) = if is_input {
        (stylesheet.get::<InputOpClass>(), "input-op")
    } else if is_effect {
        (stylesheet.get::<EffectOpClass>(), "effect-op")
    } else {
        (stylesheet.get::<OpClass>(), "op")
    };

    let hat_height = 10.0;
    let mut elements = Vec::new();
    let body_frame = &body.frame;

    // Top hat for input ops
    if is_input {
        elements.push(gen_hat_top(
            frame.position.x.0,
            frame.position.y.0,
            frame.size.width.0.0,
            hat_height,
            &style.fill_color.to_string(),
            &style.border_color.to_string(),
            style.border_width.0,
        ));
    }

    // Main rectangle
    elements.push(SvgElement::Rect {
        x: frame.position.x.0,
        y: frame.position.y.0,
        width: frame.size.width.0.0,
        height: frame.size.height.0.0,
        fill: Some(style.fill_color.to_string()),
        stroke: Some(style.border_color.to_string()),
        stroke_width: Some(style.border_width.0),
        class: Some(class_name.into()),
        id: None,
        data_val: None,
    });

    // Input ports
    let input_port_style = stylesheet.get::<OpInputPortClass>();
    for (idx, arg_sol) in args.iter().enumerate() {
        let arg_frame = &arg_sol.frame;
        // Port rectangle with data-val for hover highlighting
        elements.push(SvgElement::Rect {
            x: arg_frame.position.x.0,
            y: arg_frame.position.y.0,
            width: arg_frame.size.width.0.0,
            height: arg_frame.size.height.0.0,
            fill: Some(input_port_style.fill_color.to_string()),
            stroke: Some(input_port_style.border_color.to_string()),
            stroke_width: Some(input_port_style.border_width.0),
            class: Some("input-port".into()),
            id: None,
            data_val: arg_valids.as_slice().get(idx).copied(),
        });
        // Port label - top-left aligned with padding
        let label = orig_op
            .args
            .as_slice()
            .get(idx)
            .map(|v| format!("{}", v))
            .unwrap_or_else(|| format!("#{}", idx));
        elements.push(SvgElement::Text {
            x: arg_frame.position.x.0 + input_port_style.padding.0,
            y: arg_frame.position.y.0 + input_port_style.padding.0,
            content: label,
            font_size: input_port_style.font_size.0,
            font_family: Some(input_port_style.font.0.to_string()),
            fill: Some(input_port_style.font_color.to_string()),
            text_anchor: TextAnchor::Start,
            dominant_baseline: DominantBaseline::Hanging,
            class: None,
            id: None,
        });
    }

    // Separator after inputs (if any) - at midpoint between inputs bottom and body top
    if !args.is_empty() {
        let last_arg = &args[args.len() - 1].frame;
        let sep_y = (last_arg.bottom_left().y.0 + body_frame.top_left().y.0) / 2.0;
        elements.push(gen_hseparator(
            frame.position.x.0,
            sep_y,
            frame.size.width.0.0,
            &style.border_color.to_string(),
            style.border_width.0,
        ));
    }

    // Body
    let body_style = stylesheet.get::<OpBodyClass>();
    let body_text = orig_op.call.clone();
    for (line_index, line) in body_text.lines().enumerate() {
        elements.push(SvgElement::Text {
            x: body_frame.position.x.0 + body_style.padding.0,
            y: body_frame.position.y.0
                + body_style.padding.0
                + (line_index as f64 * body_style.font_size.0 * 1.2),
            content: line.to_string(),
            font_size: body_style.font_size.0,
            font_family: Some(body_style.font.0.to_string()),
            fill: Some(body_style.font_color.to_string()),
            text_anchor: TextAnchor::from(body_style.font_halign),
            dominant_baseline: DominantBaseline::Hanging,
            class: None,
            id: None,
        });
    }

    // Comment (if present in the original op)
    if let Some(comment_text) = &orig_op.comment {
        // Only render if we also have the annotation frame
        if let Some(comment_sol) = comment {
            let comment_style = stylesheet.get::<OpCommentClass>();
            let comment_frame = &comment_sol.frame;

            // Separator before comment - at midpoint between body bottom and comment top
            let sep_y = (body_frame.bottom_left().y.0 + comment_frame.top_left().y.0) / 2.0;
            elements.push(gen_hseparator(
                frame.position.x.0,
                sep_y,
                frame.size.width.0.0,
                &style.border_color.to_string(),
                style.border_width.0,
            ));

            for (line_index, line) in comment_text.lines().enumerate() {
                elements.push(SvgElement::Text {
                    x: comment_frame.position.x.0 + comment_style.padding.0,
                    y: comment_frame.position.y.0
                        + comment_style.padding.0
                        + (line_index as f64 * comment_style.font_size.0 * 1.2),
                    content: line.to_string(),
                    font_size: comment_style.font_size.0,
                    font_family: Some(comment_style.font.0.to_string()),
                    fill: Some(comment_style.font_color.to_string()),
                    text_anchor: TextAnchor::from(comment_style.font_halign),
                    dominant_baseline: DominantBaseline::Hanging,
                    class: Some("comment".into()),
                    id: None,
                });
            }
        }
    }

    // Separator before outputs (if any) - at midpoint between previous section and outputs top
    if !rets.is_empty() {
        let first_ret = &rets[0].frame;
        // Previous section is either comment (if present) or body
        let prev_bottom = if let Some(comment_sol) = comment {
            if orig_op.comment.is_some() {
                comment_sol.frame.bottom_left().y.0
            } else {
                body_frame.bottom_left().y.0
            }
        } else {
            body_frame.bottom_left().y.0
        };
        let sep_y = (prev_bottom + first_ret.top_left().y.0) / 2.0;
        elements.push(gen_hseparator(
            frame.position.x.0,
            sep_y,
            frame.size.width.0.0,
            &style.border_color.to_string(),
            style.border_width.0,
        ));
    }

    // Output ports
    let output_port_style = stylesheet.get::<OpOutputPortClass>();
    for (idx, ret_sol) in rets.iter().enumerate() {
        let ret_frame = &ret_sol.frame;
        // Port rectangle with data-val for hover highlighting
        elements.push(SvgElement::Rect {
            x: ret_frame.position.x.0,
            y: ret_frame.position.y.0,
            width: ret_frame.size.width.0.0,
            height: ret_frame.size.height.0.0,
            fill: Some(output_port_style.fill_color.to_string()),
            stroke: Some(output_port_style.border_color.to_string()),
            stroke_width: Some(output_port_style.border_width.0),
            class: Some("output-port".into()),
            id: None,
            data_val: ret_valids.as_slice().get(idx).copied(),
        });
        // Port label - top-left aligned with padding
        let label = orig_op
            .returns
            .as_slice()
            .get(idx)
            .map(|v| format!("{}", v))
            .unwrap_or_else(|| format!("#{}", idx));
        elements.push(SvgElement::Text {
            x: ret_frame.position.x.0 + output_port_style.padding.0,
            y: ret_frame.position.y.0 + output_port_style.padding.0,
            content: label,
            font_size: output_port_style.font_size.0,
            font_family: Some(output_port_style.font.0.to_string()),
            fill: Some(output_port_style.font_color.to_string()),
            text_anchor: TextAnchor::Start,
            dominant_baseline: DominantBaseline::Hanging,
            class: None,
            id: None,
        });
    }

    // Bottom hat for effect ops
    if is_effect {
        elements.push(gen_hat_bottom(
            frame.position.x.0,
            frame.position.y.0 + frame.size.height.0.0,
            frame.size.width.0.0,
            hat_height,
            &style.fill_color.to_string(),
            &style.border_color.to_string(),
            style.border_width.0,
        ));
    }

    SvgElement::Group {
        elements,
        transform: None,
        id: None,
        class: Some("operation".into()),
    }
}

fn gen_hseparator(x: f64, y: f64, width: f64, color: &str, thickness: f64) -> SvgElement {
    SvgElement::Rect {
        x,
        y: y - thickness / 2.0,
        width,
        height: thickness,
        fill: Some(color.into()),
        stroke: None,
        stroke_width: None,
        class: None,
        id: None,
        data_val: None,
    }
}

fn gen_links<'ir, 'ann>(
    view: &AnnIRView<'ir, 'ann, LayoutDialect, CompositionSolution, ()>,
    stylesheet: &StyleSheet,
) -> SvgElement {
    let mut elements = Vec::new();
    gen_links_recursive(view, stylesheet, &mut elements);
    SvgElement::Group {
        elements,
        transform: None,
        id: Some("links".into()),
        class: None,
    }
}

fn gen_links_recursive<'ir, 'ann>(
    view: &AnnIRView<'ir, 'ann, LayoutDialect, CompositionSolution, ()>,
    stylesheet: &StyleSheet,
    elements: &mut Vec<SvgElement>,
) {
    // Collect all paths with their ValIds for hover highlighting
    let mut paths: Vec<(ValId, Vec<Position>)> = Vec::new();
    collect_link_paths(view, &mut paths);

    let style = stylesheet.get::<LinkClass>();

    // Draw each path as a bezier curve with data-val
    for (valid, waypoints) in paths {
        if waypoints.len() >= 2 {
            elements.push(gen_bezier_path(&waypoints, valid, style));
        }
    }

    // Recurse into groups
    for op in view.walk_ops_linear() {
        if let LayoutInstructionSet::Group { ir, .. } = op.get_instruction() {
            if let CompositionSolution::Group { maps, .. } = op.get_annotation() {
                let nested_view = AnnIRView::new(&ir, &maps.0, &maps.1);
                gen_links_recursive(&nested_view, stylesheet, elements);
            }
        }
    }
}

fn collect_link_paths<'ir, 'ann>(
    view: &AnnIRView<'ir, 'ann, LayoutDialect, CompositionSolution, ()>,
    paths: &mut Vec<(ValId, Vec<Position>)>,
) {
    // For each operation, trace its output values forward with their ValIds
    for op in view.walk_ops_linear() {
        match (op.get_instruction(), op.get_annotation()) {
            (
                LayoutInstructionSet::Operation { returns, .. },
                CompositionSolution::Op { rets, .. },
            ) => {
                for (idx, (ret, ret_sol)) in (op.get_returns_iter(), rets.iter()).mzip().enumerate()
                {
                    let start_pos = ret_sol.frame.bottom_center();
                    let valid = returns.as_slice().get(idx).copied().unwrap();
                    trace_paths_forward(ret, valid, vec![start_pos], paths);
                }
            }
            (
                LayoutInstructionSet::Group { ir, .. },
                CompositionSolution::Group { outputs, .. },
            ) => {
                // Find GroupOutput nodes inside the group to get ValIds (copy values)
                let group_outputs: Vec<(u16, ValId)> = ir
                    .walk_ops_linear()
                    .filter_map(|inner_op| {
                        if let LayoutInstructionSet::GroupOutput { pos, valid } =
                            inner_op.get_instruction()
                        {
                            Some((pos, valid))
                        } else {
                            None
                        }
                    })
                    .collect();

                for (idx, (ret, out_sol)) in
                    (op.get_returns_iter(), outputs.iter()).mzip().enumerate()
                {
                    let start_pos = out_sol.frame.bottom_center();
                    // Find the GroupOutput with matching position
                    if let Some((_, valid)) =
                        group_outputs.iter().find(|(pos, _)| *pos as usize == idx)
                    {
                        trace_paths_forward(ret, *valid, vec![start_pos], paths);
                    }
                }
            }
            (
                LayoutInstructionSet::GroupInput { valid, .. },
                CompositionSolution::GroupInput { sol },
            ) => {
                for ret in op.get_returns_iter() {
                    // GroupInput is a boundary point - use center
                    let start_pos = sol.frame.center();
                    trace_paths_forward(ret, valid, vec![start_pos], paths);
                }
            }
            _ => {}
        }
    }
}

fn trace_paths_forward<'ir, 'ann>(
    val: AnnValRef<'ir, 'ann, LayoutDialect, CompositionSolution, ()>,
    valid: ValId,
    waypoints: Vec<Position>,
    paths: &mut Vec<(ValId, Vec<Position>)>,
) {
    for AnnValUseRef { opref, position } in val.get_uses_iter() {
        match (opref.get_instruction(), opref.get_annotation()) {
            (LayoutInstructionSet::Dummy { .. }, CompositionSolution::Dummy { sol }) => {
                let mut new_waypoints = waypoints.clone();
                new_waypoints.push(sol.frame.center());
                if let Some(next_val) = opref.get_returns_iter().next() {
                    trace_paths_forward(next_val, valid, new_waypoints, paths);
                }
            }
            (LayoutInstructionSet::Operation { .. }, CompositionSolution::Op { args, .. }) => {
                let arg_sol = args.as_slice().get(position as usize).unwrap();
                let end_pos = arg_sol.frame.top_center();
                let mut final_path = waypoints.clone();
                final_path.push(end_pos);
                paths.push((valid, final_path));
            }
            (LayoutInstructionSet::Group { .. }, CompositionSolution::Group { inputs, .. }) => {
                let in_sol = inputs.as_slice().get(position as usize).unwrap();
                let end_pos = in_sol.frame.top_center();
                let mut final_path = waypoints.clone();
                final_path.push(end_pos);
                paths.push((valid, final_path));
            }
            (
                LayoutInstructionSet::GroupOutput { .. },
                CompositionSolution::GroupOutput { sol },
            ) => {
                let end_pos = sol.frame.center();
                let mut final_path = waypoints.clone();
                final_path.push(end_pos);
                paths.push((valid, final_path));
            }
            _ => {}
        }
    }
}

fn gen_bezier_path(waypoints: &[Position], valid: ValId, style: &Style) -> SvgElement {
    if waypoints.len() < 2 {
        return SvgElement::Group {
            elements: vec![],
            transform: None,
            id: None,
            class: None,
        };
    }

    let mut commands = Vec::new();
    commands.push(PathCommand::MoveTo(waypoints[0]));

    // Generate cubic bezier segments between consecutive waypoints
    for i in 0..waypoints.len() - 1 {
        let start = waypoints[i];
        let end = waypoints[i + 1];

        // Control points: vertical offset for smooth curves
        let dy = (end.y.0 - start.y.0) / 3.0;
        let cp1 = Position {
            x: start.x,
            y: Y::new(start.y.0 + dy),
        };
        let cp2 = Position {
            x: end.x,
            y: Y::new(end.y.0 - dy),
        };

        commands.push(PathCommand::CubicTo(cp1, cp2, end));
    }

    // Wider invisible path for easier hit detection (carries the data-val)
    let hitarea = SvgElement::Path {
        commands: commands.clone(),
        fill: Some("none".into()),
        stroke: Some("transparent".into()),
        stroke_width: Some(15.0),
        class: Some("link-hitarea".into()),
        id: None,
        title: None,
        data_val: Some(valid),
    };

    // Visible thin path (also carries data-val for highlighting)
    let visible = SvgElement::Path {
        commands,
        fill: Some("none".into()),
        stroke: Some(style.border_color.to_string()),
        stroke_width: Some(style.border_width.0),
        class: Some("link".into()),
        id: None,
        title: None,
        data_val: Some(valid),
    };

    // Group both with data-val for hover detection
    SvgElement::Group {
        elements: vec![hitarea, visible],
        transform: None,
        id: None,
        class: Some(format!("link-group")),
    }
}

fn gen_hat_top(
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    fill: &str,
    stroke: &str,
    stroke_width: f64,
) -> SvgElement {
    let rx = width / 2.0;
    let ry = height;
    SvgElement::Path {
        commands: vec![
            PathCommand::MoveTo(Position {
                x: X::new(x),
                y: Y::new(y),
            }),
            PathCommand::EllipticalArc {
                rx,
                ry,
                x_axis_rotation: 0.0,
                large_arc: false,
                sweep: true,
                end: Position {
                    x: X::new(x + width),
                    y: Y::new(y),
                },
            },
            PathCommand::ClosePath,
        ],
        fill: Some(fill.into()),
        stroke: Some(stroke.into()),
        stroke_width: Some(stroke_width),
        class: None,
        id: None,
        title: None,
        data_val: None,
    }
}

fn gen_hat_bottom(
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    fill: &str,
    stroke: &str,
    stroke_width: f64,
) -> SvgElement {
    let rx = width / 2.0;
    let ry = height;
    SvgElement::Path {
        commands: vec![
            PathCommand::MoveTo(Position {
                x: X::new(x),
                y: Y::new(y),
            }),
            PathCommand::EllipticalArc {
                rx,
                ry,
                x_axis_rotation: 0.0,
                large_arc: false,
                sweep: false,
                end: Position {
                    x: X::new(x + width),
                    y: Y::new(y),
                },
            },
            PathCommand::ClosePath,
        ],
        fill: Some(fill.into()),
        stroke: Some(stroke.into()),
        stroke_width: Some(stroke_width),
        class: None,
        id: None,
        title: None,
        data_val: None,
    }
}
