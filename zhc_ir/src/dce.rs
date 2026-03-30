//! Dead code elimination pass.
//!
//! An operation is considered *live* if it is an effect (zero return values) or
//! if any operation transitively consuming its return values is an effect. All
//! other operations are *dead* and can be removed without changing observable
//! behavior. [`DeadCodeAnalysis`] computes per-operation liveness, and
//! [`eliminate_dead_code`] applies it destructively to an [`IR`].

use zhc_utils::iter::CollectInSmallVec;

use crate::OpMap;

use super::{Dialect, IR, OpId};
use std::ops::Index;

/// Represents the liveness of an operation in dead code analysis.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Liveness {
    /// The operation is live and should be kept.
    Live,
    /// The operation is dead and can be safely removed.
    Dead,
}

/// Analysis result containing dead code elimination information.
///
/// This analysis determines which operations are "live" (have observable effects
/// or contribute to operations with observable effects) versus "dead" (can be
/// safely removed without changing program behavior).
pub struct DeadCodeAnalysis {
    states: OpMap<Liveness>,
}

impl DeadCodeAnalysis {
    /// Performs dead code analysis on the given IR.
    pub fn from_ir<D: Dialect>(ir: &IR<D>) -> Self {
        let mut states = ir.filled_opmap(Liveness::Dead);

        let mut worklist = Vec::new();
        for effect in ir.raw_walk_ops_linear().filter(|op| op.is_effect()) {
            states.insert(effect.get_id(), Liveness::Live);
            worklist.push(effect);
        }
        while let Some(op) = worklist.pop() {
            for pred in op.get_predecessors_iter() {
                if states[pred.get_id()] == Liveness::Dead {
                    states.insert(pred.get_id(), Liveness::Live);
                    worklist.push(pred);
                }
            }
        }
        DeadCodeAnalysis { states }
    }

    /// Returns an iterator over all active operations and their liveness status.
    pub fn get_statuses_iter(&self) -> impl DoubleEndedIterator<Item = (OpId, &Liveness)> {
        self.states.iter().cosvec().into_iter()
    }

    /// Returns `true` if the IR contains any dead operations.
    pub fn has_dead_code(&self) -> bool {
        self.states.iter().any(|(_, a)| matches!(a, Liveness::Dead))
    }
}

impl Index<OpId> for DeadCodeAnalysis {
    type Output = Liveness;

    fn index(&self, index: OpId) -> &Self::Output {
        &self.states[index]
    }
}

