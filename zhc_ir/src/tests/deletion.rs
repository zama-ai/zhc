//! Deleting ops: `delete_op`, `batch_delete_op`, and how deleted elements show up afterwards.

use crate::IR;
use crate::tests::testlang::{TestInstructionSet, TestLang};
use zhc_utils::{assert_display_is, iter::CollectInVec, svec};

/// Tests that attempting to delete an operation still in use panics
#[test]
#[should_panic]
fn test_delete_op_in_use() {
    let mut store: IR<TestLang> = IR::empty();
    let (lhs_id, v0) = store.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![]);
    let (_, v1) = store.add_op(TestInstructionSet::IntInput { pos: 1 }, svec![]);
    let _ = store.add_op(TestInstructionSet::Add, svec![v0[0], v1[0]]);
    store.delete_op(lhs_id);
}

/// Tests successful deletion of operations and verification of inactive state
#[test]
fn test_delete_op() {
    let mut store: IR<TestLang> = IR::empty();
    let (_, v0) = store.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![]);
    let (rhs_id, v1) = store.add_op(TestInstructionSet::IntInput { pos: 1 }, svec![]);
    let (join_id, v2) = store.add_op(TestInstructionSet::Add, svec![v0[0], v1[0]]);
    store.delete_op(join_id);
    store.delete_op(rhs_id);
    assert_display_is!(
        store.format(),
        r#"
            %0 = int_input<pos: 0>();
        "#
    );
    assert!(store.raw_get_val(v2[0]).is_inactive());
    assert!(store.raw_get_val(v1[0]).is_inactive());
    assert!(store.raw_get_op(join_id).is_inactive());
    assert!(store.raw_get_op(rhs_id).is_inactive());
}

/// Tests that double deletion is captured
#[test]
#[should_panic(expected = "Tried to delete an already inactive operation")]
fn test_double_deletion() {
    let mut store: IR<TestLang> = IR::empty();
    let (op_id, _) = store.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![]);
    store.delete_op(op_id);
    store.delete_op(op_id); // Should panic on second deletion
}

/// Tests iteration behavior over IR with deleted elements
#[test]
fn test_iteration_with_deleted_elements() {
    let mut store: IR<TestLang> = IR::empty();
    let (op1_id, _) = store.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![]);
    let (op2_id, _) = store.add_op(TestInstructionSet::IntInput { pos: 1 }, svec![]);
    let (op3_id, _) = store.add_op(TestInstructionSet::IntInput { pos: 2 }, svec![]);

    store.delete_op(op2_id); // Delete middle operation

    let active_ops = store.walk_ops_linear().map(|op| op.get_id()).covec();

    // Should only see active operations
    assert_eq!(active_ops.len(), 2);
    assert!(active_ops.contains(&op1_id));
    assert!(!active_ops.contains(&op2_id));
    assert!(active_ops.contains(&op3_id));

    // Raw iterator should see all operations
    let all_ops = store.raw_walk_ops_linear().map(|op| op.get_id()).covec();
    assert_eq!(all_ops.len(), 3);
}

/// Tests that user lists remain consistent after deletions
#[test]
fn test_user_consistency_after_deletion() {
    let mut store: IR<TestLang> = IR::empty();
    let (_inp_id, vals) = store.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![]);
    let (inc1_id, _) = store.add_op(TestInstructionSet::Inc, svec![vals[0]]);
    let (_inc2_id, _) = store.add_op(TestInstructionSet::Inc, svec![vals[0]]);

    // Value should have 2 users
    assert_eq!(store.get_val(vals[0]).get_users_iter().count(), 2);

    store.delete_op(inc1_id);

    // Value should still have 1 user, but the deleted op shouldn't appear in iteration
    let remaining_users: Vec<_> = store
        .get_val(vals[0])
        .raw_get_uses_iter()
        .map(|op| op.opref.get_id())
        .collect();
    assert_eq!(remaining_users.len(), 2); // Raw users list still contains deleted op

    // But active users should only show the remaining one
    let active_users: Vec<_> = store
        .get_val(vals[0])
        .get_users_iter()
        // .filter(|op| op.is_active())
        .collect();
    assert_eq!(active_users.len(), 1);
}

