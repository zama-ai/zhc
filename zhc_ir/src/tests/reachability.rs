//! Reachability between ops: `reaches`, `get_reaching_iter`, and `get_reached_iter`.

use crate::IR;
use crate::tests::testlang::{TestInstructionSet, TestLang};
use zhc_utils::{assert_display_is, svec};

/// Tests that an operation reaches itself (reflexive property)
#[test]
fn test_reaches_self() {
    let mut store: IR<TestLang> = IR::empty();
    let (lhs_id, _) = store.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![]);
    let lhs = store.get_op(lhs_id);
    assert_display_is!(
        store.format(),
        r#"
            %0 = int_input<pos: 0>();
        "#
    );
    assert!(lhs.reaches(&lhs));
}

/// Tests basic reachability between two operations in a dependency chain
#[test]
fn test_reaches_base() {
    let mut store: IR<TestLang> = IR::empty();
    let (lhs_id, v0) = store.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![]);
    let (ulhs_id, _) = store.add_op(TestInstructionSet::Inc, svec![v0[0]]);
    let lhs = store.get_op(lhs_id);
    let ulhs = store.get_op(ulhs_id);
    assert_display_is!(
        store.format(),
        r#"
            %0 = int_input<pos: 0>();
            %1 = inc(%0);
        "#
    );
    assert!(lhs.reaches(&ulhs));
}

/// Tests reachability through a longer chain of operations
#[test]
fn test_reaches_chain() {
    let mut store: IR<TestLang> = IR::empty();
    let (lhs_id, v0) = store.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![]);
    let (_, v1) = store.add_op(TestInstructionSet::Inc, svec![v0[0]]);
    let (_, v2) = store.add_op(TestInstructionSet::Inc, svec![v1[0]]);
    let (_, v3) = store.add_op(TestInstructionSet::Inc, svec![v2[0]]);
    let (_, v4) = store.add_op(TestInstructionSet::Inc, svec![v3[0]]);
    let (ulhs_id, _) = store.add_op(TestInstructionSet::Inc, svec![v4[0]]);
    let lhs = store.get_op(lhs_id);
    let ulhs = store.get_op(ulhs_id);
    assert_display_is!(
        store.format(),
        r#"
            %0 = int_input<pos: 0>();
            %1 = inc(%0);
            %2 = inc(%1);
            %3 = inc(%2);
            %4 = inc(%3);
            %5 = inc(%4);
        "#
    );
    assert!(lhs.reaches(&ulhs));
}

/// Tests that independent operations don't reach each other
#[test]
fn test_reaches_happy_path() {
    let mut store: IR<TestLang> = IR::empty();
    let (lhs_id, v0) = store.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![]);
    let (rhs_id, _) = store.add_op(TestInstructionSet::IntInput { pos: 1 }, svec![]);
    let _ = store.add_op(TestInstructionSet::Inc, svec![v0[0]]);
    let lhs = store.get_op(lhs_id);
    let rhs = store.get_op(rhs_id);
    assert_display_is!(
        store.format(),
        r#"
            %0 = int_input<pos: 0>();
            %1 = int_input<pos: 1>();
            %2 = inc(%0);
        "#
    );
    assert!(!lhs.reaches(&rhs));
}

/// Tests get_reaching_iter returns all operations that can reach the current operation
#[test]
fn test_get_reaching_iter_simple() {
    let mut store: IR<TestLang> = IR::empty();
    let (inp1_id, v0) = store.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![]);
    let (inp2_id, v1) = store.add_op(TestInstructionSet::IntInput { pos: 1 }, svec![]);
    let (add_id, v2) = store.add_op(TestInstructionSet::Add, svec![v0[0], v1[0]]);
    let (inc_id, _) = store.add_op(TestInstructionSet::Inc, svec![v2[0]]);

    let inc_op = store.get_op(inc_id);
    let reaching_ops: Vec<_> = inc_op.get_reaching_iter().map(|op| op.get_id()).collect();

    // inc operation should reach all its predecessors: add, inp1, inp2
    assert_eq!(reaching_ops.len(), 3);
    assert!(reaching_ops.contains(&inp1_id));
    assert!(reaching_ops.contains(&inp2_id));
    assert!(reaching_ops.contains(&add_id));
}

