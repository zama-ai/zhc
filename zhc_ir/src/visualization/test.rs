use super::*;
use crate::IR;
use crate::testlang::{TestInstructionSet, TestLang};
use crate::visualization::Hierarchy;
use zhc_utils::svec;

/// Build a small IR and annotate it with a flat hierarchy (all ops at root).
#[test]
fn test_flat_hierarchy() {
    // Build IR: inp0 -> inc -> ret
    let mut ir: IR<TestLang> = IR::empty();
    let (_, inp) = ir.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![]);
    let (_, inc) = ir.add_op(TestInstructionSet::Inc, svec![inp[0]]);
    ir.add_op(TestInstructionSet::Return, svec![inc[0]]);

    // Annotate all ops with the root hierarchy
    let root = Hierarchy::new();
    let op_annotations = ir.filled_opmap(root);

    draw_ir_html(&ir, op_annotations, "test1.html");
}

/// Build an IR where some ops are in a nested hierarchy level.
#[test]
fn test_nested_hierarchy() {
    // Build IR: inp0 -> inc1 -> inc2 -> ret
    let mut ir: IR<TestLang> = IR::empty();
    let (op0, inp) = ir.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![]);
    let (op1, inc1) = ir.add_op(TestInstructionSet::Inc, svec![inp[0]]);
    let (op2, inc2) = ir.add_op(TestInstructionSet::Inc, svec![inc1[0]]);
    let (op3, _) = ir.add_op(TestInstructionSet::Return, svec![inc2[0]]);

    // Create hierarchies: root and a child "group_a"
    let root = Hierarchy::new();
    let mut group_a = root.clone();
    group_a.push("group_a");

    // Annotate: op0, op3 at root; op1, op2 in group_a
    let mut op_annotations = ir.empty_opmap();
    op_annotations.insert(op0, root.clone());
    op_annotations.insert(op1, group_a.clone());
    op_annotations.insert(op2, group_a.clone());
    op_annotations.insert(op3, root.clone());

    draw_ir_html(&ir, op_annotations, "test2.html");
}

/// Test with two separate groups at the same level.
#[test]
fn test_sibling_groups() {
    // Build IR: inp0, inp1 -> add -> inc -> ret
    //           where add is in group_a, inc is in group_b
    let mut ir: IR<TestLang> = IR::empty();
    let (op0, inp0) = ir.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![]);
    let (op1, inp1) = ir.add_op(TestInstructionSet::IntInput { pos: 1 }, svec![]);
    let (op2, add) = ir.add_op(TestInstructionSet::Add, svec![inp0[0], inp1[0]]);
    let (op3, inc) = ir.add_op(TestInstructionSet::Inc, svec![add[0]]);
    let (op4, _) = ir.add_op(TestInstructionSet::Return, svec![inc[0]]);

    let root = Hierarchy::new();
    let mut group_a = root.clone();
    group_a.push("group_a");
    let mut group_b = root.clone();
    group_b.push("group_b");

    let mut op_annotations = ir.empty_opmap();
    op_annotations.insert(op0, root.clone());
    op_annotations.insert(op1, root.clone());
    op_annotations.insert(op2, group_a.clone());
    op_annotations.insert(op3, group_b.clone());
    op_annotations.insert(op4, root.clone());
    draw_ir_html(&ir, op_annotations, "test3.html");
}

/// Test deeply nested hierarchy (2 levels deep).
#[test]
fn test_deep_nesting() {
    // Build IR: inp -> inc1 -> inc2 -> ret
    // Hierarchy: inp at root, inc1 in group_a, inc2 in group_a/group_b, ret at root
    let mut ir: IR<TestLang> = IR::empty();
    let (op0, inp) = ir.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![]);
    let (op1, inc1) = ir.add_op(TestInstructionSet::Inc, svec![inp[0]]);
    let (op2, inc2) = ir.add_op(TestInstructionSet::Inc, svec![inc1[0]]);
    let (op3, _) = ir.add_op(TestInstructionSet::Return, svec![inc2[0]]);

    let root = Hierarchy::new();
    let mut group_a = root.clone();
    group_a.push("group_a");
    let mut group_ab = group_a.clone();
    group_ab.push("group_b");

    let mut op_annotations = ir.empty_opmap();
    op_annotations.insert(op0, root.clone());
    op_annotations.insert(op1, group_a.clone());
    op_annotations.insert(op2, group_ab.clone());
    op_annotations.insert(op3, root.clone());

    draw_ir_html(&ir, op_annotations, "test4.html");
}

/// Operations along a group boundary (enter/exit same group multiple times).
#[test]
fn test_operations_along_group() {
    // root -> group_a -> root -> group_a -> root
    let mut ir: IR<TestLang> = IR::empty();
    let (op0, inp) = ir.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![]);
    let (op1, inc1) = ir.add_op(TestInstructionSet::Inc, svec![inp[0]]);
    let (op2, inc2) = ir.add_op(TestInstructionSet::Inc, svec![inc1[0]]);
    let (op3, inc3) = ir.add_op(TestInstructionSet::Inc, svec![inc2[0]]);
    let (op4, _) = ir.add_op(TestInstructionSet::Return, svec![inc3[0]]);

    let root = Hierarchy::new();
    let mut group_a = root.clone();
    group_a.push("group_a");

    let mut op_annotations = ir.empty_opmap();
    op_annotations.insert(op0, root.clone());
    op_annotations.insert(op1, group_a.clone());
    op_annotations.insert(op2, root.clone());
    op_annotations.insert(op3, group_a.clone());
    op_annotations.insert(op4, root.clone());
    draw_ir_html(&ir, op_annotations, "test5.html");
}

/// Diamond with different path lengths (slack test).
#[test]
fn test_diamond_different_slacks() {
    // inp -> inc1 -> inc2 -> inc3 -> add -> ret
    //    \------------------> inc4 -/
    let mut ir: IR<TestLang> = IR::empty();
    let (op0, inp) = ir.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![]);
    let (op1, inc1) = ir.add_op(TestInstructionSet::Inc, svec![inp[0]]);
    let (op2, inc2) = ir.add_op(TestInstructionSet::Inc, svec![inc1[0]]);
    let (op3, inc3) = ir.add_op(TestInstructionSet::Inc, svec![inc2[0]]);
    let (op4, inc4) = ir.add_op(TestInstructionSet::Inc, svec![inp[0]]);
    let (op5, add) = ir.add_op(TestInstructionSet::Add, svec![inc3[0], inc4[0]]);
    let (op6, _) = ir.add_op(TestInstructionSet::Return, svec![add[0]]);

    let root = Hierarchy::new();
    let mut group_a = root.clone();
    group_a.push("group_a");

    // Long path in group_a, short path at root
    let mut op_annotations = ir.empty_opmap();
    op_annotations.insert(op0, root.clone());
    op_annotations.insert(op1, group_a.clone());
    op_annotations.insert(op2, group_a.clone());
    op_annotations.insert(op3, group_a.clone());
    op_annotations.insert(op4, root.clone());
    op_annotations.insert(op5, root.clone());
    op_annotations.insert(op6, root.clone());
    draw_ir_html(&ir, op_annotations, "test6.html");
}

/// Branches with asymmetric slack in nested groups.
#[test]
fn test_asymmetric_slack_nested() {
    // inp -> inc1 -> inc2 -> add -> ret
    //    \---------> inc3 --/
    // inc1, inc2 in group_a/group_b; inc3 in group_c
    let mut ir: IR<TestLang> = IR::empty();
    let (op0, inp) = ir.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![]);
    let (op1, inc1) = ir.add_op(TestInstructionSet::Inc, svec![inp[0]]);
    let (op2, inc2) = ir.add_op(TestInstructionSet::Inc, svec![inc1[0]]);
    let (op3, inc3) = ir.add_op(TestInstructionSet::Inc, svec![inp[0]]);
    let (op4, add) = ir.add_op(TestInstructionSet::Add, svec![inc2[0], inc3[0]]);
    let (op5, _) = ir.add_op(TestInstructionSet::Return, svec![add[0]]);

    let root = Hierarchy::new();
    let mut group_a = root.clone();
    group_a.push("group_a");
    let mut group_ab = group_a.clone();
    group_ab.push("group_b");
    let mut group_c = root.clone();
    group_c.push("group_c");

    let mut op_annotations = ir.empty_opmap();
    op_annotations.insert(op0, root.clone());
    op_annotations.insert(op1, group_ab.clone());
    op_annotations.insert(op2, group_ab.clone());
    op_annotations.insert(op3, group_c.clone());
    op_annotations.insert(op4, root.clone());
    op_annotations.insert(op5, root.clone());
    draw_ir_html(&ir, op_annotations, "test7.html");
}

