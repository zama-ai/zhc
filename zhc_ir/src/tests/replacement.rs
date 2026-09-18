//! Rewriting an IR in place: `replace_val_use`, `replace_val_use_at`, and `replace_op_instr`.

use crate::tests::testlang::{TestInstructionSet, TestLang};
use crate::{IR, ValUse};
use zhc_utils::{assert_display_is, iter::CollectInVec, svec};

/// Tests that replacing a value with one that would create a cycle panics
#[test]
#[should_panic(expected = "Tried to replace a value with one it reaches.")]
fn test_replace_val_use_wrong() {
    let mut store: IR<TestLang> = IR::empty();
    let (_, v0) = store.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![]);
    let (_, v1) = store.add_op(TestInstructionSet::Inc, svec![v0[0]]);
    assert_display_is!(
        store.format(),
        r#"
            %0 = int_input<pos: 0>();
            %1 = inc(%0);
        "#
    );
    store.replace_val_use(v0[0], v1[0]);
}

/// Tests that replacing a value with one that would create a cycle panics (longer chain)
#[test]
#[should_panic(expected = "Tried to replace a value with one it reaches.")]
fn test_replace_val_use_wrong_longer() {
    let mut store: IR<TestLang> = IR::empty();
    let (_, v0) = store.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![]);
    let (_, v1) = store.add_op(TestInstructionSet::Inc, svec![v0[0]]);
    assert_display_is!(
        store.format(),
        r#"
            %0 = int_input<pos: 0>();
            %1 = inc(%0);
        "#
    );
    store.replace_val_use(v0[0], v1[0]);
}

/// Tests successful value use replacement and user list updates
#[test]
fn test_replace_val_use() {
    let mut store: IR<TestLang> = IR::empty();
    let (_inp1_id, v0) = store.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![]);
    let (_inp2_id, v1) = store.add_op(TestInstructionSet::IntInput { pos: 1 }, svec![]);
    let (inc_id, _v2) = store.add_op(TestInstructionSet::Inc, svec![v0[0]]);
    assert_display_is!(
        store.format(),
        r#"
            %0 = int_input<pos: 0>();
            %1 = int_input<pos: 1>();
            %2 = inc(%0);
        "#
    );
    store.replace_val_use(v0[0], v1[0]);
    assert_display_is!(
        store.format(),
        r#"
            %0 = int_input<pos: 0>();
            %1 = int_input<pos: 1>();
            %2 = inc(%1);
        "#
    );
    let inc = store.get_op(inc_id);
    let v0 = store.get_val(v0[0]);
    let v1 = store.get_val(v1[0]);
    assert_eq!(v0.get_users_iter().covec(), []);
    assert_eq!(v1.get_users_iter().covec(), [inc.clone()]);
}

/// Tests that value replacement makes operation depth shallower when appropriate
#[test]
fn test_replace_val_use_make_shallower() {
    let mut store: IR<TestLang> = IR::empty();
    let (_inp1_id, v0) = store.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![]);
    let (_inp2_id, v1) = store.add_op(TestInstructionSet::IntInput { pos: 1 }, svec![]);
    let (_, v2) = store.add_op(TestInstructionSet::Inc, svec![v1[0]]);
    let (_, v3) = store.add_op(TestInstructionSet::Inc, svec![v2[0]]);
    let (_, v4) = store.add_op(TestInstructionSet::Inc, svec![v3[0]]);
    let (last_id, _v5) = store.add_op(TestInstructionSet::Inc, svec![v4[0]]);
    assert_display_is!(
        store.format(),
        r#"
            %0 = int_input<pos: 0>();
            %1 = int_input<pos: 1>();
            %2 = inc(%1);
            %3 = inc(%2);
            %4 = inc(%3);
            %5 = inc(%4);
        "#
    );
    let last = store.get_op(last_id);
    assert_eq!(last.get_depth(), 5);
    store.replace_val_use(v4[0], v0[0]);
    assert_display_is!(
        store.format(),
        r#"
            %0 = int_input<pos: 0>();
            %1 = int_input<pos: 1>();
            %2 = inc(%1);
            %3 = inc(%2);
            %4 = inc(%3);
            %5 = inc(%0);
        "#
    );
    let last = store.get_op(last_id);
    assert_eq!(last.get_depth(), 2);
}