/// Tests get_reaching_iter with complex dependency graph
#[test]
fn test_get_reaching_iter_complex() {
    let mut store: IR<TestLang> = IR::empty();
    let (inp_id, v0) = store.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![]);
    let (inc1_id, v1) = store.add_op(TestInstructionSet::Inc, svec![v0[0]]);
    let (inc2_id, v2) = store.add_op(TestInstructionSet::Inc, svec![v1[0]]);
    let (inc3_id, v3) = store.add_op(TestInstructionSet::Inc, svec![v0[0]]); // Alternative branch
    let (add_id, _) = store.add_op(TestInstructionSet::Add, svec![v2[0], v3[0]]);

    let add_op = store.get_op(add_id);
    let reaching_ops: Vec<_> = add_op.get_reaching_iter().map(|op| op.get_id()).collect();

    // add should reach inp, inc1, inc2, inc3
    assert_eq!(reaching_ops.len(), 4);
    assert!(reaching_ops.contains(&inp_id));
    assert!(reaching_ops.contains(&inc1_id));
    assert!(reaching_ops.contains(&inc2_id));
    assert!(reaching_ops.contains(&inc3_id));
}

/// Tests get_reaching_iter with diamond pattern
#[test]
fn test_get_reaching_iter_diamond() {
    let mut store: IR<TestLang> = IR::empty();
    let (a_id, a_vals) = store.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![]); // A
    let (b_id, b_vals) = store.add_op(TestInstructionSet::Inc, svec![a_vals[0]]); // B depends on A
    let (c_id, c_vals) = store.add_op(TestInstructionSet::Inc, svec![a_vals[0]]); // C depends on A
    let (d_id, _) = store.add_op(TestInstructionSet::Add, svec![b_vals[0], c_vals[0]]); // D depends on B,C

    let d_op = store.get_op(d_id);
    let reaching_ops: Vec<_> = d_op.get_reaching_iter().map(|op| op.get_id()).collect();

    // D should reach A, B, C but not include itself
    assert_eq!(reaching_ops.len(), 3);
    assert!(reaching_ops.contains(&a_id));
    assert!(reaching_ops.contains(&b_id));
    assert!(reaching_ops.contains(&c_id));
    assert!(!reaching_ops.contains(&d_id));
}

/// Tests get_reaching_iter on input operation (no predecessors)
#[test]
fn test_get_reaching_iter_input() {
    let mut store: IR<TestLang> = IR::empty();
    let (inp_id, _) = store.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![]);

    let inp_op = store.get_op(inp_id);
    let reaching_ops: Vec<_> = inp_op.get_reaching_iter().map(|op| op.get_id()).collect();

    // Input operation has no predecessors
    assert_eq!(reaching_ops.len(), 0);
}

/// Tests get_reaching_iter with two completely disconnected subgraphs
#[test]
fn test_get_reaching_iter_disconnected_subgraphs() {
    let mut store: IR<TestLang> = IR::empty();

    // Subgraph 1: inp1 → inc1 → inc2
    let (inp1_id, v0) = store.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![]);
    let (inc1_id, v1) = store.add_op(TestInstructionSet::Inc, svec![v0[0]]);
    let (inc2_id, _v2) = store.add_op(TestInstructionSet::Inc, svec![v1[0]]);

    // Subgraph 2: inp2 → inc3 → inc4
    let (inp2_id, v3) = store.add_op(TestInstructionSet::IntInput { pos: 1 }, svec![]);
    let (inc3_id, v4) = store.add_op(TestInstructionSet::Inc, svec![v3[0]]);
    let (inc4_id, _v5) = store.add_op(TestInstructionSet::Inc, svec![v4[0]]);

    // Test that subgraph 1 operations only reach within their own subgraph
    let inc2_op = store.get_op(inc2_id);
    let inc2_reaching: Vec<_> = inc2_op.get_reaching_iter().map(|op| op.get_id()).collect();

    assert_eq!(inc2_reaching.len(), 2); // Should only reach inp1 and inc1
    assert!(inc2_reaching.contains(&inp1_id));
    assert!(inc2_reaching.contains(&inc1_id));
    assert!(!inc2_reaching.contains(&inp2_id));
    assert!(!inc2_reaching.contains(&inc3_id));
    assert!(!inc2_reaching.contains(&inc4_id));

    // Test that subgraph 2 operations only reach within their own subgraph
    let inc4_op = store.get_op(inc4_id);
    let inc4_reaching: Vec<_> = inc4_op.get_reaching_iter().map(|op| op.get_id()).collect();

    assert_eq!(inc4_reaching.len(), 2); // Should only reach inp2 and inc3
    assert!(inc4_reaching.contains(&inp2_id));
    assert!(inc4_reaching.contains(&inc3_id));
    assert!(!inc4_reaching.contains(&inp1_id));
    assert!(!inc4_reaching.contains(&inc1_id));
    assert!(!inc4_reaching.contains(&inc2_id));

    // Test that operations from different subgraphs don't reach each other
    assert!(!inc2_op.reaches(&inc4_op));
    assert!(!inc4_op.reaches(&inc2_op));
    assert!(!store.get_op(inc1_id).reaches(&store.get_op(inc3_id)));
    assert!(!store.get_op(inc3_id).reaches(&store.get_op(inc1_id)));
}