/// Deep entry immediately, slow exit with ops at each level.
#[test]
fn test_deep_entry_slow_exit() {
    // inp (root) -> inc1 (a/b/c) -> inc2 (a/b) -> inc3 (a) -> ret (root)
    let mut ir: IR<TestLang> = IR::empty();
    let (op0, inp) = ir.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![]);
    let (op1, inc1) = ir.add_op(TestInstructionSet::Inc, svec![inp[0]]);
    let (op2, inc2) = ir.add_op(TestInstructionSet::Inc, svec![inc1[0]]);
    let (op3, inc3) = ir.add_op(TestInstructionSet::Inc, svec![inc2[0]]);
    let (op4, _) = ir.add_op(TestInstructionSet::Return, svec![inc3[0]]);

    let root = Hierarchy::new();
    let mut group_a = root.clone();
    group_a.push("a");
    let mut group_ab = group_a.clone();
    group_ab.push("b");
    let mut group_abc = group_ab.clone();
    group_abc.push("c");

    let mut op_annotations = ir.empty_opmap();
    op_annotations.insert(op0, root.clone());
    op_annotations.insert(op1, group_abc.clone());
    op_annotations.insert(op2, group_ab.clone());
    op_annotations.insert(op3, group_a.clone());
    op_annotations.insert(op4, root.clone());
    draw_ir_html(&ir, op_annotations, "test8.html");
}

/// Slow entry with ops at each level, deep exit immediately.
#[test]
fn test_slow_entry_deep_exit() {
    // inp (root) -> inc1 (a) -> inc2 (a/b) -> inc3 (a/b/c) -> ret (root)
    let mut ir: IR<TestLang> = IR::empty();
    let (op0, inp) = ir.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![]);
    let (op1, inc1) = ir.add_op(TestInstructionSet::Inc, svec![inp[0]]);
    let (op2, inc2) = ir.add_op(TestInstructionSet::Inc, svec![inc1[0]]);
    let (op3, inc3) = ir.add_op(TestInstructionSet::Inc, svec![inc2[0]]);
    let (op4, _) = ir.add_op(TestInstructionSet::Return, svec![inc3[0]]);

    let root = Hierarchy::new();
    let mut group_a = root.clone();
    group_a.push("a");
    let mut group_ab = group_a.clone();
    group_ab.push("b");
    let mut group_abc = group_ab.clone();
    group_abc.push("c");

    let mut op_annotations = ir.empty_opmap();
    op_annotations.insert(op0, root.clone());
    op_annotations.insert(op1, group_a.clone());
    op_annotations.insert(op2, group_ab.clone());
    op_annotations.insert(op3, group_abc.clone());
    op_annotations.insert(op4, root.clone());
    draw_ir_html(&ir, op_annotations, "test9.html");
}

/// Immediate deep entry and exit (no intermediate ops).
#[test]
fn test_immediate_deep_entry_exit() {
    // inp (root) -> inc (a/b/c) -> ret (root)
    let mut ir: IR<TestLang> = IR::empty();
    let (op0, inp) = ir.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![]);
    let (op1, inc) = ir.add_op(TestInstructionSet::Inc, svec![inp[0]]);
    let (op2, _) = ir.add_op(TestInstructionSet::Return, svec![inc[0]]);

    let root = Hierarchy::new();
    let mut group_abc = root.clone();
    group_abc.push("a");
    group_abc.push("b");
    group_abc.push("c");

    let mut op_annotations = ir.empty_opmap();
    op_annotations.insert(op0, root.clone());
    op_annotations.insert(op1, group_abc.clone());
    op_annotations.insert(op2, root.clone());
    draw_ir_html(&ir, op_annotations, "test10.html");
}

/// Multiple ops deep, immediate jump out and back in.
#[test]
fn test_deep_oscillation() {
    // All at a/b/c except inc2 at root
    let mut ir: IR<TestLang> = IR::empty();
    let (op0, inp) = ir.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![]);
    let (op1, inc1) = ir.add_op(TestInstructionSet::Inc, svec![inp[0]]);
    let (op2, inc2) = ir.add_op(TestInstructionSet::Inc, svec![inc1[0]]);
    let (op3, inc3) = ir.add_op(TestInstructionSet::Inc, svec![inc2[0]]);
    let (op4, _) = ir.add_op(TestInstructionSet::Return, svec![inc3[0]]);

    let root = Hierarchy::new();
    let mut group_abc = root.clone();
    group_abc.push("a");
    group_abc.push("b");
    group_abc.push("c");

    let mut op_annotations = ir.empty_opmap();
    op_annotations.insert(op0, group_abc.clone());
    op_annotations.insert(op1, group_abc.clone());
    op_annotations.insert(op2, root.clone());
    op_annotations.insert(op3, group_abc.clone());
    op_annotations.insert(op4, group_abc.clone());
    draw_ir_html(&ir, op_annotations, "test11.html");
}

/// Fan-out with different nesting depths per branch.
#[test]
fn test_fanout_varied_depths() {
    // inp -> inc1 (a/b) -> add -> ret
    //    \-> inc2 (c)   -/
    //    \-> inc3 (root)-/
    let mut ir: IR<TestLang> = IR::empty();
    let (op0, inp) = ir.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![]);
    let (op1, inc1) = ir.add_op(TestInstructionSet::Inc, svec![inp[0]]);
    let (op2, inc2) = ir.add_op(TestInstructionSet::Inc, svec![inp[0]]);
    let (op3, inc3) = ir.add_op(TestInstructionSet::Inc, svec![inp[0]]);
    let (op4, add1) = ir.add_op(TestInstructionSet::Add, svec![inc1[0], inc2[0]]);
    let (op5, add2) = ir.add_op(TestInstructionSet::Add, svec![add1[0], inc3[0]]);
    let (op6, _) = ir.add_op(TestInstructionSet::Return, svec![add2[0]]);

    let root = Hierarchy::new();
    let mut group_ab = root.clone();
    group_ab.push("a");
    group_ab.push("b");
    let mut group_c = root.clone();
    group_c.push("c");

    let mut op_annotations = ir.empty_opmap();
    op_annotations.insert(op0, root.clone());
    op_annotations.insert(op1, group_ab.clone());
    op_annotations.insert(op2, group_c.clone());
    op_annotations.insert(op3, root.clone());
    op_annotations.insert(op4, root.clone());
    op_annotations.insert(op5, root.clone());
    op_annotations.insert(op6, root.clone());
    draw_ir_html(&ir, op_annotations, "test12.html");
}