/// Tests has_opid/has_valid behavior with deleted elements
#[test]
fn test_has_id_with_deleted_elements() {
    let mut store: IR<TestLang> = IR::empty();
    let (op_id, vals) = store.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![]);

    assert!(store.has_opid(op_id));
    assert!(store.has_valid(vals[0]));

    store.delete_op(op_id);

    // Public API should return false for deleted elements
    assert!(!store.has_opid(op_id));
    assert!(!store.has_valid(vals[0]));

    // Raw API should still return true
    assert!(store.raw_has_opid(op_id));
    assert!(store.raw_has_valid(vals[0]));
}

/// Tests topological ordering with deleted operations
#[test]
fn test_topological_order_with_deletions() {
    let mut store: IR<TestLang> = IR::empty();
    let (op1_id, vals1) = store.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![]);
    let (op2_id, vals2) = store.add_op(TestInstructionSet::Inc, svec![vals1[0]]);
    let (op3_id, _vals3) = store.add_op(TestInstructionSet::Inc, svec![vals2[0]]);
    let (op4_id, _) = store.add_op(TestInstructionSet::Inc, svec![vals1[0]]); // Independent branch

    // Delete operations in reverse dependency order (leaf first)
    store.delete_op(op3_id); // Delete op3 first since it depends on op2
    store.delete_op(op2_id); // Now safe to delete op2

    // Topological order should include deleted operations in raw iterator
    let all_topo: Vec<_> = store.raw_topological_opwalker().collect();
    assert_eq!(all_topo.len(), 4);
    assert!(all_topo.contains(&op1_id));
    assert!(all_topo.contains(&op2_id));
    assert!(all_topo.contains(&op3_id));
    assert!(all_topo.contains(&op4_id));

    // Order should still be maintained: op1, then op2/op4 (same depth), then op3
    let op1_pos = all_topo.iter().position(|&id| id == op1_id).unwrap();
    let op2_pos = all_topo.iter().position(|&id| id == op2_id).unwrap();
    let op3_pos = all_topo.iter().position(|&id| id == op3_id).unwrap();
    let op4_pos = all_topo.iter().position(|&id| id == op4_id).unwrap();

    assert!(op1_pos < op2_pos);
    assert!(op1_pos < op4_pos);
    assert!(op2_pos < op3_pos);
}

#[test]
fn test_batch_delete_empty() {
    let mut ir = IR::<TestLang>::empty();
    let (op1, _) = ir.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![]);

    // Empty batch should be no-op
    ir.batch_delete_op(std::iter::empty());

    assert!(ir.has_opid(op1));
    assert_eq!(ir.n_ops(), 1);
}

#[test]
fn test_batch_delete_single() {
    let mut ir = IR::<TestLang>::empty();
    let (op1, vals1) = ir.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![]);
    let (op2, _) = ir.add_op(TestInstructionSet::Return, vals1);

    ir.batch_delete_op(std::iter::once(op2));

    assert!(ir.has_opid(op1));
    assert!(!ir.has_opid(op2));
    assert_eq!(ir.n_ops(), 1);
}

#[test]
fn test_batch_delete_dependency_chain() {
    let mut ir = IR::<TestLang>::empty();
    let (op1, vals1) = ir.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![]);
    let (op2, vals2) = ir.add_op(TestInstructionSet::Inc, vals1);
    let (op3, vals3) = ir.add_op(TestInstructionSet::Inc, vals2);
    let (op4, _) = ir.add_op(TestInstructionSet::Return, vals3);

    // Delete the entire chain
    ir.batch_delete_op([op2, op3, op4].into_iter());

    assert!(ir.has_opid(op1));
    assert!(!ir.has_opid(op2));
    assert!(!ir.has_opid(op3));
    assert!(!ir.has_opid(op4));
    assert_eq!(ir.n_ops(), 1);
}

