//! Building an IR: adding ops, signature checks, and basic graph shapes.

use crate::tests::testlang::{TestInstructionSet, TestLang};
use crate::{DialectInstructionSet, IR};
use zhc_utils::{assert_display_is, iter::CollectInVec, svec};

/// Tests basic IR construction with complex operation graph and validates
/// all operation properties, value relationships, and depth calculations
#[test]
fn test_construction() {
    let mut store: IR<TestLang> = IR::empty();

    let (lhs_id, v0) = store.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![]);
    let (rhs_id, v1) = store.add_op(TestInstructionSet::IntInput { pos: 1 }, svec![]);
    let (join_id, v2) = store.add_op(TestInstructionSet::Add, svec![v0[0], v1[0]]);
    let (split_id, v3) = store.add_op(TestInstructionSet::DivRem, svec![v2[0], v0[0]]);
    let (ulhs_id, v4) = store.add_op(TestInstructionSet::Inc, svec![v3[0]]);
    let (urhs_id, v5) = store.add_op(TestInstructionSet::Inc, svec![v3[1]]);
    let (final_add_id, v6) = store.add_op(TestInstructionSet::Add, svec![v4[0], v5[0]]);
    let (effect_id, _) = store.add_op(TestInstructionSet::Return, svec![v3[0]]);

    let lhs = store.get_op(lhs_id);
    let p0 = store.get_val(v0[0]);
    let rhs = store.get_op(rhs_id);
    let p1 = store.get_val(v1[0]);
    let join = store.get_op(join_id);
    let p2 = store.get_val(v2[0]);
    let split = store.get_op(split_id);
    let p3 = store.get_val(v3[0]);
    let p4 = store.get_val(v3[1]);
    let ulhs = store.get_op(ulhs_id);
    let p5 = store.get_val(v4[0]);
    let urhs = store.get_op(urhs_id);
    let p6 = store.get_val(v5[0]);
    let final_add = store.get_op(final_add_id);
    let p7 = store.get_val(v6[0]);
    let effect = store.get_op(effect_id);

    assert_display_is!(
        store.format(),
        r#"
            %0 = int_input<pos: 0>();
            %1 = int_input<pos: 1>();
            %2 = add(%0, %1);
            %3, %4 = div_rem(%2, %0);
            %5 = inc(%3);
            %6 = inc(%4);
            %7 = add(%5, %6);
            return(%3);
        "#
    );

    assert_eq!(store.n_ops(), 8);
    assert_eq!(store.n_vals(), 8);
    assert!(lhs.is_active());
    assert_eq!(lhs.get_depth(), 1);
    assert_eq!(lhs.get_args_iter().covec(), []);
    assert_eq!(lhs.get_returns_iter().covec(), [p0.clone()]);

    assert!(rhs.is_active());
    assert_eq!(rhs.get_depth(), 1);
    assert_eq!(rhs.get_args_iter().covec(), []);
    assert_eq!(rhs.get_returns_iter().covec(), [p1.clone()]);

    assert!(p0.is_active());
    assert_eq!(p0.get_origin().opref, lhs);
    assert_eq!(p0.get_users_iter().covec(), [join.clone(), split.clone()]);

    assert!(p1.is_active());
    assert_eq!(p1.get_origin().opref, rhs);
    assert_eq!(p1.get_users_iter().covec(), [join.clone()]);

    assert!(join.is_active());
    assert_eq!(join.get_depth(), 2);
    assert_eq!(join.get_args_iter().covec(), [p0.clone(), p1.clone()]);
    assert_eq!(join.get_returns_iter().covec(), [p2.clone()]);

    assert!(p2.is_active());
    assert_eq!(p2.get_origin().opref, join);
    assert_eq!(p2.get_users_iter().covec(), [split.clone()]);

    assert!(split.is_active());
    assert_eq!(split.get_depth(), 3);
    assert_eq!(split.get_args_iter().covec(), [p2.clone(), p0.clone()]);
    assert_eq!(split.get_returns_iter().covec(), [p3.clone(), p4.clone()]);

    assert!(p3.is_active());
    assert_eq!(p3.get_origin().opref, split);
    assert_eq!(p3.get_users_iter().covec(), [ulhs.clone(), effect.clone()]);

    assert!(p4.is_active());
    assert_eq!(p4.get_origin().opref, split);
    assert_eq!(p4.get_users_iter().covec(), [urhs.clone()]);

    assert!(ulhs.is_active());
    assert_eq!(ulhs.get_depth(), 4);
    assert_eq!(ulhs.get_args_iter().covec(), [p3.clone()]);
    assert_eq!(ulhs.get_returns_iter().covec(), [p5.clone()]);

    assert!(p5.is_active());
    assert_eq!(p5.get_origin().opref, ulhs);
    assert_eq!(p5.get_users_iter().covec(), [final_add.clone()]);

    assert!(urhs.is_active());
    assert_eq!(urhs.get_depth(), 4);
    assert_eq!(urhs.get_args_iter().covec(), [p4.clone()]);
    assert_eq!(urhs.get_returns_iter().covec(), [p6.clone()]);

    assert!(p6.is_active());
    assert_eq!(p6.get_origin().opref, urhs);
    assert_eq!(p6.get_users_iter().covec(), [final_add.clone()]);

    assert!(final_add.is_active());
    assert_eq!(final_add.get_depth(), 5);
    assert_eq!(final_add.get_args_iter().covec(), [p5.clone(), p6.clone()]);
    assert_eq!(final_add.get_returns_iter().covec(), [p7.clone()]);

    assert!(p7.is_active());
    assert_eq!(p7.get_origin().opref, final_add);
    assert_eq!(p7.get_users_iter().covec(), []);

    assert!(effect.is_active());
    assert_eq!(effect.get_depth(), 4);
    assert_eq!(effect.get_args_iter().covec(), [p3.clone()]);
    assert_eq!(effect.get_returns_iter().covec(), []);
}