/// Paths of lengths 1-6 converging, producing slacks 5 down to 0.
#[test]
fn test_slack_gradient_0_to_5() {
    // 6 parallel paths of increasing length, all converging via chained adds.
    // Path lengths: 1, 2, 3, 4, 5, 6 → slacks: 5, 4, 3, 2, 1, 0
    let mut ir: IR<TestLang> = IR::empty();

    // Inputs
    let (inp_id, inp) = ir.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![]);

    // Path of length 6 (critical path, slack 0)
    let (p6_1_id, p6_1) = ir.add_op(TestInstructionSet::Inc, svec![inp[0]]);
    let (p6_2_id, p6_2) = ir.add_op(TestInstructionSet::Inc, svec![p6_1[0]]);
    let (p6_3_id, p6_3) = ir.add_op(TestInstructionSet::Inc, svec![p6_2[0]]);
    let (p6_4_id, p6_4) = ir.add_op(TestInstructionSet::Inc, svec![p6_3[0]]);
    let (p6_5_id, p6_5) = ir.add_op(TestInstructionSet::Inc, svec![p6_4[0]]);
    let (p6_6_id, p6_6) = ir.add_op(TestInstructionSet::Inc, svec![p6_5[0]]);

    // Path of length 5 (slack 1)
    let (p5_1_id, p5_1) = ir.add_op(TestInstructionSet::Inc, svec![inp[0]]);
    let (p5_2_id, p5_2) = ir.add_op(TestInstructionSet::Inc, svec![p5_1[0]]);
    let (p5_3_id, p5_3) = ir.add_op(TestInstructionSet::Inc, svec![p5_2[0]]);
    let (p5_4_id, p5_4) = ir.add_op(TestInstructionSet::Inc, svec![p5_3[0]]);
    let (p5_5_id, p5_5) = ir.add_op(TestInstructionSet::Inc, svec![p5_4[0]]);

    // Path of length 4 (slack 2)
    let (p4_1_id, p4_1) = ir.add_op(TestInstructionSet::Inc, svec![inp[0]]);
    let (p4_2_id, p4_2) = ir.add_op(TestInstructionSet::Inc, svec![p4_1[0]]);
    let (p4_3_id, p4_3) = ir.add_op(TestInstructionSet::Inc, svec![p4_2[0]]);
    let (p4_4_id, p4_4) = ir.add_op(TestInstructionSet::Inc, svec![p4_3[0]]);

    // Path of length 3 (slack 3)
    let (p3_1_id, p3_1) = ir.add_op(TestInstructionSet::Inc, svec![inp[0]]);
    let (p3_2_id, p3_2) = ir.add_op(TestInstructionSet::Inc, svec![p3_1[0]]);
    let (p3_3_id, p3_3) = ir.add_op(TestInstructionSet::Inc, svec![p3_2[0]]);

    // Path of length 2 (slack 4)
    let (p2_1_id, p2_1) = ir.add_op(TestInstructionSet::Inc, svec![inp[0]]);
    let (p2_2_id, p2_2) = ir.add_op(TestInstructionSet::Inc, svec![p2_1[0]]);

    // Path of length 1 (slack 5)
    let (p1_1_id, p1_1) = ir.add_op(TestInstructionSet::Inc, svec![inp[0]]);

    // Converge all paths via chained adds
    let (add1_id, add1) = ir.add_op(TestInstructionSet::Add, svec![p6_6[0], p5_5[0]]);
    let (add2_id, add2) = ir.add_op(TestInstructionSet::Add, svec![add1[0], p4_4[0]]);
    let (add3_id, add3) = ir.add_op(TestInstructionSet::Add, svec![add2[0], p3_3[0]]);
    let (add4_id, add4) = ir.add_op(TestInstructionSet::Add, svec![add3[0], p2_2[0]]);
    let (add5_id, add5) = ir.add_op(TestInstructionSet::Add, svec![add4[0], p1_1[0]]);
    let (ret_id, _) = ir.add_op(TestInstructionSet::Return, svec![add5[0]]);

    // Each path in its own group
    let root = Hierarchy::new();
    let mut g6 = root.clone();
    g6.push("p6");
    let mut g5 = root.clone();
    g5.push("p5");
    let mut g4 = root.clone();
    g4.push("p4");
    let mut g3 = root.clone();
    g3.push("p3");
    let mut g2 = root.clone();
    g2.push("p2");
    let mut g1 = root.clone();
    g1.push("p1");

    let mut op_annotations = ir.empty_opmap();
    op_annotations.insert(inp_id, root.clone());
    op_annotations.insert(p6_1_id, g6.clone());
    op_annotations.insert(p6_2_id, g6.clone());
    op_annotations.insert(p6_3_id, g6.clone());
    op_annotations.insert(p6_4_id, g6.clone());
    op_annotations.insert(p6_5_id, g6.clone());
    op_annotations.insert(p6_6_id, g6.clone());
    op_annotations.insert(p5_1_id, g5.clone());
    op_annotations.insert(p5_2_id, g5.clone());
    op_annotations.insert(p5_3_id, g5.clone());
    op_annotations.insert(p5_4_id, g5.clone());
    op_annotations.insert(p5_5_id, g5.clone());
    op_annotations.insert(p4_1_id, g4.clone());
    op_annotations.insert(p4_2_id, g4.clone());
    op_annotations.insert(p4_3_id, g4.clone());
    op_annotations.insert(p4_4_id, g4.clone());
    op_annotations.insert(p3_1_id, g3.clone());
    op_annotations.insert(p3_2_id, g3.clone());
    op_annotations.insert(p3_3_id, g3.clone());
    op_annotations.insert(p2_1_id, g2.clone());
    op_annotations.insert(p2_2_id, g2.clone());
    op_annotations.insert(p1_1_id, g1.clone());
    op_annotations.insert(add1_id, root.clone());
    op_annotations.insert(add2_id, root.clone());
    op_annotations.insert(add3_id, root.clone());
    op_annotations.insert(add4_id, root.clone());
    op_annotations.insert(add5_id, root.clone());
    op_annotations.insert(ret_id, root.clone());
    draw_ir_html(&ir, op_annotations, "test13.html");
}

/// Multi-return op with outputs consumed at different depths.
#[test]
fn test_multireturn_different_depths() {
    // divrem produces two outputs; one consumed deep, one shallow
    let mut ir: IR<TestLang> = IR::empty();
    let (op0, inp0) = ir.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![]);
    let (op1, inp1) = ir.add_op(TestInstructionSet::IntInput { pos: 1 }, svec![]);
    let (op2, divrem) = ir.add_op(TestInstructionSet::DivRem, svec![inp0[0], inp1[0]]);
    let (op3, inc1) = ir.add_op(TestInstructionSet::Inc, svec![divrem[0]]); // quotient
    let (op4, inc2) = ir.add_op(TestInstructionSet::Inc, svec![divrem[1]]); // remainder
    let (op5, add) = ir.add_op(TestInstructionSet::Add, svec![inc1[0], inc2[0]]);
    let (op6, _) = ir.add_op(TestInstructionSet::Return, svec![add[0]]);

    let root = Hierarchy::new();
    let mut group_ab = root.clone();
    group_ab.push("a");
    group_ab.push("b");
    let mut group_c = root.clone();
    group_c.push("c");

    let mut op_annotations = ir.empty_opmap();
    op_annotations.insert(op0, root.clone());
    op_annotations.insert(op1, root.clone());
    op_annotations.insert(op2, root.clone());
    op_annotations.insert(op3, group_ab.clone()); // deep
    op_annotations.insert(op4, group_c.clone()); // shallow
    op_annotations.insert(op5, root.clone());
    op_annotations.insert(op6, root.clone());
    draw_ir_html(&ir, op_annotations, "test14.html");
}

/// Multiple inputs entering same group from root.
#[test]
fn test_multi_input_to_group() {
    // inp0, inp1, inp2 (root) -> add1, add2 (group_a) -> ret (root)
    let mut ir: IR<TestLang> = IR::empty();
    let (op0, inp0) = ir.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![]);
    let (op1, inp1) = ir.add_op(TestInstructionSet::IntInput { pos: 1 }, svec![]);
    let (op2, inp2) = ir.add_op(TestInstructionSet::IntInput { pos: 2 }, svec![]);
    let (op3, add1) = ir.add_op(TestInstructionSet::Add, svec![inp0[0], inp1[0]]);
    let (op4, add2) = ir.add_op(TestInstructionSet::Add, svec![add1[0], inp2[0]]);
    let (op5, _) = ir.add_op(TestInstructionSet::Return, svec![add2[0]]);

    let root = Hierarchy::new();
    let mut group_a = root.clone();
    group_a.push("group_a");

    let mut op_annotations = ir.empty_opmap();
    op_annotations.insert(op0, root.clone());
    op_annotations.insert(op1, root.clone());
    op_annotations.insert(op2, root.clone());
    op_annotations.insert(op3, group_a.clone());
    op_annotations.insert(op4, group_a.clone());
    op_annotations.insert(op5, root.clone());
    draw_ir_html(&ir, op_annotations, "test15.html");
}

