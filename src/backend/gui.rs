use eframe::{
    App, CreationContext, NativeOptions,
    egui::{
        Color32, Context, FontFamily, FontId, Pos2, Rect, Shape, Stroke, Vec2, epaint::TextShape, *,
    },
    epaint::CornerRadiusF32,
    run_native,
};
use egui_graphs::{
    DefaultEdgeShape, DisplayEdge, DisplayNode, DrawContext, EdgeProps, Graph, GraphView, Node,
    NodeProps, SettingsInteraction, SettingsNavigation, SettingsStyle,
};
use petgraph::{
    Directed, EdgeType,
    stable_graph::{DefaultIx, EdgeIndex, IndexType, NodeIndex, StableGraph},
    visit::EdgeRef,
};

use crate::ir::{IrCell, IrDag, IrOperation, OpKind};
pub struct DagViewer<E: Clone + std::fmt::Display> {
    graph: Graph<NodePayload, E, Directed, DefaultIx, NodeShapeFlex, EdgeShapeFlex>,
}

#[derive(Clone, Debug)]
pub struct NodePayload {
    pub label: String,
    pub kind: OpKind,
    pub op: IrOperation,
}
impl<E: Clone + std::fmt::Display> From<&IrDag<IrOperation, E>> for DagViewer<E> {
    fn from(ir_dag: &IrDag<IrOperation, E>) -> Self {
        let petgraph_dag = ir_dag.get_graph();

        // Create a StableGraph compatible with egui_graphs
        let mut stable_graph: StableGraph<NodePayload, E> = StableGraph::new();
        let mut node_map = std::collections::HashMap::new();

        // Add nodes
        for node_idx in petgraph_dag.node_indices() {
            let op = petgraph_dag[node_idx].clone();

            let kind = OpKind::from(&op);
            let label = op
                .to_string()
                .split(' ')
                .next()
                .expect("Invalid IrOperation format")
                .to_string();
            let payload = NodePayload { label, kind, op };
            let new_idx = stable_graph.add_node(payload);
            node_map.insert(node_idx, new_idx);
        }

        // Add edges
        for edge_ref in petgraph_dag.edge_references() {
            let src = edge_ref.source();
            let dst = edge_ref.target();
            stable_graph.add_edge(node_map[&src], node_map[&dst], edge_ref.weight().clone());
        }

        // Convert to egui_graphs Graph
        Self {
            graph: Graph::from(&stable_graph),
        }
    }
}

pub struct ViewerApp<E: Clone + std::fmt::Display> {
    dag: DagViewer<E>,
}

impl<E: Clone + std::fmt::Display> ViewerApp<E> {
    pub fn new(dag: DagViewer<E>, _cc: &CreationContext<'_>) -> Self {
        Self { dag }
    }
}

impl<E: Clone + std::fmt::Display> App for ViewerApp<E> {
    fn update(&mut self, ctx: &Context, _: &mut eframe::Frame) {
        TopBottomPanel::bottom("Node properties").show(ctx, |ui| {
            let props = if !self.dag.graph.selected_nodes().is_empty() {
                let idx = self.dag.graph.selected_nodes().first().unwrap();
                format!("{}", self.dag.graph.node(*idx).unwrap().payload().op)
            } else if !self.dag.graph.selected_edges().is_empty() {
                let idx = self.dag.graph.selected_edges().first().unwrap();
                format!("{}", self.dag.graph.edge(*idx).unwrap().payload())
            } else {
                "".to_string()
            };
            ui.label(props);
        });
        eframe::egui::CentralPanel::default().show(ctx, |ui| {
            // Use custom node display for different shapes
            type L = egui_graphs::LayoutHierarchical;
            type S = egui_graphs::LayoutStateHierarchical;
            let mut graph_view =
                GraphView::<NodePayload, E, _, _, _, _, S, L>::new(&mut self.dag.graph)
                    .with_navigations(
                        &SettingsNavigation::default()
                            .with_fit_to_screen_enabled(true)
                            .with_zoom_and_pan_enabled(true),
                    )
                    .with_styles(&SettingsStyle::default().with_labels_always(false))
                    .with_interactions(
                        &SettingsInteraction::default()
                            .with_node_selection_enabled(true)
                            .with_edge_selection_enabled(true),
                    );

            ui.add(&mut graph_view);
        });
    }
}

pub fn dag_display<E: Clone + std::fmt::Display>(ir_dag: &IrDag<IrOperation, E>, app_name: &str) {
    let dag_viewer = DagViewer::from(ir_dag);
    run_native(
        app_name,
        NativeOptions::default(),
        Box::new(|cc| Ok(Box::new(ViewerApp::new(dag_viewer, cc)))),
    )
    .unwrap();
}