/// Tests get_reached_iter returns all operations that can be reached from the current operation
#[test]
fn test_get_reached_iter_simple() {
    let mut store: IR<TestLang> = IR::empty();
    let (inp1_id, v0) = store.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![]);
    let (inp2_id, v1) = store.add_op(TestInstructionSet::IntInput { pos: 1 }, svec![]);
    let (add_id, v2) = store.add_op(TestInstructionSet::Add, svec![v0[0], v1[0]]);
    let (inc_id, _) = store.add_op(TestInstructionSet::Inc, svec![v2[0]]);

    let add_op = store.get_op(add_id);
    let reached_ops: Vec<_> = add_op.get_reached_iter().map(|op| op.get_id()).collect();

    // add operation should reach its successors: inc
    assert_eq!(reached_ops.len(), 1);
    assert!(reached_ops.contains(&inc_id));
    assert!(!reached_ops.contains(&inp1_id));
    assert!(!reached_ops.contains(&inp2_id));
    assert!(!reached_ops.contains(&add_id));
}

/// Tests get_reached_iter with complex dependency graph
#[test]
fn test_get_reached_iter_complex() {
    let mut store: IR<TestLang> = IR::empty();
    let (inp_id, v0) = store.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![]);
    let (inc1_id, v1) = store.add_op(TestInstructionSet::Inc, svec![v0[0]]);
    let (inc2_id, v2) = store.add_op(TestInstructionSet::Inc, svec![v1[0]]);
    let (inc3_id, v3) = store.add_op(TestInstructionSet::Inc, svec![v0[0]]); // Alternative branch
    let (add_id, _) = store.add_op(TestInstructionSet::Add, svec![v2[0], v3[0]]);

    let inp_op = store.get_op(inp_id);
    let reached_ops: Vec<_> = inp_op.get_reached_iter().map(|op| op.get_id()).collect();

    // inp should reach inc1, inc2, inc3, add
    assert_eq!(reached_ops.len(), 4);
    assert!(reached_ops.contains(&inc1_id));
    assert!(reached_ops.contains(&inc2_id));
    assert!(reached_ops.contains(&inc3_id));
    assert!(reached_ops.contains(&add_id));
}

/// Tests get_reached_iter with diamond pattern
#[test]
fn test_get_reached_iter_diamond() {
    let mut store: IR<TestLang> = IR::empty();
    let (a_id, a_vals) = store.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![]); // A
    let (b_id, b_vals) = store.add_op(TestInstructionSet::Inc, svec![a_vals[0]]); // B depends on A
    let (c_id, c_vals) = store.add_op(TestInstructionSet::Inc, svec![a_vals[0]]); // C depends on A
    let (d_id, _) = store.add_op(TestInstructionSet::Add, svec![b_vals[0], c_vals[0]]); // D depends on B,C

    let a_op = store.get_op(a_id);
    let reached_ops: Vec<_> = a_op.get_reached_iter().map(|op| op.get_id()).collect();

    // A should reach B, C, D but not include itself
    assert_eq!(reached_ops.len(), 3);
    assert!(reached_ops.contains(&b_id));
    assert!(reached_ops.contains(&c_id));
    assert!(reached_ops.contains(&d_id));
    assert!(!reached_ops.contains(&a_id));
}

/// Tests get_reached_iter on operation with no successors (effect)
#[test]
fn test_get_reached_iter_effect() {
    let mut store: IR<TestLang> = IR::empty();
    let (_, vals) = store.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![]);
    let (ret_id, _) = store.add_op(TestInstructionSet::Return, vals);

    let ret_op = store.get_op(ret_id);
    let reached_ops: Vec<_> = ret_op.get_reached_iter().map(|op| op.get_id()).collect();

    // Return operation has no successors
    assert_eq!(reached_ops.len(), 0);
}

