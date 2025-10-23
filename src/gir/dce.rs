use std::ops::Index;
use crate::utils::Store;
use super::{Dialect, IR, OpId};

/// Represents the liveness of an operation in dead code analysis.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Liveness {
    /// The operation is live and should be kept.
    Live,
    /// The operation is dead and can be safely removed.
    Dead,
}

/// A structure containing the result of the dead code analysis.
///
/// This analysis determines which operations in an IR are "live" (have observable effects
/// or contribute to operations with observable effects) versus "dead" (can be safely removed).
pub struct DeadCodeAnalysis {
    states: Store<OpId, Option<Liveness>>,
}

impl DeadCodeAnalysis {
    fn raw_get_statuses_iter(&self) -> impl Iterator<Item = (OpId, Option<&Liveness>)> {
        self.states.enumerate_iter().map(|(i, a)| (i, a.as_ref()))
    }
}

impl DeadCodeAnalysis {

    /// Performs dead code analysis on the given IR.
    pub fn from_ir<D: Dialect>(ir: &IR<D>) -> Self {
        let mut states: Store<OpId, _> = ir
            .raw_ops_iter()
            .map(|op| {
                if op.is_active() {
                    Some(Liveness::Dead)
                } else {
                    None
                }
            })
            .collect();
        let mut worklist = Vec::new();
        for effect in ir.raw_ops_iter().filter(|op| op.is_effect()) {
            states[effect.get_id()] = Some(Liveness::Live);
            worklist.push(effect);
        }
        while let Some(op) = worklist.pop() {
            for pred in op.get_predecessors_iter() {
                if states[pred.get_id()] == Some(Liveness::Dead) {
                    states[pred.get_id()] = Some(Liveness::Live);
                    worklist.push(pred);
                } else if states[pred.get_id()].is_none() {
                    panic!("Fatal error");
                }
            }
        }
        DeadCodeAnalysis { states }
    }

    /// Returns an iterator over all active operations and their liveness status.
    pub fn get_statuses_iter(&self) -> impl Iterator<Item = (OpId, &Liveness)> {
        self.raw_get_statuses_iter().filter_map(|(i, s)| s.map(|v| (i, v)))
    }
}

impl Index<OpId> for DeadCodeAnalysis {
    type Output = Option<Liveness>;

    fn index(&self, index: OpId) -> &Self::Output {
        &self.states[index]
    }
}

