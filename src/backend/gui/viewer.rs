use eframe::{
    App, NativeOptions,
    egui::{
        Context, *,
    },
    run_native,
};
use egui_graphs::{
    DisplayEdge, DisplayNode, Graph, GraphView, SettingsInteraction, SettingsNavigation, SettingsStyle,
};
use petgraph::{
    Directed, EdgeType,
    stable_graph::{DefaultIx, IndexType, StableGraph},
    visit::EdgeRef,
};

use super::node::NodeShape;
use super::{GraphFmt, GraphShow};
use crate::ir::IrDag;

pub struct DagViewer<N, E>
where
    N: GraphShow + GraphFmt,
    E: GraphFmt,
{
    // Inner graph
    graph: Graph<N, E, Directed, DefaultIx, NodeShape<N>>,

    // Custom runtime options
    fit_to_screen: bool,
}

impl<N, E> From<&IrDag<N, E>> for DagViewer<N, E>
where
    N: GraphShow + GraphFmt,
    E: GraphFmt,
{
    fn from(ir_dag: &IrDag<N, E>) -> Self {
        let petgraph_dag = ir_dag.get_graph();

        // Create a StableGraph compatible with egui_graphs
        let mut stable_graph: StableGraph<N, E> = StableGraph::new();
        let mut node_map = std::collections::HashMap::new();

        // Add nodes
        for node_idx in petgraph_dag.node_indices() {
            let op = petgraph_dag[node_idx].clone();
            let new_idx = stable_graph.add_node(op);
            node_map.insert(node_idx, new_idx);
        }

        // Add edges
        for edge_ref in petgraph_dag.edge_references() {
            let src = edge_ref.source();
            let dst = edge_ref.target();
            stable_graph.add_edge(node_map[&src], node_map[&dst], edge_ref.weight().clone());
        }

        // Convert to egui_graphs Graph
        // Use custom transform for edge label extraction
        // Node use custom shape and could handle label extraction directly
        let graph = egui_graphs::to_graph_custom(
            &stable_graph,
            &mut egui_graphs::default_node_transform,
            &mut graph_fmt_edge_transform,
        );

        Self {
            graph,
            fit_to_screen: false,
        }
    }
}

/// Custom edge transform
/// Work on any type that implement GraphFmt and use short_fmt as label
pub fn graph_fmt_edge_transform<
    N: Clone,
    E: GraphFmt,
    Ty: EdgeType,
    Ix: IndexType,
    Dn: DisplayNode<N, E, Ty, Ix>,
    D: DisplayEdge<N, E, Ty, Ix, Dn>,
>(
    edge: &mut egui_graphs::Edge<N, E, Ty, Ix, Dn, D>,
) {
    edge.set_label(edge.payload().fmt_short());
}

impl<N, E> App for DagViewer<N, E>
where
    N: GraphFmt + GraphShow,
    E: GraphFmt,
{
    fn update(&mut self, ctx: &Context, _: &mut eframe::Frame) {
        TopBottomPanel::top("Button").show(ctx, |ui| {
            ui.horizontal(|ui| {
                // Add button to manually required a fit to screen
                if ui.button("🔍 Fit to Screen").clicked() {
                    self.fit_to_screen = true;
                }
            });
        });

        TopBottomPanel::bottom("Properties").show(ctx, |ui| {
            let props = if !self.graph.selected_nodes().is_empty() {
                let idx = self.graph.selected_nodes().first().unwrap();
                self.graph.node(*idx).unwrap().payload().fmt_long()
            } else if !self.graph.selected_edges().is_empty() {
                let idx = self.graph.selected_edges().first().unwrap();
                self.graph.edge(*idx).unwrap().payload().fmt_long()
            } else {
                "".to_string()
            };
            ui.label(props);
        });

        CentralPanel::default().show(ctx, |ui| {
            // Configure settings
            let interaction_settings = SettingsInteraction::default()
                .with_edge_clicking_enabled(true)
                .with_edge_selection_enabled(true)
                .with_node_clicking_enabled(true)
                .with_node_selection_enabled(true);

            // Check if we should fit now
            let navigation_settings = if self.fit_to_screen {
                self.fit_to_screen = false; // Reset flag
                SettingsNavigation::default()
                    .with_fit_to_screen_enabled(false)
                    .with_zoom_and_pan_enabled(true)
                    .with_fit_to_screen_enabled(true)
            } else {
                SettingsNavigation::default()
                    .with_fit_to_screen_enabled(false)
                    .with_zoom_and_pan_enabled(true)
            };

            // Only show labels for selected edges
            let style_settings = SettingsStyle::default().with_labels_always(false);

            // Use custom node display for different shapes
            type L = egui_graphs::LayoutHierarchical;
            type S = egui_graphs::LayoutStateHierarchical;

            let mut graph_view = GraphView::<N, E, _, _, _, _, S, L>::new(&mut self.graph)
                .with_interactions(&interaction_settings)
                .with_navigations(&navigation_settings)
                .with_styles(&style_settings);

            ui.add(&mut graph_view);
        });
    }
}

pub fn dag_display<N, E>(ir_dag: &IrDag<N, E>, app_name: &str)
where
    N: GraphFmt + GraphShow,
    E: GraphFmt,
{
    run_native(
        app_name,
        NativeOptions::default(),
        Box::new(|_cc| Ok(Box::new(DagViewer::from(ir_dag)))),
    )
    .unwrap();
}