/// Tests get_reached_iter with two completely disconnected subgraphs
#[test]
fn test_get_reached_iter_disconnected_subgraphs() {
    let mut store: IR<TestLang> = IR::empty();

    // Subgraph 1: inp1 → inc1 → inc2
    let (inp1_id, v0) = store.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![]);
    let (inc1_id, v1) = store.add_op(TestInstructionSet::Inc, svec![v0[0]]);
    let (inc2_id, _v2) = store.add_op(TestInstructionSet::Inc, svec![v1[0]]);

    // Subgraph 2: inp2 → inc3 → inc4
    let (inp2_id, v3) = store.add_op(TestInstructionSet::IntInput { pos: 1 }, svec![]);
    let (inc3_id, v4) = store.add_op(TestInstructionSet::Inc, svec![v3[0]]);
    let (inc4_id, _v5) = store.add_op(TestInstructionSet::Inc, svec![v4[0]]);

    // Test that subgraph 1 operations only reach within their own subgraph
    let inp1_op = store.get_op(inp1_id);
    let inp1_reached: Vec<_> = inp1_op.get_reached_iter().map(|op| op.get_id()).collect();

    assert_eq!(inp1_reached.len(), 2); // Should only reach inc1 and inc2
    assert!(inp1_reached.contains(&inc1_id));
    assert!(inp1_reached.contains(&inc2_id));
    assert!(!inp1_reached.contains(&inp2_id));
    assert!(!inp1_reached.contains(&inc3_id));
    assert!(!inp1_reached.contains(&inc4_id));

    // Test that subgraph 2 operations only reach within their own subgraph
    let inp2_op = store.get_op(inp2_id);
    let inp2_reached: Vec<_> = inp2_op.get_reached_iter().map(|op| op.get_id()).collect();

    assert_eq!(inp2_reached.len(), 2); // Should only reach inc3 and inc4
    assert!(inp2_reached.contains(&inc3_id));
    assert!(inp2_reached.contains(&inc4_id));
    assert!(!inp2_reached.contains(&inp1_id));
    assert!(!inp2_reached.contains(&inc1_id));
    assert!(!inp2_reached.contains(&inc2_id));

    // Test that operations from different subgraphs don't reach each other
    assert!(!inp1_op.reaches(&store.get_op(inc3_id)));
    assert!(!inp2_op.reaches(&store.get_op(inc1_id)));
    assert!(!store.get_op(inc1_id).reaches(&store.get_op(inc3_id)));
    assert!(!store.get_op(inc3_id).reaches(&store.get_op(inc1_id)));
}

/// Tests get_reached_iter with branching pattern (one-to-many)
#[test]
fn test_get_reached_iter_branching() {
    let mut store: IR<TestLang> = IR::empty();
    let (inp_id, vals) = store.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![]);
    let (ret1_id, _) = store.add_op(TestInstructionSet::Return, vals.clone());
    let (inc_id, inc_vals) = store.add_op(TestInstructionSet::Inc, vals);
    let (ret2_id, _) = store.add_op(TestInstructionSet::Return, inc_vals);

    let inp_op = store.get_op(inp_id);
    let reached_ops: Vec<_> = inp_op.get_reached_iter().map(|op| op.get_id()).collect();

    // inp should reach all three operations that use its output
    assert_eq!(reached_ops.len(), 3);
    assert!(reached_ops.contains(&ret1_id));
    assert!(reached_ops.contains(&inc_id));
    assert!(reached_ops.contains(&ret2_id));
}

/// Tests get_reached_iter with convergent pattern (many-to-one)
#[test]
fn test_get_reached_iter_convergent() {
    let mut store: IR<TestLang> = IR::empty();
    let (inp1_id, v0) = store.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![]);
    let (inp2_id, v1) = store.add_op(TestInstructionSet::IntInput { pos: 1 }, svec![]);
    let (add_id, _) = store.add_op(TestInstructionSet::Add, svec![v0[0], v1[0]]);

    // Both inputs should reach the same add operation
    let inp1_op = store.get_op(inp1_id);
    let inp1_reached: Vec<_> = inp1_op.get_reached_iter().map(|op| op.get_id()).collect();

    let inp2_op = store.get_op(inp2_id);
    let inp2_reached: Vec<_> = inp2_op.get_reached_iter().map(|op| op.get_id()).collect();

    // Both should reach the add operation
    assert_eq!(inp1_reached.len(), 1);
    assert_eq!(inp2_reached.len(), 1);
    assert!(inp1_reached.contains(&add_id));
    assert!(inp2_reached.contains(&add_id));
}