#[test]
fn test_batch_delete_order_independence() {
    let mut ir = IR::<TestLang>::empty();
    let (op1, vals1) = ir.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![]);
    let (op2, vals2) = ir.add_op(TestInstructionSet::Inc, vals1);
    let (op3, _) = ir.add_op(TestInstructionSet::Return, vals2);

    // Delete in reverse dependency order - should still work
    ir.batch_delete_op([op2, op3].into_iter());

    assert!(ir.has_opid(op1));
    assert!(!ir.has_opid(op2));
    assert!(!ir.has_opid(op3));
    assert_eq!(ir.n_ops(), 1);
}

#[test]
fn test_batch_delete_diamond_pattern() {
    let mut ir = IR::<TestLang>::empty();
    let (op1, vals1) = ir.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![]);
    let (op2, vals2) = ir.add_op(TestInstructionSet::Inc, vals1.clone());
    let (op3, vals3) = ir.add_op(TestInstructionSet::Inc, vals1);
    let (op4, _) = ir.add_op(TestInstructionSet::Add, svec![vals2[0], vals3[0]]);

    // Delete the diamond (op2, op3, op4) but leave op1
    ir.batch_delete_op([op2, op3, op4].into_iter());

    assert!(ir.has_opid(op1));
    assert!(!ir.has_opid(op2));
    assert!(!ir.has_opid(op3));
    assert!(!ir.has_opid(op4));
    assert_eq!(ir.n_ops(), 1);
}

#[test]
fn test_batch_delete_independent_operations() {
    let mut ir = IR::<TestLang>::empty();
    let (op1, vals1) = ir.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![]);
    let (op2, vals2) = ir.add_op(TestInstructionSet::IntInput { pos: 1 }, svec![]);
    let (op3, _) = ir.add_op(TestInstructionSet::Return, vals1);
    let (op4, _) = ir.add_op(TestInstructionSet::Return, vals2);

    // Delete two independent subgraphs
    ir.batch_delete_op([op3, op4].into_iter());

    assert!(ir.has_opid(op1));
    assert!(ir.has_opid(op2));
    assert!(!ir.has_opid(op3));
    assert!(!ir.has_opid(op4));
    assert_eq!(ir.n_ops(), 2);
}

#[test]
#[should_panic(expected = "Tried to delete an operation whose return values are still in use")]
fn test_batch_delete_with_external_users() {
    let mut ir = IR::<TestLang>::empty();
    let (_op1, vals1) = ir.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![]);
    let (op2, vals2) = ir.add_op(TestInstructionSet::Inc, vals1);
    let (op3, _) = ir.add_op(TestInstructionSet::Return, vals2.clone());
    let (_op4, _) = ir.add_op(TestInstructionSet::Return, vals2); // op4 also uses vals?2

    // Try to delete op2 and op3, but op4 still uses vals2 from op2
    ir.batch_delete_op([op2, op3].into_iter());
}

#[test]
fn test_batch_delete_partial_dependency_closure() {
    let mut ir = IR::<TestLang>::empty();
    let (op1, vals1) = ir.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![]);
    let (op2, vals2) = ir.add_op(TestInstructionSet::Inc, vals1);
    let (op3, vals3) = ir.add_op(TestInstructionSet::Inc, vals2.clone());
    let (op4, _) = ir.add_op(TestInstructionSet::Return, vals3);
    let (op5, _) = ir.add_op(TestInstructionSet::Return, vals2); // Also uses vals2

    // Can delete op3 and op4 (leaves op2 and op5 intact)
    ir.batch_delete_op([op3, op4].into_iter());

    assert!(ir.has_opid(op1));
    assert!(ir.has_opid(op2));
    assert!(!ir.has_opid(op3));
    assert!(!ir.has_opid(op4));
    assert!(ir.has_opid(op5));
    assert_eq!(ir.n_ops(), 3);
}

#[test]
#[should_panic(expected = "Tried to get a dead op")]
fn test_batch_delete_already_deleted() {
    let mut ir = IR::<TestLang>::empty();
    let (_, vals1) = ir.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![]);
    let (op2, _) = ir.add_op(TestInstructionSet::Return, vals1);

    ir.delete_op(op2); // Delete normally first

    // Try to batch delete already deleted operation
    ir.batch_delete_op(std::iter::once(op2));
}