/// Tests that add_op panics when argument types don't match operation signature
#[test]
#[should_panic(expected = "add_op failed: Signature error: received [Bool] instead of [Int]")]
fn test_add_op_type_mismatch() {
    let mut store: IR<TestLang> = IR::empty();
    let (_, bool_val) = store.add_op(TestInstructionSet::BoolConstant { val: true }, svec![]);
    // Try to use bool value where int is expected
    store.add_op(TestInstructionSet::Inc, svec![bool_val[0]]);
}

/// Tests that add_op return error when wrong number of arguments provided
#[test]
#[should_panic(expected = "add_op failed: Signature error: received [Int] instead of [Int, Int]")]
fn test_add_op_wrong_arg_count() {
    let mut store: IR<TestLang> = IR::empty();
    let (_, int_vals) = store.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![]);
    // Add operation expects 2 args, providing only 1
    store.add_op(TestInstructionSet::Add, svec![int_vals[0]]);
}

/// Tests that using inactive ValId in add_op panics
#[test]
#[should_panic(expected = "add_op failed: Inactive value: %0")]
fn test_add_op_with_deleted_value() {
    let mut store: IR<TestLang> = IR::empty();
    let (op_id, vals) = store.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![]);
    store.delete_op(op_id);
    // Try to use deleted value
    store.add_op(TestInstructionSet::Inc, svec![vals[0]]);
}

/// Tests that accessing deleted operation via public API panics
#[test]
#[should_panic(expected = "Tried to get a dead op")]
fn test_get_deleted_op() {
    let mut store: IR<TestLang> = IR::empty();
    let (op_id, _) = store.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![]);
    store.delete_op(op_id);
    store.get_op(op_id);
}

/// Tests that accessing deleted value via public API panics
#[test]
#[should_panic(expected = "Tried to get a dead val")]
fn test_get_deleted_val() {
    let mut store: IR<TestLang> = IR::empty();
    let (op_id, vals) = store.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![]);
    store.delete_op(op_id);
    store.get_val(vals[0]);
}

/// Tests using same value multiple times in single operation
#[test]
fn test_same_value_multiple_args() {
    let mut store: IR<TestLang> = IR::empty();
    let (_, vals) = store.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![]);
    let (add_id, _) = store.add_op(TestInstructionSet::Add, svec![vals[0], vals[0]]);

    let val = store.get_val(vals[0]);
    let add_op = store.get_op(add_id);

    // Value should appear in users list only once, but op uses it twice
    assert_eq!(val.get_users_iter().count(), 1);
    assert_eq!(
        add_op
            .get_args_iter()
            .filter(|v| v.get_id() == vals[0])
            .count(),
        2
    );
}

