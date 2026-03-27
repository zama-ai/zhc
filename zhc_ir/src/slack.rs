use zhc_utils::svec;

use crate::{AnnIR, Dialect, IR};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Slack(pub u16);

pub fn compute_slack<D: Dialect>(ir: &IR<D>) -> AnnIR<'_, D, Slack, ()> {
    let ir_depth = ir.walk_ops_linear().map(|op| op.depth).max().unwrap();
    ir.backward_dataflow_analysis::<u16, ()>(|opref| {
        let previous_height = opref
            .get_users_iter()
            .map(|u| u.get_annotation().clone().unwrap_analyzed())
            .max()
            .unwrap_or(0u16);
        (previous_height + 1, svec![(); opref.get_return_arity()])
    })
    .map_opann(|opref| Slack(ir_depth + 1 - opref.depth - opref.get_annotation()))
}