/// Tests that value replacement makes operation depth deeper when appropriate
#[test]
fn test_replace_val_use_make_deeper() {
    let mut store: IR<TestLang> = IR::empty();
    let (_inp1_id, v0) = store.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![]);
    let (_inp2_id, v1) = store.add_op(TestInstructionSet::IntInput { pos: 1 }, svec![]);
    let (_, v2) = store.add_op(TestInstructionSet::Inc, svec![v1[0]]);
    let (_, v3) = store.add_op(TestInstructionSet::Inc, svec![v2[0]]);
    let (_, v4) = store.add_op(TestInstructionSet::Inc, svec![v0[0]]);
    let (last_id, _v5) = store.add_op(TestInstructionSet::Inc, svec![v4[0]]);
    assert_display_is!(
        store.format(),
        r#"
            %0 = int_input<pos: 0>();
            %1 = int_input<pos: 1>();
            %2 = inc(%1);
            %3 = inc(%2);
            %4 = inc(%0);
            %5 = inc(%4);
        "#
    );
    let last = store.get_op(last_id);
    assert_eq!(last.get_depth(), 3);
    store.replace_val_use(v0[0], v3[0]);
    assert_display_is!(
        store.format(),
        r#"
            %0 = int_input<pos: 0>();
            %1 = int_input<pos: 1>();
            %2 = inc(%1);
            %3 = inc(%2);
            %4 = inc(%3);
            %5 = inc(%4);
        "#
    );
    let last = store.get_op(last_id);
    assert_eq!(last.get_depth(), 5);
}

/// Tests self-value replacement (should be no-op)
#[test]
fn test_replace_val_use_self() {
    let mut store: IR<TestLang> = IR::empty();
    let (_, vals) = store.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![]);
    let (inc_id, _) = store.add_op(TestInstructionSet::Inc, svec![vals[0]]);

    let inc_before = format!("{:?}", store.get_op(inc_id));
    store.replace_val_use(vals[0], vals[0]); // Should be no-op
    let inc_after = format!("{:?}", store.get_op(inc_id));

    assert_eq!(inc_before, inc_after);
    assert_display_is!(
        store.format(),
        r#"
            %0 = int_input<pos: 0>();
            %1 = inc(%0);
        "#
    );
}

/// Tests replacement cascade affecting multiple dependency levels
#[test]
fn test_replacement_cascade_multiple_levels() {
    let mut store: IR<TestLang> = IR::empty();

    // Create: inp1 → inc1 → inc2 → inc3
    //         inp2 (unused initially)
    let (_, inp1) = store.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![]);
    let (_, inp2) = store.add_op(TestInstructionSet::IntInput { pos: 1 }, svec![]);
    let (inc1_id, inc1_vals) = store.add_op(TestInstructionSet::Inc, svec![inp1[0]]);
    let (inc2_id, inc2_vals) = store.add_op(TestInstructionSet::Inc, svec![inc1_vals[0]]);
    let (inc3_id, _) = store.add_op(TestInstructionSet::Inc, svec![inc2_vals[0]]);

    let depths_before = [
        store.get_op(inc1_id).get_depth(),
        store.get_op(inc2_id).get_depth(),
        store.get_op(inc3_id).get_depth(),
    ];

    // Replace inc1's input with inp2, should cascade depth updates
    store.replace_val_use(inp1[0], inp2[0]);

    let depths_after = [
        store.get_op(inc1_id).get_depth(),
        store.get_op(inc2_id).get_depth(),
        store.get_op(inc3_id).get_depth(),
    ];

    // Depths should remain the same since both inputs are at depth 1
    assert_eq!(depths_before, depths_after);
}

