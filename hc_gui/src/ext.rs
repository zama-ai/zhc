use std::sync::Arc;

use hc_ir::{Dialect, IR, OpId, ValId};
use petgraph::prelude::StableGraph;

pub trait IRExt {
    fn to_contextual_graph(self: Arc<Self>) -> StableGraph<(OpId, Arc<Self>), (ValId, Arc<Self>)>;
}

impl<D: Dialect> IRExt for IR<D> {
    fn to_contextual_graph(self: Arc<Self>) -> StableGraph<(OpId, Arc<Self>), (ValId, Arc<Self>)> {
        let mut output = StableGraph::new();
        let map = self.totally_mapped_opmap(|op| output.add_node((op.get_id(), self.clone())));
        for value in self.walk_vals_linear() {
            for user in value.get_users_iter() {
                output.add_edge(
                    map[value.get_origin().get_id()],
                    map[user.get_id()],
                    (value.get_id(), self.clone()),
                );
            }
        }
        output
    }
}