#[derive(Clone)]
pub struct NodeShapeFlex {
    props: NodeProps<NodePayload>,
    label: String,
    loc: Pos2,

    size_x: f32,
    size_y: f32,
}

impl From<NodeProps<NodePayload>> for NodeShapeFlex {
    fn from(node_props: NodeProps<NodePayload>) -> Self {
        Self {
            props: node_props.clone(),
            label: node_props.label.clone(),
            loc: node_props.location(),

            size_x: 0.,
            size_y: 0.,
        }
    }
}

impl<E: Clone, Ty: EdgeType, Ix: IndexType> DisplayNode<NodePayload, E, Ty, Ix> for NodeShapeFlex {
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
                self.props.payload.label.clone(),
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
        let shape = custom_shape(surrounding_rect, &self.props.payload.kind);

        // update self size
        self.size_x = surrounding_rect.size().x;
        self.size_y = surrounding_rect.size().y;

        vec![shape, shape_label.into()]
    }

    fn update(&mut self, state: &NodeProps<NodePayload>) {
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

fn custom_shape(rect: Rect, kind: &OpKind) -> Shape {
    match kind {
        OpKind::Mem => {
            // Green Rectangle
            Shape::rect_filled(rect, CornerRadiusF32::default(), Color32::GREEN)
        }
        OpKind::Arith => {
            // Orange ellipse
            let top_left = rect.min;
            let bottom_right = rect.max;
            let radius = Vec2::new(bottom_right.x - top_left.x, bottom_right.y - top_left.y);

            Shape::ellipse_filled(rect.center(), radius, Color32::ORANGE)
        }
        OpKind::ArithMsg => {
            // Yellow ellipse
            let top_left = rect.min;
            let bottom_right = rect.max;
            let radius = Vec2::new(bottom_right.x - top_left.x, bottom_right.y - top_left.y);
            Shape::ellipse_filled(rect.center(), radius, Color32::YELLOW)
        }
        OpKind::Pbs => {
            // Circle
            let top_left = rect.min;
            let bottom_right = rect.max;
            let radius = (bottom_right.x - top_left.x).max(bottom_right.y - top_left.y);
            // Green Rectangle
            Shape::circle_filled(rect.center(), radius, Color32::RED)
        }
        OpKind::Ucore => {
            // White Rectangle
            Shape::rect_filled(rect, CornerRadiusF32::default(), Color32::WHITE)
        }
    }
}

#[derive(Clone)]
pub struct EdgeShapeFlex {
    label: String,
    default_impl: DefaultEdgeShape,
}

impl<E: Clone + std::fmt::Display> From<EdgeProps<E>> for EdgeShapeFlex {
    fn from(props: EdgeProps<E>) -> Self {
        Self {
            label: format!("{}", props.payload),
            default_impl: DefaultEdgeShape::from(props),
        }
    }
}

impl<
    N: Clone,
    E: Clone + std::fmt::Display,
    Ty: EdgeType,
    Ix: IndexType,
    D: DisplayNode<N, E, Ty, Ix>,
> DisplayEdge<N, E, Ty, Ix, D> for EdgeShapeFlex
{
    fn shapes(
        &mut self,
        start: &Node<N, E, Ty, Ix, D>,
        end: &Node<N, E, Ty, Ix, D>,
        ctx: &DrawContext,
    ) -> Vec<Shape> {
        let mut shape = self.default_impl.shapes(start, end, ctx);
        let edge_center = start.location() + ((end.location() - start.location()) / 2.0);

        // create label
        let center = ctx.meta.canvas_to_screen_pos(edge_center);
        let color = ctx.ctx.style().visuals.text_color();
        let galley = ctx.ctx.fonts(|f| {
            f.layout_no_wrap(
                self.label.clone(),
                FontId::new(ctx.meta.canvas_to_screen_size(10.), FontFamily::Monospace),
                color,
            )
        });
        let offset = Vec2::new(-galley.size().x / 2., -galley.size().y / 2.);

        // create the shape and add it to the layers
        let shape_label = TextShape::new(center + offset, galley, color);
        shape.push(shape_label.into());
        shape
    }

    fn update(&mut self, _: &egui_graphs::EdgeProps<E>) {}

    fn is_inside(
        &self,
        start: &Node<N, E, Ty, Ix, D>,
        end: &Node<N, E, Ty, Ix, D>,
        pos: Pos2,
    ) -> bool {
        self.default_impl.is_inside(start, end, pos)
    }
}