/// Group produces multiple outputs consumed by different ops at root.
#[test]
fn test_multi_output_from_group() {
    // inp (root) -> inc1, inc2, inc3 (group_a) -> add1, add2 (root) -> ret
    let mut ir: IR<TestLang> = IR::empty();
    let (op0, inp) = ir.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![]);
    let (op1, inc1) = ir.add_op(TestInstructionSet::Inc, svec![inp[0]]);
    let (op2, inc2) = ir.add_op(TestInstructionSet::Inc, svec![inp[0]]);
    let (op3, inc3) = ir.add_op(TestInstructionSet::Inc, svec![inp[0]]);
    let (op4, add1) = ir.add_op(TestInstructionSet::Add, svec![inc1[0], inc2[0]]);
    let (op5, add2) = ir.add_op(TestInstructionSet::Add, svec![add1[0], inc3[0]]);
    let (op6, _) = ir.add_op(TestInstructionSet::Return, svec![add2[0]]);

    let root = Hierarchy::new();
    let mut group_a = root.clone();
    group_a.push("group_a");

    let mut op_annotations = ir.empty_opmap();
    op_annotations.insert(op0, root.clone());
    op_annotations.insert(op1, group_a.clone());
    op_annotations.insert(op2, group_a.clone());
    op_annotations.insert(op3, group_a.clone());
    op_annotations.insert(op4, root.clone());
    op_annotations.insert(op5, root.clone());
    op_annotations.insert(op6, root.clone());
    draw_ir_html(&ir, op_annotations, "test16.html");
}

/// Multiple inputs and outputs crossing group boundary simultaneously.
#[test]
fn test_multi_io_group() {
    // inp0, inp1 (root) -> add, inc (group_a) -> add2, inc2 (root) -> ret
    let mut ir: IR<TestLang> = IR::empty();
    let (op0, inp0) = ir.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![]);
    let (op1, inp1) = ir.add_op(TestInstructionSet::IntInput { pos: 1 }, svec![]);
    let (op2, add) = ir.add_op(TestInstructionSet::Add, svec![inp0[0], inp1[0]]);
    let (op3, inc) = ir.add_op(TestInstructionSet::Inc, svec![inp0[0]]);
    let (op4, add2) = ir.add_op(TestInstructionSet::Add, svec![add[0], inc[0]]);
    let (op5, inc2) = ir.add_op(TestInstructionSet::Inc, svec![add[0]]);
    let (op6, add3) = ir.add_op(TestInstructionSet::Add, svec![add2[0], inc2[0]]);
    let (op7, _) = ir.add_op(TestInstructionSet::Return, svec![add3[0]]);

    let root = Hierarchy::new();
    let mut group_a = root.clone();
    group_a.push("group_a");

    let mut op_annotations = ir.empty_opmap();
    op_annotations.insert(op0, root.clone());
    op_annotations.insert(op1, root.clone());
    op_annotations.insert(op2, group_a.clone());
    op_annotations.insert(op3, group_a.clone());
    op_annotations.insert(op4, root.clone());
    op_annotations.insert(op5, root.clone());
    op_annotations.insert(op6, root.clone());
    op_annotations.insert(op7, root.clone());
    draw_ir_html(&ir, op_annotations, "test17.html");
}

/// Large diamond subgraph entirely within a group.
#[test]
fn test_big_subgraph_diamond() {
    // inp (root) -> [inc1 -> inc2 -> inc3] (group_a)
    //                   \-> inc4 -> inc5 -/
    //            -> add (group_a) -> ret (root)
    let mut ir: IR<TestLang> = IR::empty();
    let (op0, inp) = ir.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![]);
    let (op1, inc1) = ir.add_op(TestInstructionSet::Inc, svec![inp[0]]);
    let (op2, inc2) = ir.add_op(TestInstructionSet::Inc, svec![inc1[0]]);
    let (op3, inc3) = ir.add_op(TestInstructionSet::Inc, svec![inc2[0]]);
    let (op4, inc4) = ir.add_op(TestInstructionSet::Inc, svec![inc1[0]]);
    let (op5, inc5) = ir.add_op(TestInstructionSet::Inc, svec![inc4[0]]);
    let (op6, add) = ir.add_op(TestInstructionSet::Add, svec![inc3[0], inc5[0]]);
    let (op7, _) = ir.add_op(TestInstructionSet::Return, svec![add[0]]);

    let root = Hierarchy::new();
    let mut group_a = root.clone();
    group_a.push("group_a");

    let mut op_annotations = ir.empty_opmap();
    op_annotations.insert(op0, root.clone());
    op_annotations.insert(op1, group_a.clone());
    op_annotations.insert(op2, group_a.clone());
    op_annotations.insert(op3, group_a.clone());
    op_annotations.insert(op4, group_a.clone());
    op_annotations.insert(op5, group_a.clone());
    op_annotations.insert(op6, group_a.clone());
    op_annotations.insert(op7, root.clone());
    draw_ir_html(&ir, op_annotations, "test18.html");
}

/// Chain of 8 ops inside nested group with single entry/exit.
#[test]
fn test_long_chain_in_nested_group() {
    let mut ir: IR<TestLang> = IR::empty();
    let (op0, inp) = ir.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![]);
    let (op1, v1) = ir.add_op(TestInstructionSet::Inc, svec![inp[0]]);
    let (op2, v2) = ir.add_op(TestInstructionSet::Inc, svec![v1[0]]);
    let (op3, v3) = ir.add_op(TestInstructionSet::Inc, svec![v2[0]]);
    let (op4, v4) = ir.add_op(TestInstructionSet::Inc, svec![v3[0]]);
    let (op5, v5) = ir.add_op(TestInstructionSet::Inc, svec![v4[0]]);
    let (op6, v6) = ir.add_op(TestInstructionSet::Inc, svec![v5[0]]);
    let (op7, v7) = ir.add_op(TestInstructionSet::Inc, svec![v6[0]]);
    let (op8, v8) = ir.add_op(TestInstructionSet::Inc, svec![v7[0]]);
    let (op9, _) = ir.add_op(TestInstructionSet::Return, svec![v8[0]]);

    let root = Hierarchy::new();
    let mut group_ab = root.clone();
    group_ab.push("a");
    group_ab.push("b");

    let mut op_annotations = ir.empty_opmap();
    op_annotations.insert(op0, root.clone());
    op_annotations.insert(op1, group_ab.clone());
    op_annotations.insert(op2, group_ab.clone());
    op_annotations.insert(op3, group_ab.clone());
    op_annotations.insert(op4, group_ab.clone());
    op_annotations.insert(op5, group_ab.clone());
    op_annotations.insert(op6, group_ab.clone());
    op_annotations.insert(op7, group_ab.clone());
    op_annotations.insert(op8, group_ab.clone());
    op_annotations.insert(op9, root.clone());
    draw_ir_html(&ir, op_annotations, "test19.html");
}

/// Cross-group edges: two groups each receive input and produce output to the other.
#[test]
fn test_cross_group_multi_edge() {
    // inp0 -> inc1 (group_a) -> add1 (group_b) -> ret
    // inp1 -> inc2 (group_b) -> add2 (group_a) -/
    let mut ir: IR<TestLang> = IR::empty();
    let (op0, inp0) = ir.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![]);
    let (op1, inp1) = ir.add_op(TestInstructionSet::IntInput { pos: 1 }, svec![]);
    let (op2, inc1) = ir.add_op(TestInstructionSet::Inc, svec![inp0[0]]);
    let (op3, inc2) = ir.add_op(TestInstructionSet::Inc, svec![inp1[0]]);
    let (op4, add1) = ir.add_op(TestInstructionSet::Add, svec![inc1[0], inc2[0]]);
    let (op5, add2) = ir.add_op(TestInstructionSet::Add, svec![inc1[0], inc2[0]]);
    let (op6, add3) = ir.add_op(TestInstructionSet::Add, svec![add1[0], add2[0]]);
    let (op7, _) = ir.add_op(TestInstructionSet::Return, svec![add3[0]]);

    let root = Hierarchy::new();
    let mut group_a = root.clone();
    group_a.push("group_a");
    let mut group_b = root.clone();
    group_b.push("group_b");

    let mut op_annotations = ir.empty_opmap();
    op_annotations.insert(op0, root.clone());
    op_annotations.insert(op1, root.clone());
    op_annotations.insert(op2, group_a.clone());
    op_annotations.insert(op3, group_b.clone());
    op_annotations.insert(op4, group_b.clone());
    op_annotations.insert(op5, group_a.clone());
    op_annotations.insert(op6, root.clone());
    op_annotations.insert(op7, root.clone());
    draw_ir_html(&ir, op_annotations, "test20.html");
}