/// Tests replacement creating deeper dependency chain
#[test]
fn test_replacement_deeper_chain() {
    let mut store: IR<TestLang> = IR::empty();

    // Create: inp1, inp2 → inc1, inp2 → inc2
    let (_, inp1) = store.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![]);
    let (_, inp2) = store.add_op(TestInstructionSet::IntInput { pos: 1 }, svec![]);
    let (_, inc1_vals) = store.add_op(TestInstructionSet::Inc, svec![inp2[0]]);
    let (inc2_id, _) = store.add_op(TestInstructionSet::Inc, svec![inp1[0]]); // Initially uses inp1

    assert_eq!(store.get_op(inc2_id).get_depth(), 2);

    // Replace inp1 with inc1's output, making inc2 deeper
    store.replace_val_use(inp1[0], inc1_vals[0]);

    assert_eq!(store.get_op(inc2_id).get_depth(), 3); // Now inp2→inc1→inc2
}

/// Tests replacement with type validation
#[test]
#[should_panic(expected = "Tried to replace a value with one of different type")]
fn test_replace_val_different_types() {
    let mut store: IR<TestLang> = IR::empty();
    let (_, int_vals) = store.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![]);
    let (_, bool_vals) = store.add_op(TestInstructionSet::BoolConstant { val: true }, svec![]);

    // Try to replace int value with bool value
    store.replace_val_use(int_vals[0], bool_vals[0]);
}

/// Tests operations that become unreachable after replacement but aren't deleted
#[test]
fn test_unreachable_after_replacement() {
    let mut store: IR<TestLang> = IR::empty();
    let (_, inp1) = store.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![]);
    let (_, inp2) = store.add_op(TestInstructionSet::IntInput { pos: 1 }, svec![]);
    let (inc1_id, inc1_vals) = store.add_op(TestInstructionSet::Inc, svec![inp1[0]]);
    let (_ret_id, _) = store.add_op(TestInstructionSet::Return, svec![inc1_vals[0]]);

    // Replace return's input with inp2, making inc1 unreachable
    store.replace_val_use(inc1_vals[0], inp2[0]);

    // inc1 should still exist and be active, but have no users
    assert!(store.has_opid(inc1_id));
    assert_eq!(store.get_val(inc1_vals[0]).get_users_iter().count(), 0);

    // Should be able to delete it now
    store.delete_op(inc1_id);
    assert!(!store.has_opid(inc1_id));
}

/// Tests that replace_val_use_at rewrites only the targeted use site,
/// leaving other uses of the same value untouched.
#[test]
fn test_replace_val_use_at_selective() {
    let mut store: IR<TestLang> = IR::empty();
    let (_, v0) = store.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![]);
    let (_, v1) = store.add_op(TestInstructionSet::IntInput { pos: 1 }, svec![]);
    // add_op1 uses v0 at positions 0 and 1
    let (add1_id, _) = store.add_op(TestInstructionSet::Add, svec![v0[0], v0[0]]);
    // add_op2 also uses v0
    let (add2_id, _) = store.add_op(TestInstructionSet::Add, svec![v0[0], v0[0]]);
    assert_display_is!(
        store.format(),
        r#"
            %0 = int_input<pos: 0>();
            %1 = int_input<pos: 1>();
            %2 = add(%0, %0);
            %3 = add(%0, %0);
        "#
    );
    // Replace only position 0 of add1_id with v1
    store.replace_val_use_at(
        ValUse {
            opid: add1_id,
            position: 0,
        },
        v1[0],
    );
    assert_display_is!(
        store.format(),
        r#"
            %0 = int_input<pos: 0>();
            %1 = int_input<pos: 1>();
            %2 = add(%1, %0);
            %3 = add(%0, %0);
        "#
    );
    // v0 still used 3 times (pos 1 of add1, pos 0+1 of add2); v1 used once
    let val0 = store.get_val(v0[0]);
    let val1 = store.get_val(v1[0]);
    assert_eq!(val0.users.len(), 3);
    assert_eq!(val1.users.len(), 1);
    assert_eq!(val1.users[0].opid, add1_id);
    assert_eq!(val1.users[0].position, 0);
    // position 1 of add1 still uses v0
    assert!(
        val0.users
            .iter()
            .any(|u| u.opid == add1_id && u.position == 1)
    );
    // add2 still uses v0 at both positions
    assert!(
        val0.users
            .iter()
            .any(|u| u.opid == add2_id && u.position == 0)
    );
    assert!(
        val0.users
            .iter()
            .any(|u| u.opid == add2_id && u.position == 1)
    );
}