/// Tests diamond dependency pattern (A→B, A→C, B→D, C→D)
#[test]
fn test_diamond_dependencies() {
    let mut store: IR<TestLang> = IR::empty();
    let (_, a_vals) = store.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![]); // A
    let (_b_id, b_vals) = store.add_op(TestInstructionSet::Inc, svec![a_vals[0]]); // B depends on A
    let (_c_id, c_vals) = store.add_op(TestInstructionSet::Inc, svec![a_vals[0]]); // C depends on A
    let (d_id, _) = store.add_op(TestInstructionSet::Add, svec![b_vals[0], c_vals[0]]); // D depends on B,C

    let a_val = store.get_val(a_vals[0]);
    let d_op = store.get_op(d_id);

    // A should have 2 users (B and C)
    assert_eq!(a_val.get_users_iter().count(), 2);
    // D should be at depth 3 (A:1 → B,C:2 → D:3)
    assert_eq!(d_op.get_depth(), 3);
}

/// Tests multiple independent subgraphs in same IR
#[test]
fn test_independent_subgraphs() {
    let mut store: IR<TestLang> = IR::empty();

    // Subgraph 1: input1 → inc1
    let (_, vals1) = store.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![]);
    let (inc1_id, _) = store.add_op(TestInstructionSet::Inc, svec![vals1[0]]);

    // Subgraph 2: input2 → inc2
    let (_, vals2) = store.add_op(TestInstructionSet::IntInput { pos: 1 }, svec![]);
    let (inc2_id, _) = store.add_op(TestInstructionSet::Inc, svec![vals2[0]]);

    let inc1 = store.get_op(inc1_id);
    let inc2 = store.get_op(inc2_id);

    // Neither should reach the other
    assert!(!inc1.reaches(&inc2));
    assert!(!inc2.reaches(&inc1));
}

/// Tests operations with multi-return where returns have different user patterns
#[test]
fn test_multi_return_different_users() {
    let mut store: IR<TestLang> = IR::empty();
    let (_, inp1) = store.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![]);
    let (_, inp2) = store.add_op(TestInstructionSet::IntInput { pos: 1 }, svec![]);
    let (_, div_vals) = store.add_op(TestInstructionSet::DivRem, svec![inp1[0], inp2[0]]);

    // Use first return in one op, second return in another
    let (_, _) = store.add_op(TestInstructionSet::Inc, svec![div_vals[0]]); // Use quotient
    let (_, _) = store.add_op(TestInstructionSet::Inc, svec![div_vals[1]]); // Use remainder

    let quot = store.get_val(div_vals[0]);
    let rem = store.get_val(div_vals[1]);

    assert_eq!(quot.get_users_iter().count(), 1);
    assert_eq!(rem.get_users_iter().count(), 1);
    assert_ne!(
        quot.get_users_iter().next().unwrap().get_id(),
        rem.get_users_iter().next().unwrap().get_id()
    );
}

/// Tests operations on empty IR
#[test]
fn test_empty_ir_operations() {
    let store: IR<TestLang> = IR::empty();

    assert_eq!(store.n_ops(), 0);
    assert_eq!(store.n_vals(), 0);
    assert_eq!(store.walk_ops_linear().count(), 0);

    // Check that topological order works on empty IR
    let topo_ops: Vec<_> = store.raw_walk_ops_topo().collect();
    assert_eq!(topo_ops.len(), 0);
}

#[test]
fn test_is_effect() {
    let mut ir = IR::<TestLang>::empty();
    let (_, inp1) = ir.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![]);
    let (_, inp2) = ir.add_op(TestInstructionSet::IntInput { pos: 1 }, svec![]);
    let (add_op, add) = ir.add_op(TestInstructionSet::Add, svec![inp1[0], inp2[0]]);
    let (ret_op, _) = ir.add_op(TestInstructionSet::Return, svec![add[0]]);

    assert!(ir.get_op(ret_op).is_effect());
    assert!(!ir.get_op(add_op).is_effect());
}

#[test]
fn test_signature_consistency() {
    let mut ir = IR::<TestLang>::empty();
    let (_, inp1) = ir.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![]);
    let (_, inp2) = ir.add_op(TestInstructionSet::IntInput { pos: 1 }, svec![]);
    let (add_op, _) = ir.add_op(TestInstructionSet::Add, svec![inp1[0], inp2[0]]);
    let op_ref = ir.get_op(add_op);

    // Verify cached signature matches operation signature
    assert_eq!(op_ref.signature, &op_ref.operation.get_signature());
}
