use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

use petgraph::dot::{Config, Dot};
use petgraph::graph::{DiGraph, NodeIndex};

use super::operations::{IrCell, IrOperation};

pub struct IrDag<N, E> {
    graph: DiGraph<N, E>,
    // Track list of register Write and memory Write for building graph edges
    cell_map: HashMap<E, NodeIndex>,
}

/// Implement custom getter
impl<N, E> IrDag<N, E> {
    pub fn get_graph(&self) -> &DiGraph<N, E> {
        &self.graph
    }
}
impl IrDag<IrOperation, IrCell> {
    pub fn from_operations(operations: &[IrOperation]) -> Self {
        let mut graph = DiGraph::new();
        let mut cell_map = std::collections::HashMap::new();

        for op in operations {
            let node = graph.add_node(op.clone());

            // Add edges from inputs
            for input in op.get_inputs() {
                if let Some(&input_node) = cell_map.get(&input) {
                    graph.add_edge(input_node, node, input);
                }
            }

            // Store output for future node lookup
            for output in op.get_outputs() {
                cell_map.insert(output, node);
            }
        }

        IrDag { graph, cell_map }
    }
}

/// Implement Dot convertion functions
impl<N, E> IrDag<N, E>
where
    N: std::fmt::Debug,
    E: std::fmt::Debug,
{
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
}
