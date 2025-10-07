use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

use petgraph::dot::{Config, Dot};
use petgraph::graph::{DiGraph, NodeIndex};

use super::operations::{IrCell, IrOperation};

pub struct IrDag {
    graph: DiGraph<IrOperation, ()>,
    // Track list of register Write and memory Write for building graph edges
    cell_map: HashMap<IrCell, NodeIndex>,
}

impl IrDag {
    pub fn from_operations(operations: &[IrOperation]) -> Self {
        let mut graph = DiGraph::new();
        let mut cell_map = std::collections::HashMap::new();

        for op in operations {
            let node = graph.add_node(op.clone());

            // Add edges from inputs
            for input in op.get_inputs() {
                if let Some(&input_node) = cell_map.get(&input) {
                    graph.add_edge(input_node, node, ());
                }
            }

            // Store output for future node lookup
            for output in op.get_outputs() {
                cell_map.insert(output, node);
            }
        }

        IrDag { graph, cell_map }
    }

    pub fn to_dot(&self) -> String {
        format!(
            "{:?}",
            Dot::with_config(&self.graph, &[Config::EdgeNoLabel])
        )
    }

    pub fn write_dot_file(&self, filename: &str) -> std::io::Result<()> {
        let mut file = File::create(filename)?;
        write!(file, "{}", self.to_dot())?;
        Ok(())
    }

    pub fn get_graph(&self) -> &DiGraph<IrOperation, ()> {
        &self.graph
    }
}
