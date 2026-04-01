use zhc_utils::svec;

use crate::{AnnIR, Dialect, IR, OpIdRaw};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Slack(pub OpIdRaw);

pub fn compute_slack<D: Dialect>(ir: &IR<D>) -> AnnIR<'_, D, Slack, ()> {
    let ir_depth = ir.walk_ops_linear().map(|op| op.depth).max().unwrap();
    ir.backward_dataflow_analysis::<OpIdRaw, ()>(|opref| {
        let previous_height = opref
            .get_users_iter()
            .map(|u| u.get_annotation().clone().unwrap_analyzed())
            .max()
            .unwrap_or(0);
        (previous_height + 1, svec![(); opref.get_return_arity()])
    })
    .map_opann(|opref| Slack(ir_depth + 1 - opref.depth - opref.get_annotation()))
}
