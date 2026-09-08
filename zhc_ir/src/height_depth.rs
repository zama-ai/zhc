//! Dialect-independent path scoring over operation dependencies.

use serde::Serialize;
use zhc_utils::svec;

use crate::{Analysing, AnnIR, AnnOpRef, AsOpRef, Dialect, IR, OpIdRaw, OpRef};

pub type Cost = OpIdRaw;

/// Scores toward sinks (height) and from sources (depth).
#[derive(PartialEq, Eq, Debug, Clone, Serialize)]
pub struct HeightDepth {
    /// Score accumulated backward through users, including this operation.
    pub height: Cost,
    /// Score accumulated forward through predecessors, including this operation.
    pub depth: Cost,
}

/// Computes height backward and depth forward using the same scoring policy.
///
/// `advance` receives an operation and the maximum neighboring score (zero when
/// there are no neighbors). It must incorporate the current operation's cost and
/// any rounding. For additive costs, both fields are longest weighted path
/// lengths including the current operation. The policy is called once per
/// operation in each pass and should return the same result for the same inputs.
/// Values receive unit annotations.
pub fn analyze_height_depth<D: Dialect>(
    ir: &IR<D>,
    advance: impl Fn(OpRef<'_, D>, Cost) -> Cost,
) -> AnnIR<'_, D, HeightDepth, ()> {
    let heighted = ir.backward_dataflow_analysis::<Cost, ()>(|op| {
        let score = op
            .get_users_iter()
            .map(|user| user.get_annotation().clone().unwrap_analyzed())
            .max()
            .unwrap_or(0);
        let height = advance(op.op_ref(), score);
        (height, svec![(); op.get_return_arity()])
    });
    heighted.forward_dataflow_analysis(
        |op: AnnOpRef<'_, '_, _, Analysing<HeightDepth>, _>, previous| {
            let score = op
                .get_predecessors_iter()
                .map(|pred| pred.get_annotation().clone().unwrap_analyzed().depth)
                .max()
                .unwrap_or(0);
            let stats = HeightDepth {
                height: *previous.get_annotation(),
                depth: advance(op.op_ref(), score),
            };
            (stats, svec![(); op.get_return_arity()])
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testlang::{TestInstructionSet::*, TestLang};

    #[test]
    fn weighted_branches_and_rounding() {
        let mut ir = IR::<TestLang>::empty();
        let (source, a) = ir.add_op(IntInput { pos: 0 }, svec![]);
        let (branch, b) = ir.add_op(Inc, svec![a[0]]);
        let (join, c) = ir.add_op(Add, svec![a[0], b[0]]);
        let (sink, _) = ir.add_op(Return, svec![c[0]]);
        let (isolated, _) = ir.add_op(IntInput { pos: 1 }, svec![]);

        for rounded in [false, true] {
            let result = analyze_height_depth(&ir, |op, score| {
                if matches!(op.get_instruction(), Add) {
                    let score = score.strict_add(10);
                    if rounded { score / 10 * 10 } else { score }
                } else {
                    score.strict_add(1)
                }
            });
            let expected = if rounded {
                [(12, 1), (11, 2), (10, 10), (1, 11), (1, 1)]
            } else {
                [(13, 1), (12, 2), (11, 12), (1, 13), (1, 1)]
            };
            for (id, (height, depth)) in [source, branch, join, sink, isolated]
                .into_iter()
                .zip(expected)
            {
                assert_eq!(
                    result.get_op(id).get_annotation(),
                    &HeightDepth { height, depth }
                );
            }
            assert!(
                result
                    .walk_vals_linear()
                    .all(|val| *val.get_annotation() == ())
            );
        }
    }

    #[test]
    fn empty_ir() {
        let ir = IR::<TestLang>::empty();
        let result = analyze_height_depth(&ir, |_, _| panic!("empty IR"));
        assert_eq!(result.walk_ops_linear().count(), 0);
        assert_eq!(result.walk_vals_linear().count(), 0);
    }
}