/// Tests that replace_val_use_at with a same-value replacement is a no-op.
#[test]
fn test_replace_val_use_at_noop_same_value() {
    let mut store: IR<TestLang> = IR::empty();
    let (_, v0) = store.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![]);
    let (inc_id, _) = store.add_op(TestInstructionSet::Inc, svec![v0[0]]);
    assert_display_is!(
        store.format(),
        r#"
            %0 = int_input<pos: 0>();
            %1 = inc(%0);
        "#
    );
    let users_before: Vec<_> = { store.get_val(v0[0]).users.to_vec() };
    store.replace_val_use_at(
        ValUse {
            opid: inc_id,
            position: 0,
        },
        v0[0],
    );
    assert_display_is!(
        store.format(),
        r#"
            %0 = int_input<pos: 0>();
            %1 = inc(%0);
        "#
    );
    let users_after: Vec<_> = store.get_val(v0[0]).users.to_vec();
    assert_eq!(users_before, users_after);
}

/// Tests that replace_val_use_at updates depth correctly.
#[test]
fn test_replace_val_use_at_depth_update() {
    let mut store: IR<TestLang> = IR::empty();
    let (_, v0) = store.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![]);
    let (_, v1) = store.add_op(TestInstructionSet::IntInput { pos: 1 }, svec![]);
    let (_, v2) = store.add_op(TestInstructionSet::Inc, svec![v1[0]]);
    let (_, v3) = store.add_op(TestInstructionSet::Inc, svec![v2[0]]);
    // last uses v0 (depth 1), replaced with v3 whose producer is at depth 3 → new depth = 4
    let (last_id, _) = store.add_op(TestInstructionSet::Inc, svec![v0[0]]);
    assert_display_is!(
        store.format(),
        r#"
            %0 = int_input<pos: 0>();
            %1 = int_input<pos: 1>();
            %2 = inc(%1);
            %3 = inc(%2);
            %4 = inc(%0);
        "#
    );
    assert_eq!(store.get_op(last_id).get_depth(), 2);
    store.replace_val_use_at(
        ValUse {
            opid: last_id,
            position: 0,
        },
        v3[0],
    );
    assert_display_is!(
        store.format(),
        r#"
            %0 = int_input<pos: 0>();
            %1 = int_input<pos: 1>();
            %2 = inc(%1);
            %3 = inc(%2);
            %4 = inc(%3);
        "#
    );
    assert_eq!(store.get_op(last_id).get_depth(), 4);
}

/// Tests that replace_val_use_at panics on an out-of-bounds position.
#[test]
#[should_panic(expected = "Invalid use_site.")]
fn test_replace_val_use_at_invalid_position() {
    let mut store: IR<TestLang> = IR::empty();
    let (_, v0) = store.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![]);
    let (_, v1) = store.add_op(TestInstructionSet::IntInput { pos: 1 }, svec![]);
    let (inc_id, _) = store.add_op(TestInstructionSet::Inc, svec![v0[0]]);
    // Inc takes 1 argument; position 1 is out of bounds
    store.replace_val_use_at(
        ValUse {
            opid: inc_id,
            position: 1,
        },
        v1[0],
    );
}

/// Tests that replace_val_use_at panics on a type mismatch.
#[test]
#[should_panic(expected = "Tried to replace a value with one of different type.")]
fn test_replace_val_use_at_type_mismatch() {
    let mut store: IR<TestLang> = IR::empty();
    let (_, v_int) = store.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![]);
    let (_, v_bool) = store.add_op(TestInstructionSet::BoolConstant { val: true }, svec![]);
    let (inc_id, _) = store.add_op(TestInstructionSet::Inc, svec![v_int[0]]);
    store.replace_val_use_at(
        ValUse {
            opid: inc_id,
            position: 0,
        },
        v_bool[0],
    );
}