/// Nested groups with multiple inputs at different depths.
#[test]
fn test_multi_input_nested_depths() {
    // inp0 (root) -> inc1 (a) -> add1 (a/b) -> ret (root)
    // inp1 (root) -> inc2 (a/b) -/
    // inp2 (root) --------------/
    let mut ir: IR<TestLang> = IR::empty();
    let (op0, inp0) = ir.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![]);
    let (op1, inp1) = ir.add_op(TestInstructionSet::IntInput { pos: 1 }, svec![]);
    let (op2, inp2) = ir.add_op(TestInstructionSet::IntInput { pos: 2 }, svec![]);
    let (op3, inc1) = ir.add_op(TestInstructionSet::Inc, svec![inp0[0]]);
    let (op4, inc2) = ir.add_op(TestInstructionSet::Inc, svec![inp1[0]]);
    let (op5, add1) = ir.add_op(TestInstructionSet::Add, svec![inc1[0], inc2[0]]);
    let (op6, add2) = ir.add_op(TestInstructionSet::Add, svec![add1[0], inp2[0]]);
    let (op7, _) = ir.add_op(TestInstructionSet::Return, svec![add2[0]]);

    let root = Hierarchy::new();
    let mut group_a = root.clone();
    group_a.push("a");
    let mut group_ab = group_a.clone();
    group_ab.push("b");

    let mut op_annotations = ir.empty_opmap();
    op_annotations.insert(op0, root.clone());
    op_annotations.insert(op1, root.clone());
    op_annotations.insert(op2, root.clone());
    op_annotations.insert(op3, group_a.clone());
    op_annotations.insert(op4, group_ab.clone());
    op_annotations.insert(op5, group_ab.clone());
    op_annotations.insert(op6, group_ab.clone());
    op_annotations.insert(op7, root.clone());
    draw_ir_html(&ir, op_annotations, "test21.html");
}

/// Group with internal fanout: one input, multiple parallel chains, multiple outputs.
#[test]
fn test_group_internal_fanout() {
    // inp (root) -> inc1 -> inc2 (group_a) -> add1 (root) -> ret
    //           \-> inc3 -> inc4 (group_a) -/
    //           \-> inc5 -> inc6 (group_a) -> add2 (root) -/
    let mut ir: IR<TestLang> = IR::empty();
    let (op0, inp) = ir.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![]);
    let (op1, inc1) = ir.add_op(TestInstructionSet::Inc, svec![inp[0]]);
    let (op2, inc2) = ir.add_op(TestInstructionSet::Inc, svec![inc1[0]]);
    let (op3, inc3) = ir.add_op(TestInstructionSet::Inc, svec![inp[0]]);
    let (op4, inc4) = ir.add_op(TestInstructionSet::Inc, svec![inc3[0]]);
    let (op5, inc5) = ir.add_op(TestInstructionSet::Inc, svec![inp[0]]);
    let (op6, inc6) = ir.add_op(TestInstructionSet::Inc, svec![inc5[0]]);
    let (op7, add1) = ir.add_op(TestInstructionSet::Add, svec![inc2[0], inc4[0]]);
    let (op8, add2) = ir.add_op(TestInstructionSet::Add, svec![add1[0], inc6[0]]);
    let (op9, _) = ir.add_op(TestInstructionSet::Return, svec![add2[0]]);

    let root = Hierarchy::new();
    let mut group_a = root.clone();
    group_a.push("group_a");

    let mut op_annotations = ir.empty_opmap();
    op_annotations.insert(op0, root.clone());
    op_annotations.insert(op1, group_a.clone());
    op_annotations.insert(op2, group_a.clone());
    op_annotations.insert(op3, group_a.clone());
    op_annotations.insert(op4, group_a.clone());
    op_annotations.insert(op5, group_a.clone());
    op_annotations.insert(op6, group_a.clone());
    op_annotations.insert(op7, root.clone());
    op_annotations.insert(op8, root.clone());
    op_annotations.insert(op9, root.clone());
    draw_ir_html(&ir, op_annotations, "test22.html");
}

/// Two groups each with internal diamond, connected in sequence.
#[test]
fn test_sequential_diamonds_in_groups() {
    // inp -> [diamond in group_a] -> [diamond in group_b] -> ret
    let mut ir: IR<TestLang> = IR::empty();
    let (op0, inp) = ir.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![]);
    // Diamond 1 in group_a
    let (op1, a1) = ir.add_op(TestInstructionSet::Inc, svec![inp[0]]);
    let (op2, a2) = ir.add_op(TestInstructionSet::Inc, svec![a1[0]]);
    let (op3, a3) = ir.add_op(TestInstructionSet::Inc, svec![a1[0]]);
    let (op4, a4) = ir.add_op(TestInstructionSet::Add, svec![a2[0], a3[0]]);
    // Diamond 2 in group_b
    let (op5, b1) = ir.add_op(TestInstructionSet::Inc, svec![a4[0]]);
    let (op6, b2) = ir.add_op(TestInstructionSet::Inc, svec![b1[0]]);
    let (op7, b3) = ir.add_op(TestInstructionSet::Inc, svec![b1[0]]);
    let (op8, b4) = ir.add_op(TestInstructionSet::Add, svec![b2[0], b3[0]]);
    let (op9, _) = ir.add_op(TestInstructionSet::Return, svec![b4[0]]);

    let root = Hierarchy::new();
    let mut group_a = root.clone();
    group_a.push("group_a");
    let mut group_b = root.clone();
    group_b.push("group_b");

    let mut op_annotations = ir.empty_opmap();
    op_annotations.insert(op0, root.clone());
    op_annotations.insert(op1, group_a.clone());
    op_annotations.insert(op2, group_a.clone());
    op_annotations.insert(op3, group_a.clone());
    op_annotations.insert(op4, group_a.clone());
    op_annotations.insert(op5, group_b.clone());
    op_annotations.insert(op6, group_b.clone());
    op_annotations.insert(op7, group_b.clone());
    op_annotations.insert(op8, group_b.clone());
    op_annotations.insert(op9, root.clone());
    draw_ir_html(&ir, op_annotations, "test23.html");
}