/// Eliminates dead code from the given IR.
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
    use crate::{
        gir::{tests::test_dialect::*, *},
        svec,
    };

    #[test]
    fn test_empty_ir() {
        let mut ir = IR::<TestDialect>::empty();
        let analysis = DeadCodeAnalysis::from_ir(&ir);
        assert_eq!(analysis.get_statuses_iter().count(), 0);
        eliminate_dead_code(&mut ir);
        assert_eq!(ir.n_ops(), 0);
        ir.check_ir("");
    }

    #[test]
    fn test_single_effect_operation() {
        let mut ir = IR::<TestDialect>::empty();
        let (input_op, input_vals) = ir.add_op(Operations::IntInput { pos: 0 }, svec![]);
        let (return_op, _) = ir.add_op(Operations::Return, input_vals);
        ir.check_ir(
            "
            %0 : Int = int_input<pos: 0>();
            return(%0);
            ",
        );

        let analysis = DeadCodeAnalysis::from_ir(&ir);
        assert_eq!(analysis[input_op], Some(Liveness::Live));
        assert_eq!(analysis[return_op], Some(Liveness::Live));

        eliminate_dead_code(&mut ir);
        assert_eq!(ir.n_ops(), 2);
        assert!(ir.has_opid(input_op));
        assert!(ir.has_opid(return_op));
        ir.check_ir(
            "
            %0 : Int = int_input<pos: 0>();
            return(%0);
            ",
        );
    }

    #[test]
    fn test_dead_operation_no_users() {
        let mut ir = IR::<TestDialect>::empty();
        let (input_op, input_vals) = ir.add_op(Operations::IntInput { pos: 0 }, svec![]);
        let (add_op, _add_vals) = ir.add_op(Operations::Add, svec![input_vals[0], input_vals[0]]);
        let (return_op, _) = ir.add_op(Operations::Return, svec![input_vals[0]]);

        ir.check_ir(
            "
            %0 : Int = int_input<pos: 0>();
            %1 : Int = add(%0, %0);
            return(%0);
            ",
        );

        let analysis = DeadCodeAnalysis::from_ir(&ir);
        assert_eq!(analysis[input_op], Some(Liveness::Live));
        assert_eq!(analysis[add_op], Some(Liveness::Dead));
        assert_eq!(analysis[return_op], Some(Liveness::Live));

        eliminate_dead_code(&mut ir);
        assert_eq!(ir.n_ops(), 2);
        assert!(ir.has_opid(input_op));
        assert!(!ir.has_opid(add_op));
        assert!(ir.has_opid(return_op));

        ir.check_ir(
            "
            %0 : Int = int_input<pos: 0>();
            // %_1 : Int = add(%0, %0);
            return(%0);
            ",
        );
    }

    #[test]
    fn test_dependency_chain_all_live() {
        let mut ir = IR::<TestDialect>::empty();
        let (input1, vals1) = ir.add_op(Operations::IntInput { pos: 0 }, svec![]);
        let (input2, vals2) = ir.add_op(Operations::IntInput { pos: 1 }, svec![]);
        let (add_op, add_vals) = ir.add_op(Operations::Add, svec![vals1[0], vals2[0]]);
        let (inc_op, inc_vals) = ir.add_op(Operations::Inc, add_vals);
        let (return_op, _) = ir.add_op(Operations::Return, inc_vals);

        ir.check_ir(
            "
            %0 : Int = int_input<pos: 0>();
            %1 : Int = int_input<pos: 1>();
            %2 : Int = add(%0, %1);
            %3 : Int = inc(%2);
            return(%3);
            ",
        );

        let analysis = DeadCodeAnalysis::from_ir(&ir);
        assert_eq!(analysis[input1], Some(Liveness::Live));
        assert_eq!(analysis[input2], Some(Liveness::Live));
        assert_eq!(analysis[add_op], Some(Liveness::Live));
        assert_eq!(analysis[inc_op], Some(Liveness::Live));
        assert_eq!(analysis[return_op], Some(Liveness::Live));

        eliminate_dead_code(&mut ir);
        assert_eq!(ir.n_ops(), 5);

        ir.check_ir(
            "
            %0 : Int = int_input<pos: 0>();
            %1 : Int = int_input<pos: 1>();
            %2 : Int = add(%0, %1);
            %3 : Int = inc(%2);
            return(%3);
            ",
        );
    }

    #[test]
    fn test_diamond_pattern_partial_dead() {
        let mut ir = IR::<TestDialect>::empty();
        let (input_op, input_vals) = ir.add_op(Operations::IntInput { pos: 0 }, svec![]);
        let (inc1_op, inc1_vals) = ir.add_op(Operations::Inc, input_vals.clone());
        let (inc2_op, inc2_vals) = ir.add_op(Operations::Inc, input_vals);
        let (add_op, _) = ir.add_op(Operations::Add, svec![inc1_vals[0], inc2_vals[0]]);
        let (return_op, _) = ir.add_op(Operations::Return, svec![inc1_vals[0]]);

        ir.check_ir(
            "
            %0 : Int = int_input<pos: 0>();
            %1 : Int = inc(%0);
            %2 : Int = inc(%0);
            %3 : Int = add(%1, %2);
            return(%1);
            ",
        );

        let analysis = DeadCodeAnalysis::from_ir(&ir);
        assert_eq!(analysis[input_op], Some(Liveness::Live));
        assert_eq!(analysis[inc1_op], Some(Liveness::Live));
        assert_eq!(analysis[inc2_op], Some(Liveness::Dead));
        assert_eq!(analysis[add_op], Some(Liveness::Dead));
        assert_eq!(analysis[return_op], Some(Liveness::Live));

        eliminate_dead_code(&mut ir);
        assert_eq!(ir.n_ops(), 3);
        assert!(ir.has_opid(input_op));
        assert!(ir.has_opid(inc1_op));
        assert!(!ir.has_opid(inc2_op));
        assert!(!ir.has_opid(add_op));
        assert!(ir.has_opid(return_op));

        ir.check_ir(
            "
            %0 : Int = int_input<pos: 0>();
            %1 : Int = inc(%0);
            // %_2 : Int = inc(%0);
            // %_3 : Int = add(%1, %_2);
            return(%1);
        ",
        );
    }

    #[test]
    fn test_multiple_effects() {
        let mut ir = IR::<TestDialect>::empty();
        let (input1, vals1) = ir.add_op(Operations::IntInput { pos: 0 }, svec![]);
        let (input2, vals2) = ir.add_op(Operations::IntInput { pos: 1 }, svec![]);
        let (add_op, _) = ir.add_op(Operations::Add, svec![vals1[0], vals2[0]]);
        let (return1, _) = ir.add_op(Operations::Return, svec![vals1[0]]);
        let (return2, _) = ir.add_op(Operations::Return, svec![vals2[0]]);

        ir.check_ir(
            "
            %0 : Int = int_input<pos: 0>();
            %1 : Int = int_input<pos: 1>();
            %2 : Int = add(%0, %1);
            return(%0);
            return(%1);
            ",
        );

        let analysis = DeadCodeAnalysis::from_ir(&ir);
        assert_eq!(analysis[input1], Some(Liveness::Live));
        assert_eq!(analysis[input2], Some(Liveness::Live));
        assert_eq!(analysis[add_op], Some(Liveness::Dead));
        assert_eq!(analysis[return1], Some(Liveness::Live));
        assert_eq!(analysis[return2], Some(Liveness::Live));

        eliminate_dead_code(&mut ir);
        assert_eq!(ir.n_ops(), 4);
        assert!(!ir.has_opid(add_op));
        ir.check_ir(
            "
            %0 : Int = int_input<pos: 0>();
            %1 : Int = int_input<pos: 1>();
            // %_2 : Int = add(%0, %1);
            return(%0);
            return(%1);
            ",
        );
    }

    #[test]
    fn test_all_operations_dead() {
        let mut ir = IR::<TestDialect>::empty();
        let (input_op, input_vals) = ir.add_op(Operations::IntInput { pos: 0 }, svec![]);
        let (add_op, add_vals) = ir.add_op(Operations::Add, svec![input_vals[0], input_vals[0]]);
        let (inc_op, _) = ir.add_op(Operations::Inc, add_vals);

        ir.check_ir(
            "
            %0 : Int = int_input<pos: 0>();
            %1 : Int = add(%0, %0);
            %2 : Int = inc(%1);
            ",
        );

        let analysis = DeadCodeAnalysis::from_ir(&ir);
        assert_eq!(analysis[input_op], Some(Liveness::Dead));
        assert_eq!(analysis[add_op], Some(Liveness::Dead));
        assert_eq!(analysis[inc_op], Some(Liveness::Dead));

        eliminate_dead_code(&mut ir);
        assert_eq!(ir.n_ops(), 0);

        ir.check_ir(
            "
            // %_0 : Int = int_input<pos: 0>();
            // %_1 : Int = add(%_0, %_0);
            // %_2 : Int = inc(%_1);
            ",
        );
    }

    #[test]
    fn test_with_already_deleted_operations() {
        let mut ir = IR::<TestDialect>::empty();
        let (input_op, input_vals) = ir.add_op(Operations::IntInput { pos: 0 }, svec![]);
        let (add_op, _) = ir.add_op(Operations::Add, svec![input_vals[0], input_vals[0]]);
        let (return_op, _) = ir.add_op(Operations::Return, input_vals);

        ir.delete_op(add_op);

        ir.check_ir(
            "
            %0 : Int = int_input<pos: 0>();
            // %_1 : Int = add(%0, %0);
            return(%0);
            ",
        );

        let analysis = DeadCodeAnalysis::from_ir(&ir);
        assert_eq!(analysis[input_op], Some(Liveness::Live));
        assert_eq!(analysis[add_op], None); // Already deleted
        assert_eq!(analysis[return_op], Some(Liveness::Live));

        eliminate_dead_code(&mut ir);
        assert_eq!(ir.n_ops(), 2);
        assert!(ir.has_opid(input_op));
        assert!(ir.has_opid(return_op));

        ir.check_ir(
            "
            %0 : Int = int_input<pos: 0>();
            // %_1 : Int = add(%0, %0);
            return(%0);
            ",
        );
    }

    #[test]
    fn test_complex_mixed_scenario() {
        let mut ir = IR::<TestDialect>::empty();
        let (input1, vals1) = ir.add_op(Operations::IntInput { pos: 0 }, svec![]);
        let (input2, vals2) = ir.add_op(Operations::IntInput { pos: 1 }, svec![]);
        let (dead_add1, dead_vals1) = ir.add_op(Operations::Add, svec![vals1[0], vals2[0]]);
        let (dead_add2, _) =
            ir.add_op(Operations::Add, svec![dead_vals1[0], dead_vals1[0]]);
        let (live_inc, live_vals) = ir.add_op(Operations::Inc, svec![vals1[0]]);
        let (return_op, _) = ir.add_op(Operations::Return, live_vals);

        ir.check_ir(
            "
            %0 : Int = int_input<pos: 0>();
            %1 : Int = int_input<pos: 1>();
            %2 : Int = add(%0, %1);
            %3 : Int = inc(%0);
            %4 : Int = add(%2, %2);
            return(%3);
            ",
        );

        let analysis = DeadCodeAnalysis::from_ir(&ir);
        assert_eq!(analysis[input1], Some(Liveness::Live));
        assert_eq!(analysis[input2], Some(Liveness::Dead));
        assert_eq!(analysis[dead_add1], Some(Liveness::Dead));
        assert_eq!(analysis[dead_add2], Some(Liveness::Dead));
        assert_eq!(analysis[live_inc], Some(Liveness::Live));
        assert_eq!(analysis[return_op], Some(Liveness::Live));

        eliminate_dead_code(&mut ir);
        assert_eq!(ir.n_ops(), 3);
        assert!(ir.has_opid(input1));
        assert!(!ir.has_opid(input2));
        assert!(!ir.has_opid(dead_add1));
        assert!(!ir.has_opid(dead_add2));
        assert!(ir.has_opid(live_inc));
        assert!(ir.has_opid(return_op));
        ir.check_ir(
            "
            %0 : Int = int_input<pos: 0>();
            // %_1 : Int = int_input<pos: 1>();
            // %_2 : Int = add(%0, %_1);
            %3 : Int = inc(%0);
            // %_4 : Int = add(%_2, %_2);
            return(%3);
            ",
        );
    }
}