/// Eliminates dead code from the IR.
///
/// Performs dead code analysis to identify operations that don't contribute
/// to any observable effects, then removes those operations from the IR.
/// Operations are deleted in dependency-safe order.
pub fn eliminate_dead_code<D: Dialect>(ir: &mut IR<D>) {
    let analysis = DeadCodeAnalysis::from_ir(ir);
    let deletions = analysis
        .get_statuses_iter()
        .filter_map(|(opid, stat)| match stat {
            Liveness::Live => None,
            Liveness::Dead => Some(opid),
        });
    ir.batch_delete_op(deletions);
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{testlang::*, *};
    use zhc_utils::{assert_display_is, svec};

    #[test]
    fn test_empty_ir() {
        let mut ir = IR::<TestLang>::empty();
        let analysis = DeadCodeAnalysis::from_ir(&ir);
        assert_eq!(analysis.get_statuses_iter().count(), 0);
        eliminate_dead_code(&mut ir);
        assert_eq!(ir.n_ops(), 0);
        assert_display_is!(ir.format(), r#""#);
    }

    #[test]
    fn test_single_effect_operation() {
        let mut ir = IR::<TestLang>::empty();
        let (input_op, input_vals) = ir.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![]);
        let (return_op, _) = ir.add_op(TestInstructionSet::Return, input_vals);
        assert_display_is!(
            ir.format(),
            r#"
                %0 = int_input<pos: 0>();
                return(%0);
            "#
        );

        let analysis = DeadCodeAnalysis::from_ir(&ir);
        assert_eq!(analysis[input_op], Liveness::Live);
        assert_eq!(analysis[return_op], Liveness::Live);

        eliminate_dead_code(&mut ir);
        assert_eq!(ir.n_ops(), 2);
        assert!(ir.has_opid(input_op));
        assert!(ir.has_opid(return_op));
        assert_display_is!(
            ir.format(),
            r#"
                %0 = int_input<pos: 0>();
                return(%0);
            "#
        );
    }

    #[test]
    fn test_dead_operation_no_users() {
        let mut ir = IR::<TestLang>::empty();
        let (input_op, input_vals) = ir.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![]);
        let (add_op, _add_vals) =
            ir.add_op(TestInstructionSet::Add, svec![input_vals[0], input_vals[0]]);
        let (return_op, _) = ir.add_op(TestInstructionSet::Return, svec![input_vals[0]]);

        assert_display_is!(
            ir.format(),
            r#"
                %0 = int_input<pos: 0>();
                %1 = add(%0, %0);
                return(%0);
            "#
        );

        let analysis = DeadCodeAnalysis::from_ir(&ir);
        assert_eq!(analysis[input_op], Liveness::Live);
        assert_eq!(analysis[add_op], Liveness::Dead);
        assert_eq!(analysis[return_op], Liveness::Live);

        eliminate_dead_code(&mut ir);
        assert_eq!(ir.n_ops(), 2);
        assert!(ir.has_opid(input_op));
        assert!(!ir.has_opid(add_op));
        assert!(ir.has_opid(return_op));

        assert_display_is!(
            ir.format(),
            r#"
                %0 = int_input<pos: 0>();
                return(%0);
            "#
        );
    }

    #[test]
    fn test_dependency_chain_all_live() {
        let mut ir = IR::<TestLang>::empty();
        let (input1, vals1) = ir.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![]);
        let (input2, vals2) = ir.add_op(TestInstructionSet::IntInput { pos: 1 }, svec![]);
        let (add_op, add_vals) = ir.add_op(TestInstructionSet::Add, svec![vals1[0], vals2[0]]);
        let (inc_op, inc_vals) = ir.add_op(TestInstructionSet::Inc, add_vals);
        let (return_op, _) = ir.add_op(TestInstructionSet::Return, inc_vals);

        assert_display_is!(
            ir.format(),
            r#"
                %0 = int_input<pos: 0>();
                %1 = int_input<pos: 1>();
                %2 = add(%0, %1);
                %3 = inc(%2);
                return(%3);
            "#
        );

        let analysis = DeadCodeAnalysis::from_ir(&ir);
        assert_eq!(analysis[input1], Liveness::Live);
        assert_eq!(analysis[input2], Liveness::Live);
        assert_eq!(analysis[add_op], Liveness::Live);
        assert_eq!(analysis[inc_op], Liveness::Live);
        assert_eq!(analysis[return_op], Liveness::Live);

        eliminate_dead_code(&mut ir);
        assert_eq!(ir.n_ops(), 5);

        assert_display_is!(
            ir.format(),
            r#"
                %0 = int_input<pos: 0>();
                %1 = int_input<pos: 1>();
                %2 = add(%0, %1);
                %3 = inc(%2);
                return(%3);
            "#
        );
    }

    #[test]
    fn test_diamond_pattern_partial_dead() {
        let mut ir = IR::<TestLang>::empty();
        let (input_op, input_vals) = ir.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![]);
        let (inc1_op, inc1_vals) = ir.add_op(TestInstructionSet::Inc, input_vals.clone());
        let (inc2_op, inc2_vals) = ir.add_op(TestInstructionSet::Inc, input_vals);
        let (add_op, _) = ir.add_op(TestInstructionSet::Add, svec![inc1_vals[0], inc2_vals[0]]);
        let (return_op, _) = ir.add_op(TestInstructionSet::Return, svec![inc1_vals[0]]);

        assert_display_is!(
            ir.format(),
            r#"
                %0 = int_input<pos: 0>();
                %1 = inc(%0);
                %2 = inc(%0);
                %3 = add(%1, %2);
                return(%1);
            "#
        );

        let analysis = DeadCodeAnalysis::from_ir(&ir);
        assert_eq!(analysis[input_op], Liveness::Live);
        assert_eq!(analysis[inc1_op], Liveness::Live);
        assert_eq!(analysis[inc2_op], Liveness::Dead);
        assert_eq!(analysis[add_op], Liveness::Dead);
        assert_eq!(analysis[return_op], Liveness::Live);

        eliminate_dead_code(&mut ir);
        assert_eq!(ir.n_ops(), 3);
        assert!(ir.has_opid(input_op));
        assert!(ir.has_opid(inc1_op));
        assert!(!ir.has_opid(inc2_op));
        assert!(!ir.has_opid(add_op));
        assert!(ir.has_opid(return_op));

        assert_display_is!(
            ir.format(),
            r#"
                %0 = int_input<pos: 0>();
                %1 = inc(%0);
                return(%1);
            "#
        );
    }

    #[test]
    fn test_multiple_effects() {
        let mut ir = IR::<TestLang>::empty();
        let (input1, vals1) = ir.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![]);
        let (input2, vals2) = ir.add_op(TestInstructionSet::IntInput { pos: 1 }, svec![]);
        let (add_op, _) = ir.add_op(TestInstructionSet::Add, svec![vals1[0], vals2[0]]);
        let (return1, _) = ir.add_op(TestInstructionSet::Return, svec![vals1[0]]);
        let (return2, _) = ir.add_op(TestInstructionSet::Return, svec![vals2[0]]);

        assert_display_is!(
            ir.format(),
            r#"
                %0 = int_input<pos: 0>();
                %1 = int_input<pos: 1>();
                %2 = add(%0, %1);
                return(%0);
                return(%1);
            "#
        );

        let analysis = DeadCodeAnalysis::from_ir(&ir);
        assert_eq!(analysis[input1], Liveness::Live);
        assert_eq!(analysis[input2], Liveness::Live);
        assert_eq!(analysis[add_op], Liveness::Dead);
        assert_eq!(analysis[return1], Liveness::Live);
        assert_eq!(analysis[return2], Liveness::Live);

        eliminate_dead_code(&mut ir);
        assert_eq!(ir.n_ops(), 4);
        assert!(!ir.has_opid(add_op));
        assert_display_is!(
            ir.format(),
            r#"
                %0 = int_input<pos: 0>();
                %1 = int_input<pos: 1>();
                return(%0);
                return(%1);
            "#
        );
    }

    #[test]
    fn test_all_operations_dead() {
        let mut ir = IR::<TestLang>::empty();
        let (input_op, input_vals) = ir.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![]);
        let (add_op, add_vals) =
            ir.add_op(TestInstructionSet::Add, svec![input_vals[0], input_vals[0]]);
        let (inc_op, _) = ir.add_op(TestInstructionSet::Inc, add_vals);

        assert_display_is!(
            ir.format(),
            r#"
                %0 = int_input<pos: 0>();
                %1 = add(%0, %0);
                %2 = inc(%1);
            "#
        );

        let analysis = DeadCodeAnalysis::from_ir(&ir);
        assert_eq!(analysis[input_op], Liveness::Dead);
        assert_eq!(analysis[add_op], Liveness::Dead);
        assert_eq!(analysis[inc_op], Liveness::Dead);

        eliminate_dead_code(&mut ir);
        assert_eq!(ir.n_ops(), 0);

        assert_display_is!(
            ir.format(),
            r#"
            "#
        );
    }

    #[test]
    fn test_with_already_deleted_operations() {
        let mut ir = IR::<TestLang>::empty();
        let (input_op, input_vals) = ir.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![]);
        let (add_op, _) = ir.add_op(TestInstructionSet::Add, svec![input_vals[0], input_vals[0]]);
        let (return_op, _) = ir.add_op(TestInstructionSet::Return, input_vals);

        ir.delete_op(add_op);

        assert_display_is!(
            ir.format(),
            r#"
                %0 = int_input<pos: 0>();
                return(%0);
            "#
        );

        let analysis = DeadCodeAnalysis::from_ir(&ir);
        assert_eq!(analysis[input_op], Liveness::Live);
        assert_eq!(analysis[return_op], Liveness::Live);

        eliminate_dead_code(&mut ir);
        assert_eq!(ir.n_ops(), 2);
        assert!(ir.has_opid(input_op));
        assert!(ir.has_opid(return_op));

        assert_display_is!(
            ir.format(),
            r#"
                %0 = int_input<pos: 0>();
                return(%0);
            "#
        );
    }

    #[test]
    fn test_complex_mixed_scenario() {
        let mut ir = IR::<TestLang>::empty();
        let (input1, vals1) = ir.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![]);
        let (input2, vals2) = ir.add_op(TestInstructionSet::IntInput { pos: 1 }, svec![]);
        let (dead_add1, dead_vals1) = ir.add_op(TestInstructionSet::Add, svec![vals1[0], vals2[0]]);
        let (dead_add2, _) =
            ir.add_op(TestInstructionSet::Add, svec![dead_vals1[0], dead_vals1[0]]);
        let (live_inc, live_vals) = ir.add_op(TestInstructionSet::Inc, svec![vals1[0]]);
        let (return_op, _) = ir.add_op(TestInstructionSet::Return, live_vals);

        assert_display_is!(
            ir.format(),
            r#"
                %0 = int_input<pos: 0>();
                %1 = int_input<pos: 1>();
                %2 = add(%0, %1);
                %3 = add(%2, %2);
                %4 = inc(%0);
                return(%4);
            "#
        );

        let analysis = DeadCodeAnalysis::from_ir(&ir);
        assert_eq!(analysis[input1], Liveness::Live);
        assert_eq!(analysis[input2], Liveness::Dead);
        assert_eq!(analysis[dead_add1], Liveness::Dead);
        assert_eq!(analysis[dead_add2], Liveness::Dead);
        assert_eq!(analysis[live_inc], Liveness::Live);
        assert_eq!(analysis[return_op], Liveness::Live);

        eliminate_dead_code(&mut ir);
        assert_eq!(ir.n_ops(), 3);
        assert!(ir.has_opid(input1));
        assert!(!ir.has_opid(input2));
        assert!(!ir.has_opid(dead_add1));
        assert!(!ir.has_opid(dead_add2));
        assert!(ir.has_opid(live_inc));
        assert!(ir.has_opid(return_op));
        assert_display_is!(
            ir.format(),
            r#"
                %0 = int_input<pos: 0>();
                %4 = inc(%0);
                return(%4);
            "#
        );
    }
}