/// Five inputs feeding into a deep nested group, five outputs exiting to root.
#[test]
fn test_wide_io_deep_group() {
    let mut ir: IR<TestLang> = IR::empty();
    let (i0, inp0) = ir.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![]);
    let (i1, inp1) = ir.add_op(TestInstructionSet::IntInput { pos: 1 }, svec![]);
    let (i2, inp2) = ir.add_op(TestInstructionSet::IntInput { pos: 2 }, svec![]);
    let (i3, inp3) = ir.add_op(TestInstructionSet::IntInput { pos: 3 }, svec![]);
    let (i4, inp4) = ir.add_op(TestInstructionSet::IntInput { pos: 4 }, svec![]);
    // Process inside group (a/b/c)
    let (g0, v0) = ir.add_op(TestInstructionSet::Inc, svec![inp0[0]]);
    let (g1, v1) = ir.add_op(TestInstructionSet::Inc, svec![inp1[0]]);
    let (g2, v2) = ir.add_op(TestInstructionSet::Inc, svec![inp2[0]]);
    let (g3, v3) = ir.add_op(TestInstructionSet::Inc, svec![inp3[0]]);
    let (g4, v4) = ir.add_op(TestInstructionSet::Inc, svec![inp4[0]]);
    // Converge at root
    let (c0, w0) = ir.add_op(TestInstructionSet::Add, svec![v0[0], v1[0]]);
    let (c1, w1) = ir.add_op(TestInstructionSet::Add, svec![w0[0], v2[0]]);
    let (c2, w2) = ir.add_op(TestInstructionSet::Add, svec![w1[0], v3[0]]);
    let (c3, w3) = ir.add_op(TestInstructionSet::Add, svec![w2[0], v4[0]]);
    let (r, _) = ir.add_op(TestInstructionSet::Return, svec![w3[0]]);

    let root = Hierarchy::new();
    let mut group_abc = root.clone();
    group_abc.push("a");
    group_abc.push("b");
    group_abc.push("c");

    let mut op_annotations = ir.empty_opmap();
    for id in [i0, i1, i2, i3, i4] {
        op_annotations.insert(id, root.clone());
    }
    for id in [g0, g1, g2, g3, g4] {
        op_annotations.insert(id, group_abc.clone());
    }
    for id in [c0, c1, c2, c3, r] {
        op_annotations.insert(id, root.clone());
    }
    draw_ir_html(&ir, op_annotations, "test24.html");
}

/// Two parallel paths with crossing edges if not reordered.
/// B -> D, A -> C but inputs inserted as B, A — requires input reorder.
#[test]
fn test_crossing_two_parallel() {
    let mut ir: IR<TestLang> = IR::empty();
    // Layer 0: inputs inserted in WRONG order (B before A)
    let (b, vb) = ir.add_op(TestInstructionSet::IntInput { pos: 1 }, svec![]);
    let (a, va) = ir.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![]);
    // Layer 1: C uses A, D uses B — also inserted in wrong order
    let (d, vd) = ir.add_op(TestInstructionSet::Inc, svec![vb[0]]);
    let (c, vc) = ir.add_op(TestInstructionSet::Inc, svec![va[0]]);
    // Layer 2: converge
    let (e, ve) = ir.add_op(TestInstructionSet::Add, svec![vc[0], vd[0]]);
    let (r, _) = ir.add_op(TestInstructionSet::Return, svec![ve[0]]);

    let root = Hierarchy::new();
    let mut op_annotations = ir.empty_opmap();
    for id in [a, b, c, d, e, r] {
        op_annotations.insert(id, root.clone());
    }
    draw_ir_html(&ir, op_annotations, "test25.html");
}

/// Three inputs, three outputs, all cross-connected (K₃,₃ bipartite).
/// Unavoidable crossings — tests graceful degradation.
#[test]
fn test_bipartite_k33() {
    let mut ir: IR<TestLang> = IR::empty();
    // Layer 0
    let (a, va) = ir.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![]);
    let (b, vb) = ir.add_op(TestInstructionSet::IntInput { pos: 1 }, svec![]);
    let (c, vc) = ir.add_op(TestInstructionSet::IntInput { pos: 2 }, svec![]);
    // Layer 1: each output depends on all inputs (via adds)
    let (d, vd) = ir.add_op(TestInstructionSet::Add, svec![va[0], vb[0]]);
    let (d2, vd2) = ir.add_op(TestInstructionSet::Add, svec![vd[0], vc[0]]);
    let (e, ve) = ir.add_op(TestInstructionSet::Add, svec![vb[0], vc[0]]);
    let (e2, ve2) = ir.add_op(TestInstructionSet::Add, svec![ve[0], va[0]]);
    let (f, vf) = ir.add_op(TestInstructionSet::Add, svec![vc[0], va[0]]);
    let (f2, vf2) = ir.add_op(TestInstructionSet::Add, svec![vf[0], vb[0]]);
    // Converge
    let (g, vg) = ir.add_op(TestInstructionSet::Add, svec![vd2[0], ve2[0]]);
    let (h, vh) = ir.add_op(TestInstructionSet::Add, svec![vg[0], vf2[0]]);
    let (r, _) = ir.add_op(TestInstructionSet::Return, svec![vh[0]]);

    let root = Hierarchy::new();
    let mut op_annotations = ir.empty_opmap();
    for id in [a, b, c, d, d2, e, e2, f, f2, g, h, r] {
        op_annotations.insert(id, root.clone());
    }
    draw_ir_html(&ir, op_annotations, "test26.html");
}

/// Fan-out to 4 children, connected to fan-in in reversed order.
/// Optimal layout requires full reversal of one layer.
#[test]
fn test_fanout_reversed_fanin() {
    let mut ir: IR<TestLang> = IR::empty();
    let (inp, v) = ir.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![]);
    // Fan-out: a, b, c, d
    let (a, va) = ir.add_op(TestInstructionSet::Inc, svec![v[0]]);
    let (b, vb) = ir.add_op(TestInstructionSet::Inc, svec![v[0]]);
    let (c, vc) = ir.add_op(TestInstructionSet::Inc, svec![v[0]]);
    let (d, vd) = ir.add_op(TestInstructionSet::Inc, svec![v[0]]);
    // Fan-in layer inserted reversed: connects to d, c, b, a
    let (w, vw) = ir.add_op(TestInstructionSet::Inc, svec![vd[0]]);
    let (x, vx) = ir.add_op(TestInstructionSet::Inc, svec![vc[0]]);
    let (y, vy) = ir.add_op(TestInstructionSet::Inc, svec![vb[0]]);
    let (z, vz) = ir.add_op(TestInstructionSet::Inc, svec![va[0]]);
    // Converge
    let (m1, vm1) = ir.add_op(TestInstructionSet::Add, svec![vw[0], vx[0]]);
    let (m2, vm2) = ir.add_op(TestInstructionSet::Add, svec![vy[0], vz[0]]);
    let (m3, vm3) = ir.add_op(TestInstructionSet::Add, svec![vm1[0], vm2[0]]);
    let (r, _) = ir.add_op(TestInstructionSet::Return, svec![vm3[0]]);

    let root = Hierarchy::new();
    let mut op_annotations = ir.empty_opmap();
    for id in [inp, a, b, c, d, w, x, y, z, m1, m2, m3, r] {
        op_annotations.insert(id, root.clone());
    }
    draw_ir_html(&ir, op_annotations, "test27.html");
}

/// Ladder pattern: pairs connected with alternating cross-links.
/// (a0,b0) -> (a1,b1) -> (a2,b2) with a_i->b_{i+1} and b_i->a_{i+1}.
#[test]
fn test_ladder_alternating() {
    let mut ir: IR<TestLang> = IR::empty();
    // Rung 0
    let (a0, va0) = ir.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![]);
    let (b0, vb0) = ir.add_op(TestInstructionSet::IntInput { pos: 1 }, svec![]);
    // Rung 1: cross-connected
    let (a1, va1) = ir.add_op(TestInstructionSet::Inc, svec![vb0[0]]);
    let (b1, vb1) = ir.add_op(TestInstructionSet::Inc, svec![va0[0]]);
    // Rung 2: cross-connected again
    let (a2, va2) = ir.add_op(TestInstructionSet::Inc, svec![vb1[0]]);
    let (b2, vb2) = ir.add_op(TestInstructionSet::Inc, svec![va1[0]]);
    // Rung 3: cross-connected
    let (a3, va3) = ir.add_op(TestInstructionSet::Inc, svec![vb2[0]]);
    let (b3, vb3) = ir.add_op(TestInstructionSet::Inc, svec![va2[0]]);
    // Converge
    let (m, vm) = ir.add_op(TestInstructionSet::Add, svec![va3[0], vb3[0]]);
    let (r, _) = ir.add_op(TestInstructionSet::Return, svec![vm[0]]);

    let root = Hierarchy::new();
    let mut op_annotations = ir.empty_opmap();
    for id in [a0, b0, a1, b1, a2, b2, a3, b3, m, r] {
        op_annotations.insert(id, root.clone());
    }
    draw_ir_html(&ir, op_annotations, "test28.html");
}