/// Tests that replace_val_use_at panics when the replacement would create a cycle.
/// Direct self-reference: inc(%0) tries to consume its own output.
#[test]
#[should_panic(expected = "Tried to replace a value with one it reaches.")]
fn test_replace_val_use_at_cycle() {
    let mut store: IR<TestLang> = IR::empty();
    let (_, v0) = store.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![]);
    let (inc_id, v1) = store.add_op(TestInstructionSet::Inc, svec![v0[0]]);
    // Replacing the argument of inc_id with its own output creates a self-cycle
    store.replace_val_use_at(
        ValUse {
            opid: inc_id,
            position: 0,
        },
        v1[0],
    );
}

/// Tests that replace_val_use_at use-def chains remain consistent for multi-return ops.
#[test]
fn test_replace_val_use_at_multi_return_arg() {
    let mut store: IR<TestLang> = IR::empty();
    let (_, v0) = store.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![]);
    let (_, v1) = store.add_op(TestInstructionSet::IntInput { pos: 1 }, svec![]);
    let (_, v2) = store.add_op(TestInstructionSet::IntInput { pos: 2 }, svec![]);
    // DivRem returns two values; use both in an Add
    let (_, v_dr) = store.add_op(TestInstructionSet::DivRem, svec![v0[0], v1[0]]);
    let (add_id, _) = store.add_op(TestInstructionSet::Add, svec![v_dr[0], v_dr[1]]);
    assert_display_is!(
        store.format(),
        r#"
            %0 = int_input<pos: 0>();
            %1 = int_input<pos: 1>();
            %2 = int_input<pos: 2>();
            %3, %4 = div_rem(%0, %1);
            %5 = add(%3, %4);
        "#
    );
    // Replace the second argument of add (v_dr[1]) with v2[0]
    store.replace_val_use_at(
        ValUse {
            opid: add_id,
            position: 1,
        },
        v2[0],
    );
    assert_display_is!(
        store.format(),
        r#"
            %0 = int_input<pos: 0>();
            %1 = int_input<pos: 1>();
            %2 = int_input<pos: 2>();
            %3, %4 = div_rem(%0, %1);
            %5 = add(%3, %2);
        "#
    );
    // v_dr[1] should have no users; v2[0] should be used at position 1 of add
    let vdr1 = store.get_val(v_dr[1]);
    let vdr0 = store.get_val(v_dr[0]);
    let v2_val = store.get_val(v2[0]);
    assert_eq!(vdr1.users.len(), 0);
    assert_eq!(vdr0.users.len(), 1);
    assert_eq!(vdr0.users[0].opid, add_id);
    assert_eq!(vdr0.users[0].position, 0);
    assert_eq!(v2_val.users.len(), 1);
    assert_eq!(v2_val.users[0].opid, add_id);
    assert_eq!(v2_val.users[0].position, 1);
}

/// A `ValUse` pointing at a deleted operation must be rejected. Otherwise the dead op's
/// arguments get rewritten, its depth is recomputed, and a use coming from a dead op is
/// pushed onto the users of `new`.
#[test]
#[should_panic(expected = "Unknown or inactive op.")]
fn test_replace_val_use_at_dead_op() {
    let mut store: IR<TestLang> = IR::empty();
    let (_, v0) = store.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![]);
    let (_, v1) = store.add_op(TestInstructionSet::IntInput { pos: 1 }, svec![]);
    let (inc_id, _) = store.add_op(TestInstructionSet::Inc, svec![v0[0]]);
    store.delete_op(inc_id);
    store.replace_val_use_at(
        ValUse {
            opid: inc_id,
            position: 0,
        },
        v1[0],
    );
}

/// In-place instruction mutation must not change the signature. The stored signature and the
/// types of the return values are derived from the instruction at insertion and are never
/// re-validated.
#[test]
#[should_panic(expected = "signature")]
fn test_replace_op_instr_signature_drift() {
    let mut store: IR<TestLang> = IR::empty();
    let (_, v0) = store.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![]);
    let _ = store.add_op(TestInstructionSet::Inc, svec![v0[0]]);
    // Inc is Int -> Int, Return is Int -> (). Same arity, different signature.
    store.replace_ops_instr_linear(|op| match op.get_instruction() {
        TestInstructionSet::Inc => TestInstructionSet::Return,
        other => other.clone(),
    });
}
