use eframe::{
    egui::{
        Color32, FontFamily, FontId, Pos2, Rect, Shape, Vec2, epaint::TextShape,
    },
    epaint::CornerRadiusF32,
};
use egui_graphs::{
    DisplayNode,
    NodeProps,
};
use petgraph::{
    EdgeType,
    stable_graph::IndexType,
};

use super::{Format, GraphFmt, GraphShow};

#[derive(Clone)]
pub struct NodeShape<T: Clone> {
    props: NodeProps<T>,
    label: String,
    loc: Pos2,

    size_x: f32,
    size_y: f32,
}

impl<T: Clone> From<NodeProps<T>> for NodeShape<T> {
    fn from(node_props: NodeProps<T>) -> Self {
        Self {
            props: node_props.clone(),
            label: node_props.label.clone(),
            loc: node_props.location(),

            size_x: 0.,
            size_y: 0.,
        }
    }
}

impl<T: GraphShow + GraphFmt, E: Clone, Ty: EdgeType, Ix: IndexType> DisplayNode<T, E, Ty, Ix>
    for NodeShape<T>
{
    fn is_inside(&self, pos: Pos2) -> bool {
        let rect = Rect::from_center_size(self.loc, Vec2::new(self.size_x, self.size_y));

        rect.contains(pos)
    }

    fn closest_boundary_point(&self, dir: Vec2) -> Pos2 {
        find_intersection(self.loc, self.size_x / 2., self.size_y / 2., dir)
    }

    fn shapes(&mut self, ctx: &egui_graphs::DrawContext) -> Vec<Shape> {
        // find node center location on the screen coordinates
        let center = ctx.meta.canvas_to_screen_pos(self.loc);
        let color = ctx.ctx.style().visuals.text_color();

        // create label
        let galley = ctx.ctx.fonts(|f| {
            f.layout_no_wrap(
                self.props.payload.fmt_short(),
                FontId::new(ctx.meta.canvas_to_screen_size(10.), FontFamily::Monospace),
                color,
            )
        });

        // we need to offset label by half its size to place it in the center of the rect
        let offset = Vec2::new(-galley.size().x / 2., -galley.size().y / 2.);

        // create the shape and add it to the layers
        let shape_label = TextShape::new(center + offset, galley, color);

        // Create surrounding shape for clickable area
        let surrounding_rect = shape_label.visual_bounding_rect();
        let shape = format_shape(
            surrounding_rect,
            &self.props.payload.format(),
            self.props.payload.color(),
        );

        // update self size
        self.size_x = surrounding_rect.size().x;
        self.size_y = surrounding_rect.size().y;

        vec![shape, shape_label.into()]
    }

    fn update(&mut self, state: &NodeProps<T>) {
        self.label.clone_from(&state.label);
        self.loc = state.location();
    }
}

fn find_intersection(center: Pos2, size_x: f32, size_y: f32, direction: Vec2) -> Pos2 {
    if (direction.x.abs() * size_y) > (direction.y.abs() * size_x) {
        // intersects left or right side
        let x = if direction.x > 0.0 {
            center.x + size_x / 2.0
        } else {
            center.x - size_x / 2.0
        };
        let y = center.y + direction.y / direction.x * (x - center.x);
        Pos2::new(x, y)
    } else {
        // intersects top or bottom side
        let y = if direction.y > 0.0 {
            center.y + size_y / 2.0
        } else {
            center.y - size_y / 2.0
        };
        let x = center.x + direction.x / direction.y * (y - center.y);
        Pos2::new(x, y)
    }
}

fn format_shape(rect: Rect, format: &Format, color: Color32) -> Shape {
    match format {
        Format::Rectangle => Shape::rect_filled(rect, CornerRadiusF32::default(), color),
        Format::Ellipse => {
            let top_left = rect.min;
            let bottom_right = rect.max;
            let radius = Vec2::new(bottom_right.x - top_left.x, bottom_right.y - top_left.y);

            Shape::ellipse_filled(rect.center(), radius, color)
        }
        Format::Circle => {
            // Circle
            let top_left = rect.min;
            let bottom_right = rect.max;
            let radius = (bottom_right.x - top_left.x).max(bottom_right.y - top_left.y);
            Shape::circle_filled(rect.center(), radius, color)
        }
        Format::Diamond => todo!("Properly handle Diamond"),
    }
}