/// Permuted parallel chains: 4 chains inserted in shuffled order.
/// Tests barycentric heuristic on independent paths.
#[test]
fn test_permuted_chains() {
    let mut ir: IR<TestLang> = IR::empty();
    // Inputs in order 0,1,2,3
    let (i0, v0) = ir.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![]);
    let (i1, v1) = ir.add_op(TestInstructionSet::IntInput { pos: 1 }, svec![]);
    let (i2, v2) = ir.add_op(TestInstructionSet::IntInput { pos: 2 }, svec![]);
    let (i3, v3) = ir.add_op(TestInstructionSet::IntInput { pos: 3 }, svec![]);
    // Layer 1: inserted as 2,0,3,1 (shuffled)
    let (a2, va2) = ir.add_op(TestInstructionSet::Inc, svec![v2[0]]);
    let (a0, va0) = ir.add_op(TestInstructionSet::Inc, svec![v0[0]]);
    let (a3, va3) = ir.add_op(TestInstructionSet::Inc, svec![v3[0]]);
    let (a1, va1) = ir.add_op(TestInstructionSet::Inc, svec![v1[0]]);
    // Layer 2: inserted as 1,3,0,2 (different shuffle)
    let (b1, vb1) = ir.add_op(TestInstructionSet::Inc, svec![va1[0]]);
    let (b3, vb3) = ir.add_op(TestInstructionSet::Inc, svec![va3[0]]);
    let (b0, vb0) = ir.add_op(TestInstructionSet::Inc, svec![va0[0]]);
    let (b2, vb2) = ir.add_op(TestInstructionSet::Inc, svec![va2[0]]);
    // Converge in order
    let (m1, vm1) = ir.add_op(TestInstructionSet::Add, svec![vb0[0], vb1[0]]);
    let (m2, vm2) = ir.add_op(TestInstructionSet::Add, svec![vb2[0], vb3[0]]);
    let (m3, vm3) = ir.add_op(TestInstructionSet::Add, svec![vm1[0], vm2[0]]);
    let (r, _) = ir.add_op(TestInstructionSet::Return, svec![vm3[0]]);

    let root = Hierarchy::new();
    let mut op_annotations = ir.empty_opmap();
    for id in [
        i0, i1, i2, i3, a0, a1, a2, a3, b0, b1, b2, b3, m1, m2, m3, r,
    ] {
        op_annotations.insert(id, root.clone());
    }
    draw_ir_html(&ir, op_annotations, "test29.html");
}

/// Single source, multiple sinks at same depth with shared intermediate.
/// Tests horizontal ordering when sinks compete for position.
#[test]
fn test_shared_intermediate_multi_sink() {
    let mut ir: IR<TestLang> = IR::empty();
    let (inp, v) = ir.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![]);
    // Shared intermediate
    let (mid, vm) = ir.add_op(TestInstructionSet::Inc, svec![v[0]]);
    // Three sinks from shared, inserted in arbitrary order
    let (s2, vs2) = ir.add_op(TestInstructionSet::Inc, svec![vm[0]]);
    let (s0, vs0) = ir.add_op(TestInstructionSet::Inc, svec![vm[0]]);
    let (s1, vs1) = ir.add_op(TestInstructionSet::Inc, svec![vm[0]]);
    // Also direct edges from input to sinks (creating crossing potential)
    let (t0, vt0) = ir.add_op(TestInstructionSet::Add, svec![v[0], vs0[0]]);
    let (t1, vt1) = ir.add_op(TestInstructionSet::Add, svec![v[0], vs1[0]]);
    let (t2, vt2) = ir.add_op(TestInstructionSet::Add, svec![v[0], vs2[0]]);
    // Converge
    let (c1, vc1) = ir.add_op(TestInstructionSet::Add, svec![vt0[0], vt1[0]]);
    let (c2, vc2) = ir.add_op(TestInstructionSet::Add, svec![vc1[0], vt2[0]]);
    let (r, _) = ir.add_op(TestInstructionSet::Return, svec![vc2[0]]);

    let root = Hierarchy::new();
    let mut op_annotations = ir.empty_opmap();
    for id in [inp, mid, s0, s1, s2, t0, t1, t2, c1, c2, r] {
        op_annotations.insert(id, root.clone());
    }
    draw_ir_html(&ir, op_annotations, "test30.html");
}

/// Wide layer (8 nodes) with butterfly-pattern edges to next layer.
/// Stress test for reordering heuristics on wider graphs.
#[test]
fn test_butterfly_wide() {
    let mut ir: IR<TestLang> = IR::empty();
    // Layer 0: 8 inputs
    let (i0, v0) = ir.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![]);
    let (i1, v1) = ir.add_op(TestInstructionSet::IntInput { pos: 1 }, svec![]);
    let (i2, v2) = ir.add_op(TestInstructionSet::IntInput { pos: 2 }, svec![]);
    let (i3, v3) = ir.add_op(TestInstructionSet::IntInput { pos: 3 }, svec![]);
    let (i4, v4) = ir.add_op(TestInstructionSet::IntInput { pos: 4 }, svec![]);
    let (i5, v5) = ir.add_op(TestInstructionSet::IntInput { pos: 5 }, svec![]);
    let (i6, v6) = ir.add_op(TestInstructionSet::IntInput { pos: 6 }, svec![]);
    let (i7, v7) = ir.add_op(TestInstructionSet::IntInput { pos: 7 }, svec![]);
    // Layer 1: butterfly connections (i with i^1, like FFT)
    let (a0, va0) = ir.add_op(TestInstructionSet::Add, svec![v0[0], v1[0]]);
    let (a1, va1) = ir.add_op(TestInstructionSet::Add, svec![v1[0], v0[0]]);
    let (a2, va2) = ir.add_op(TestInstructionSet::Add, svec![v2[0], v3[0]]);
    let (a3, va3) = ir.add_op(TestInstructionSet::Add, svec![v3[0], v2[0]]);
    let (a4, va4) = ir.add_op(TestInstructionSet::Add, svec![v4[0], v5[0]]);
    let (a5, va5) = ir.add_op(TestInstructionSet::Add, svec![v5[0], v4[0]]);
    let (a6, va6) = ir.add_op(TestInstructionSet::Add, svec![v6[0], v7[0]]);
    let (a7, va7) = ir.add_op(TestInstructionSet::Add, svec![v7[0], v6[0]]);
    // Layer 2: butterfly with stride 2 (i with i^2)
    let (b0, vb0) = ir.add_op(TestInstructionSet::Add, svec![va0[0], va2[0]]);
    let (b1, vb1) = ir.add_op(TestInstructionSet::Add, svec![va1[0], va3[0]]);
    let (b2, vb2) = ir.add_op(TestInstructionSet::Add, svec![va2[0], va0[0]]);
    let (b3, vb3) = ir.add_op(TestInstructionSet::Add, svec![va3[0], va1[0]]);
    let (b4, vb4) = ir.add_op(TestInstructionSet::Add, svec![va4[0], va6[0]]);
    let (b5, vb5) = ir.add_op(TestInstructionSet::Add, svec![va5[0], va7[0]]);
    let (b6, vb6) = ir.add_op(TestInstructionSet::Add, svec![va6[0], va4[0]]);
    let (b7, vb7) = ir.add_op(TestInstructionSet::Add, svec![va7[0], va5[0]]);
    // Reduce
    let (c0, vc0) = ir.add_op(TestInstructionSet::Add, svec![vb0[0], vb1[0]]);
    let (c1, vc1) = ir.add_op(TestInstructionSet::Add, svec![vb2[0], vb3[0]]);
    let (c2, vc2) = ir.add_op(TestInstructionSet::Add, svec![vb4[0], vb5[0]]);
    let (c3, vc3) = ir.add_op(TestInstructionSet::Add, svec![vb6[0], vb7[0]]);
    let (d0, vd0) = ir.add_op(TestInstructionSet::Add, svec![vc0[0], vc1[0]]);
    let (d1, vd1) = ir.add_op(TestInstructionSet::Add, svec![vc2[0], vc3[0]]);
    let (e, ve) = ir.add_op(TestInstructionSet::Add, svec![vd0[0], vd1[0]]);
    let (r, _) = ir.add_op(TestInstructionSet::Return, svec![ve[0]]);

    let root = Hierarchy::new();
    let mut op_annotations = ir.empty_opmap();
    for id in [i0, i1, i2, i3, i4, i5, i6, i7] {
        op_annotations.insert(id, root.clone());
    }
    for id in [a0, a1, a2, a3, a4, a5, a6, a7] {
        op_annotations.insert(id, root.clone());
    }
    for id in [b0, b1, b2, b3, b4, b5, b6, b7] {
        op_annotations.insert(id, root.clone());
    }
    for id in [c0, c1, c2, c3, d0, d1, e, r] {
        op_annotations.insert(id, root.clone());
    }
    draw_ir_html(&ir, op_annotations, "test31.html");
}

/// Crossing within nested group — reordering must respect hierarchy.
#[test]
fn test_crossing_in_group() {
    let mut ir: IR<TestLang> = IR::empty();
    let (i0, v0) = ir.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![]);
    let (i1, v1) = ir.add_op(TestInstructionSet::IntInput { pos: 1 }, svec![]);
    // Inside group: crossed edges
    let (a, va) = ir.add_op(TestInstructionSet::Inc, svec![v1[0]]);
    let (b, vb) = ir.add_op(TestInstructionSet::Inc, svec![v0[0]]);
    let (c, vc) = ir.add_op(TestInstructionSet::Inc, svec![vb[0]]);
    let (d, vd) = ir.add_op(TestInstructionSet::Inc, svec![va[0]]);
    // Exit group
    let (m, vm) = ir.add_op(TestInstructionSet::Add, svec![vc[0], vd[0]]);
    let (r, _) = ir.add_op(TestInstructionSet::Return, svec![vm[0]]);

    let root = Hierarchy::new();
    let mut group = root.clone();
    group.push("inner");

    let mut op_annotations = ir.empty_opmap();
    op_annotations.insert(i0, root.clone());
    op_annotations.insert(i1, root.clone());
    for id in [a, b, c, d] {
        op_annotations.insert(id, group.clone());
    }
    op_annotations.insert(m, root.clone());
    op_annotations.insert(r, root.clone());
    draw_ir_html(&ir, op_annotations, "test32.html");
}

/// Cross-group edges that would cross if groups aren't reordered.
#[test]
fn test_cross_group_reorder() {
    let mut ir: IR<TestLang> = IR::empty();
    // Two inputs
    let (i0, v0) = ir.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![]);
    let (i1, v1) = ir.add_op(TestInstructionSet::IntInput { pos: 1 }, svec![]);
    // Group A processes i1, Group B processes i0 (crossed)
    let (a, va) = ir.add_op(TestInstructionSet::Inc, svec![v1[0]]);
    let (b, vb) = ir.add_op(TestInstructionSet::Inc, svec![v0[0]]);
    // Converge
    let (m, vm) = ir.add_op(TestInstructionSet::Add, svec![va[0], vb[0]]);
    let (r, _) = ir.add_op(TestInstructionSet::Return, svec![vm[0]]);

    let root = Hierarchy::new();
    let mut group_a = root.clone();
    group_a.push("group_a");
    let mut group_b = root.clone();
    group_b.push("group_b");

    let mut op_annotations = ir.empty_opmap();
    op_annotations.insert(i0, root.clone());
    op_annotations.insert(i1, root.clone());
    op_annotations.insert(a, group_a);
    op_annotations.insert(b, group_b);
    op_annotations.insert(m, root.clone());
    op_annotations.insert(r, root.clone());
    draw_ir_html(&ir, op_annotations, "test33.html");
}

/// Bug reproducer: GroupInput shares layer 1 with an IntInput inside the group.
/// This triggers the indexing bug in place_once_bottom_up (line 203) where
/// the layer position is used as an index into args_original_places.
///
/// Forces crossing: two external inputs A, B go into the group. An internal source C
/// also exists on layer 1. The median heuristic will reorder based on downstream users,
/// potentially placing C between the GroupInputs.
#[test]
fn test_mixed_first_layer_in_group() {
    // Structure:
    //   A, B (root) -> [group: gi_A, gi_B, internal_C (all layer 1) -> use them -> outputs] -> ret
    // With careful wiring, reordering can place internal_C at position 1, pushing gi_B to position
    // 2. args_original_places has 2 elements, so index 2 is out of bounds.
    let mut ir: IR<TestLang> = IR::empty();
    let (op_a, a) = ir.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![]);
    let (op_b, b) = ir.add_op(TestInstructionSet::IntInput { pos: 1 }, svec![]);
    // Inside group_a:
    let (op_c, c) = ir.add_op(TestInstructionSet::IntInput { pos: 2 }, svec![]); // internal source
    // Wire so that C is used first (lower position), then B, then A - forces reorder
    let (op_add1, add1) = ir.add_op(TestInstructionSet::Add, svec![c[0], b[0]]); // uses C and B
    let (op_add2, add2) = ir.add_op(TestInstructionSet::Add, svec![add1[0], a[0]]); // uses result and A
    // Back at root:
    let (op_ret, _) = ir.add_op(TestInstructionSet::Return, svec![add2[0]]);

    let root = Hierarchy::new();
    let mut group_a = root.clone();
    group_a.push("group_a");

    let mut op_annotations = ir.empty_opmap();
    op_annotations.insert(op_a, root.clone());
    op_annotations.insert(op_b, root.clone());
    op_annotations.insert(op_c, group_a.clone()); // internal source, layer 1
    op_annotations.insert(op_add1, group_a.clone());
    op_annotations.insert(op_add2, group_a.clone());
    op_annotations.insert(op_ret, root.clone());
    draw_ir_html(&ir, op_annotations, "test34.html");
}

/// Bug reproducer: GroupOutput shares last layer with a Return inside the group.
/// This triggers the indexing bug in place_once_top_down (line 100) where
/// the layer position is used as an index into rets_original_places.
///
/// Strategy: Single GroupOutput + Return on last layer. If Return ends up at
/// position 0 and GroupOutput at position 1, indexing with 1 into a 1-element
/// rets_original_places array causes out of bounds.
/// We use two inputs where the one feeding the internal Return has lower position.
#[test]
fn test_mixed_last_layer_in_group() {
    let mut ir: IR<TestLang> = IR::empty();
    // inp0 at position 0, inp1 at position 1
    let (op_inp0, inp0) = ir.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![]);
    let (op_inp1, inp1) = ir.add_op(TestInstructionSet::IntInput { pos: 1 }, svec![]);
    // Inside group_a:
    // inc1 from inp0 (position 0) -> feeds internal Return (will want position 0 on last layer)
    let (op_inc1, inc1) = ir.add_op(TestInstructionSet::Inc, svec![inp0[0]]);
    // inc2 from inp1 (position 1) -> exits group (will want position 1 on last layer)
    let (op_inc2, inc2) = ir.add_op(TestInstructionSet::Inc, svec![inp1[0]]);
    // Internal Return fed by inc1 - should get position 0 on last layer
    let (op_internal_ret, _) = ir.add_op(TestInstructionSet::Return, svec![inc1[0]]);
    // inc2 exits the group - GroupOutput should get position 1 on last layer
    // But rets_original_places has only 1 element!
    let (op_ret, _) = ir.add_op(TestInstructionSet::Return, svec![inc2[0]]);

    let root = Hierarchy::new();
    let mut group_a = root.clone();
    group_a.push("group_a");

    let mut op_annotations = ir.empty_opmap();
    op_annotations.insert(op_inp0, root.clone());
    op_annotations.insert(op_inp1, root.clone());
    op_annotations.insert(op_inc1, group_a.clone());
    op_annotations.insert(op_inc2, group_a.clone());
    op_annotations.insert(op_internal_ret, group_a.clone());
    op_annotations.insert(op_ret, root.clone());
    draw_ir_html(&ir, op_annotations, "test35.html");
}
