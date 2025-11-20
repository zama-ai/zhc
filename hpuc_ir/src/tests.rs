#[allow(unused)]
pub mod test_dialect {
    use std::fmt::Display;

    use crate::{Dialect, DialectOperations, DialectTypes, signature::Signature};

    use hpuc_utils::svec;

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub enum Types {
        Int,
        Bool,
    }
    impl Display for Types {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Types::Int => write!(f, "Int"),
                Types::Bool => write!(f, "Bool"),
            }
        }
    }
    impl DialectTypes for Types {}

    #[derive(Debug, Clone, Hash, PartialEq, Eq)]
    pub enum Operations {
        IntInput { pos: usize },
        BoolConstant { val: bool },
        Add,
        IfElse,
        DivRem,
        Inc,
        Return,
    }

    impl Display for Operations {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Operations::IntInput { pos } => write!(f, "int_input<pos: {}>", pos),
                Operations::BoolConstant { val } => write!(f, "bool_constant<val: {}>", val),
                Operations::Add => write!(f, "add"),
                Operations::IfElse => write!(f, "if_else"),
                Operations::DivRem => write!(f, "div_rem"),
                Operations::Inc => write!(f, "inc"),
                Operations::Return => write!(f, "return"),
            }
        }
    }

    impl DialectOperations for Operations {
        type Types = Types;

        fn get_signature(&self) -> crate::signature::Signature<Self::Types> {
            use Types::*;
            match self {
                Operations::IntInput { .. } => Signature(svec![], svec![Int]),
                Operations::BoolConstant { .. } => Signature(svec![], svec![Bool]),
                Operations::Add => Signature(svec![Int, Int], svec![Int]),
                Operations::IfElse => Signature(svec![Int, Bool, Int], svec![Int]),
                Operations::DivRem => Signature(svec![Int, Int], svec![Int, Int]),
                Operations::Inc => Signature(svec![Int], svec![Int]),
                Operations::Return => Signature(svec![Int], svec![]),
            }
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct TestDialect;

    impl Dialect for TestDialect {
        type Types = Types;
        type Operations = Operations;
    }
}

use crate::{DialectOperations, IR, IRError};
use hpuc_utils::{CollectInVec, svec};

use test_dialect::{Operations, TestDialect};

/// Tests basic IR construction with complex operation graph and validates
/// all operation properties, value relationships, and depth calculations
#[test]
fn test_construction() -> Result<(), IRError<TestDialect>> {
    let mut store: IR<TestDialect> = IR::empty();

    let (lhs_id, v0) = store.add_op(Operations::IntInput { pos: 0 }, svec![])?;
    let (rhs_id, v1) = store.add_op(Operations::IntInput { pos: 1 }, svec![])?;
    let (join_id, v2) = store.add_op(Operations::Add, svec![v0[0], v1[0]])?;
    let (split_id, v3) = store.add_op(Operations::DivRem, svec![v2[0], v0[0]])?;
    let (ulhs_id, v4) = store.add_op(Operations::Inc, svec![v3[0]])?;
    let (urhs_id, v5) = store.add_op(Operations::Inc, svec![v3[1]])?;
    let (final_add_id, v6) = store.add_op(Operations::Add, svec![v4[0], v5[0]])?;
    let (effect_id, _) = store.add_op(Operations::Return, svec![v3[0]])?;

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

    store.check_ir(
        "
            %0 : Int = int_input<pos: 0>();
            %1 : Int = int_input<pos: 1>();
            %2 : Int = add(%0, %1);
            %3 : Int, %4 : Int = div_rem(%2, %0);
            %5 : Int = inc(%3);
            %6 : Int = inc(%4);
            return(%3);
            %7 : Int = add(%5, %6);
            ",
    );

    assert_eq!(store.n_ops(), 8);
    assert_eq!(store.n_vals(), 8);
    assert!(lhs.is_active());
    assert_eq!(lhs.get_depth(), 1);
    assert_eq!(lhs.get_args_iter().covect(), []);
    assert_eq!(lhs.get_returns_iter().covect(), [p0.clone()]);

    assert!(rhs.is_active());
    assert_eq!(rhs.get_depth(), 1);
    assert_eq!(rhs.get_args_iter().covect(), []);
    assert_eq!(rhs.get_returns_iter().covect(), [p1.clone()]);

    assert!(p0.is_active());
    assert_eq!(p0.get_origin(), lhs);
    assert_eq!(p0.get_users_iter().covect(), [join.clone(), split.clone()]);

    assert!(p1.is_active());
    assert_eq!(p1.get_origin(), rhs);
    assert_eq!(p1.get_users_iter().covect(), [join.clone()]);

    assert!(join.is_active());
    assert_eq!(join.get_depth(), 2);
    assert_eq!(join.get_args_iter().covect(), [p0.clone(), p1.clone()]);
    assert_eq!(join.get_returns_iter().covect(), [p2.clone()]);

    assert!(p2.is_active());
    assert_eq!(p2.get_origin(), join);
    assert_eq!(p2.get_users_iter().covect(), [split.clone()]);

    assert!(split.is_active());
    assert_eq!(split.get_depth(), 3);
    assert_eq!(split.get_args_iter().covect(), [p2.clone(), p0.clone()]);
    assert_eq!(split.get_returns_iter().covect(), [p3.clone(), p4.clone()]);

    assert!(p3.is_active());
    assert_eq!(p3.get_origin(), split);
    assert_eq!(p3.get_users_iter().covect(), [ulhs.clone(), effect.clone()]);

    assert!(p4.is_active());
    assert_eq!(p4.get_origin(), split);
    assert_eq!(p4.get_users_iter().covect(), [urhs.clone()]);

    assert!(ulhs.is_active());
    assert_eq!(ulhs.get_depth(), 4);
    assert_eq!(ulhs.get_args_iter().covect(), [p3.clone()]);
    assert_eq!(ulhs.get_returns_iter().covect(), [p5.clone()]);

    assert!(p5.is_active());
    assert_eq!(p5.get_origin(), ulhs);
    assert_eq!(p5.get_users_iter().covect(), [final_add.clone()]);

    assert!(urhs.is_active());
    assert_eq!(urhs.get_depth(), 4);
    assert_eq!(urhs.get_args_iter().covect(), [p4.clone()]);
    assert_eq!(urhs.get_returns_iter().covect(), [p6.clone()]);

    assert!(p6.is_active());
    assert_eq!(p6.get_origin(), urhs);
    assert_eq!(p6.get_users_iter().covect(), [final_add.clone()]);

    assert!(final_add.is_active());
    assert_eq!(final_add.get_depth(), 5);
    assert_eq!(final_add.get_args_iter().covect(), [p5.clone(), p6.clone()]);
    assert_eq!(final_add.get_returns_iter().covect(), [p7.clone()]);

    assert!(p7.is_active());
    assert_eq!(p7.get_origin(), final_add);
    assert_eq!(p7.get_users_iter().covect(), []);

    assert!(effect.is_active());
    assert_eq!(effect.get_depth(), 4);
    assert_eq!(effect.get_args_iter().covect(), [p3.clone()]);
    assert_eq!(effect.get_returns_iter().covect(), []);
    Ok(())
}

/// Tests that an operation reaches itself (reflexive property)
#[test]
fn test_reaches_self() -> Result<(), IRError<TestDialect>> {
    let mut store: IR<TestDialect> = IR::empty();
    let (lhs_id, _) = store.add_op(Operations::IntInput { pos: 0 }, svec![])?;
    let lhs = store.get_op(lhs_id);
    store.check_ir(
        "
            %0 : Int = int_input<pos: 0>();
            ",
    );
    assert!(lhs.reaches(lhs.clone()));
    Ok(())
}

/// Tests basic reachability between two operations in a dependency chain
#[test]
fn test_reaches_base() -> Result<(), IRError<TestDialect>> {
    let mut store: IR<TestDialect> = IR::empty();
    let (lhs_id, v0) = store.add_op(Operations::IntInput { pos: 0 }, svec![])?;
    let (ulhs_id, _) = store.add_op(Operations::Inc, svec![v0[0]])?;
    let lhs = store.get_op(lhs_id);
    let ulhs = store.get_op(ulhs_id);
    store.check_ir(
        "
            %0 : Int = int_input<pos: 0>();
            %1 : Int = inc(%0);
            ",
    );
    assert!(lhs.reaches(ulhs));
    Ok(())
}

/// Tests reachability through a longer chain of operations
#[test]
fn test_reaches_chain() -> Result<(), IRError<TestDialect>> {
    let mut store: IR<TestDialect> = IR::empty();
    let (lhs_id, v0) = store.add_op(Operations::IntInput { pos: 0 }, svec![])?;
    let (_, v1) = store.add_op(Operations::Inc, svec![v0[0]])?;
    let (_, v2) = store.add_op(Operations::Inc, svec![v1[0]])?;
    let (_, v3) = store.add_op(Operations::Inc, svec![v2[0]])?;
    let (_, v4) = store.add_op(Operations::Inc, svec![v3[0]])?;
    let (ulhs_id, _) = store.add_op(Operations::Inc, svec![v4[0]])?;
    let lhs = store.get_op(lhs_id);
    let ulhs = store.get_op(ulhs_id);
    store.check_ir(
        "
            %0 : Int = int_input<pos: 0>();
            %1 : Int = inc(%0);
            %2 : Int = inc(%1);
            %3 : Int = inc(%2);
            %4 : Int = inc(%3);
            %5 : Int = inc(%4);
            ",
    );
    assert!(lhs.reaches(ulhs));
    Ok(())
}

/// Tests that independent operations don't reach each other
#[test]
fn test_reaches_happy_path() -> Result<(), IRError<TestDialect>> {
    let mut store: IR<TestDialect> = IR::empty();
    let (lhs_id, v0) = store.add_op(Operations::IntInput { pos: 0 }, svec![])?;
    let (rhs_id, _) = store.add_op(Operations::IntInput { pos: 1 }, svec![])?;
    let _ = store.add_op(Operations::Inc, svec![v0[0]])?;
    let lhs = store.get_op(lhs_id);
    let rhs = store.get_op(rhs_id);
    store.check_ir(
        "
            %0 : Int = int_input<pos: 0>();
            %1 : Int = int_input<pos: 1>();
            %2 : Int = inc(%0);
            ",
    );
    assert!(!lhs.reaches(rhs));
    Ok(())
}

/// Tests that attempting to delete an operation still in use panics
#[test]
#[should_panic]
fn test_delete_op_in_use() {
    let mut store: IR<TestDialect> = IR::empty();
    let (lhs_id, v0) = store
        .add_op(Operations::IntInput { pos: 0 }, svec![])
        .expect("Bad add_op");
    let (_, v1) = store
        .add_op(Operations::IntInput { pos: 1 }, svec![])
        .expect("Bad add_op");
    let _ = store
        .add_op(Operations::Add, svec![v0[0], v1[0]])
        .expect("Bad add_op");
    store.delete_op(lhs_id);
}

/// Tests successful deletion of operations and verification of inactive state
#[test]
fn test_delete_op() -> Result<(), IRError<TestDialect>> {
    let mut store: IR<TestDialect> = IR::empty();
    let (_, v0) = store.add_op(Operations::IntInput { pos: 0 }, svec![])?;
    let (rhs_id, v1) = store.add_op(Operations::IntInput { pos: 1 }, svec![])?;
    let (join_id, v2) = store.add_op(Operations::Add, svec![v0[0], v1[0]])?;
    store.delete_op(join_id);
    store.delete_op(rhs_id);
    store.check_ir(
        "
            %0 : Int = int_input<pos: 0>();
            // %_1 : Int = int_input<pos: 1>();
            // %_2 : Int = add(%0, %_1);
            ",
    );
    assert!(store.raw_get_val(v2[0]).is_inactive());
    assert!(store.raw_get_val(v1[0]).is_inactive());
    assert!(store.raw_get_op(join_id).is_inactive());
    assert!(store.raw_get_op(rhs_id).is_inactive());
    Ok(())
}

/// Tests that replacing a value with one that would create a cycle panics
#[test]
#[should_panic(expected = "Tried to replace a value with one it reaches.")]
fn test_replace_val_use_wrong() {
    let mut store: IR<TestDialect> = IR::empty();
    let (_, v0) = store
        .add_op(Operations::IntInput { pos: 0 }, svec![])
        .expect("Bad add_op");
    let (_, v1) = store
        .add_op(Operations::Inc, svec![v0[0]])
        .expect("Bad add_op");
    store.check_ir(
        "
            %0 : Int = int_input<pos: 0>();
            %1 : Int = inc(%0);
            ",
    );
    store.replace_val_use(v0[0], v1[0]);
}

/// Tests that replacing a value with one that would create a cycle panics (longer chain)
#[test]
#[should_panic(expected = "Tried to replace a value with one it reaches.")]
fn test_replace_val_use_wrong_longer() {
    let mut store: IR<TestDialect> = IR::empty();
    let (_, v0) = store
        .add_op(Operations::IntInput { pos: 0 }, svec![])
        .expect("Bad add_op");
    let (_, v1) = store
        .add_op(Operations::Inc, svec![v0[0]])
        .expect("Bad add_op");
    store.check_ir(
        "
            %0 : Int = int_input<pos: 0>();
            %1 : Int = inc(%0);
            ",
    );
    store.replace_val_use(v0[0], v1[0]);
}

/// Tests successful value use replacement and user list updates
#[test]
fn test_replace_val_use() -> Result<(), IRError<TestDialect>> {
    let mut store: IR<TestDialect> = IR::empty();
    let (_inp1_id, v0) = store.add_op(Operations::IntInput { pos: 0 }, svec![])?;
    let (_inp2_id, v1) = store.add_op(Operations::IntInput { pos: 1 }, svec![])?;
    let (inc_id, _v2) = store.add_op(Operations::Inc, svec![v0[0]])?;
    store.check_ir(
        "
            %0 : Int = int_input<pos: 0>();
            %1 : Int = int_input<pos: 1>();
            %2 : Int = inc(%0);
            ",
    );
    store.replace_val_use(v0[0], v1[0]);
    store.check_ir(
        "
            %0 : Int = int_input<pos: 0>();
            %1 : Int = int_input<pos: 1>();
            %2 : Int = inc(%1);
            ",
    );
    let inc = store.get_op(inc_id);
    let v0 = store.get_val(v0[0]);
    let v1 = store.get_val(v1[0]);
    assert_eq!(v0.get_users_iter().covect(), []);
    assert_eq!(v1.get_users_iter().covect(), [inc.clone()]);
    Ok(())
}

/// Tests that value replacement makes operation depth shallower when appropriate
#[test]
fn test_replace_val_use_make_shallower() -> Result<(), IRError<TestDialect>> {
    let mut store: IR<TestDialect> = IR::empty();
    let (_inp1_id, v0) = store.add_op(Operations::IntInput { pos: 0 }, svec![])?;
    let (_inp2_id, v1) = store.add_op(Operations::IntInput { pos: 1 }, svec![])?;
    let (_, v2) = store.add_op(Operations::Inc, svec![v1[0]])?;
    let (_, v3) = store.add_op(Operations::Inc, svec![v2[0]])?;
    let (_, v4) = store.add_op(Operations::Inc, svec![v3[0]])?;
    let (last_id, _v5) = store.add_op(Operations::Inc, svec![v4[0]])?;
    store.check_ir(
        "
            %0 : Int = int_input<pos: 0>();
            %1 : Int = int_input<pos: 1>();
            %2 : Int = inc(%1);
            %3 : Int = inc(%2);
            %4 : Int = inc(%3);
            %5 : Int = inc(%4);
            ",
    );
    let last = store.get_op(last_id);
    assert_eq!(last.get_depth(), 5);
    store.replace_val_use(v4[0], v0[0]);
    store.check_ir(
        "
            %0 : Int = int_input<pos: 0>();
            %1 : Int = int_input<pos: 1>();
            %2 : Int = inc(%1);
            %3 : Int = inc(%0);
            %4 : Int = inc(%2);
            %5 : Int = inc(%4);
            ",
    );
    let last = store.get_op(last_id);
    assert_eq!(last.get_depth(), 2);
    Ok(())
}

/// Tests that value replacement makes operation depth deeper when appropriate
#[test]
fn test_replace_val_use_make_deeper() -> Result<(), IRError<TestDialect>> {
    let mut store: IR<TestDialect> = IR::empty();
    let (_inp1_id, v0) = store.add_op(Operations::IntInput { pos: 0 }, svec![])?;
    let (_inp2_id, v1) = store.add_op(Operations::IntInput { pos: 1 }, svec![])?;
    let (_, v2) = store.add_op(Operations::Inc, svec![v1[0]])?;
    let (_, v3) = store.add_op(Operations::Inc, svec![v2[0]])?;
    let (_, v4) = store.add_op(Operations::Inc, svec![v0[0]])?;
    let (last_id, _v5) = store.add_op(Operations::Inc, svec![v4[0]])?;
    store.check_ir(
        "
            %0 : Int = int_input<pos: 0>();
            %1 : Int = int_input<pos: 1>();
            %2 : Int = inc(%1);
            %3 : Int = inc(%0);
            %4 : Int = inc(%2);
            %5 : Int = inc(%3);
            ",
    );
    let last = store.get_op(last_id);
    assert_eq!(last.get_depth(), 3);
    store.replace_val_use(v0[0], v3[0]);
    store.check_ir(
        "
            %0 : Int = int_input<pos: 0>();
            %1 : Int = int_input<pos: 1>();
            %2 : Int = inc(%1);
            %3 : Int = inc(%2);
            %4 : Int = inc(%3);
            %5 : Int = inc(%4);
            ",
    );
    let last = store.get_op(last_id);
    assert_eq!(last.get_depth(), 5);
    Ok(())
}

/// Tests that add_op panics when argument types don't match operation signature
#[test]
fn test_add_op_type_mismatch() -> Result<(), IRError<TestDialect>> {
    let mut store: IR<TestDialect> = IR::empty();
    let (_, bool_val) = store.add_op(Operations::BoolConstant { val: true }, svec![])?;

    // Try to use bool value where int is expected
    let ret = store.add_op(Operations::Inc, svec![bool_val[0]]);

    assert_eq!(
        std::mem::discriminant(&ret.err().expect("must return an error")),
        std::mem::discriminant(&IRError::<TestDialect>::OpSig {
            op: Operations::Add,
            recv: vec![],
            exp: vec![]
        })
    );
    Ok(())
}

/// Tests that add_op return error when wrong number of arguments provided
#[test]
fn test_add_op_wrong_arg_count() -> Result<(), IRError<TestDialect>> {
    let mut store: IR<TestDialect> = IR::empty();
    let (_, int_vals) = store.add_op(Operations::IntInput { pos: 0 }, svec![])?;
    // Add operation expects 2 args, providing only 1
    let ret = store.add_op(Operations::Add, svec![int_vals[0]]);
    assert_eq!(
        std::mem::discriminant(&ret.err().expect("must return an error")),
        std::mem::discriminant(&IRError::<TestDialect>::OpSig {
            op: Operations::Add,
            recv: vec![],
            exp: vec![]
        })
    );
    Ok(())
}

/// Tests that using inactive ValId in add_op panics
#[test]
#[should_panic(expected = "Unknown valid")]
fn test_add_op_with_deleted_value() {
    let mut store: IR<TestDialect> = IR::empty();
    let (op_id, vals) = store
        .add_op(Operations::IntInput { pos: 0 }, svec![])
        .expect("Bad add_op");
    store.delete_op(op_id);
    // Try to use deleted value
    store
        .add_op(Operations::Inc, svec![vals[0]])
        .expect("Bad add_op");
}

/// Tests that accessing deleted operation via public API panics
#[test]
#[should_panic(expected = "Tried to get a dead op")]
fn test_get_deleted_op() {
    let mut store: IR<TestDialect> = IR::empty();
    let (op_id, _) = store
        .add_op(Operations::IntInput { pos: 0 }, svec![])
        .expect("Bad add_op");
    store.delete_op(op_id);
    store.get_op(op_id);
}

/// Tests that accessing deleted value via public API panics
#[test]
#[should_panic(expected = "Tried to get a dead val")]
fn test_get_deleted_val() {
    let mut store: IR<TestDialect> = IR::empty();
    let (op_id, vals) = store
        .add_op(Operations::IntInput { pos: 0 }, svec![])
        .expect("Bad add_op");
    store.delete_op(op_id);
    store.get_val(vals[0]);
}

/// Tests that double deletion is captured
#[test]
#[should_panic(expected = "Tried to delete an already inactive operation")]
fn test_double_deletion() {
    let mut store: IR<TestDialect> = IR::empty();
    let (op_id, _) = store
        .add_op(Operations::IntInput { pos: 0 }, svec![])
        .expect("Bad add_op");
    store.delete_op(op_id);
    store.delete_op(op_id); // Should panic on second deletion
}

/// Tests depth overflow protection with very deep chains
#[test]
#[should_panic(expected = "Overflow occured while computing the depth")]
fn test_depth_overflow() {
    let mut store: IR<TestDialect> = IR::empty();
    let (_, mut vals) = store
        .add_op(Operations::IntInput { pos: 0 }, svec![])
        .expect("Bad add_op");

    // Create a chain that would cause depth overflow
    for _ in 0..=u8::MAX {
        let (_, new_vals) = store
            .add_op(Operations::Inc, svec![vals[0]])
            .expect("Bad add_op");
        vals = new_vals;
    }
}

/// Tests self-value replacement (should be no-op)
#[test]
fn test_replace_val_use_self() -> Result<(), IRError<TestDialect>> {
    let mut store: IR<TestDialect> = IR::empty();
    let (_, vals) = store.add_op(Operations::IntInput { pos: 0 }, svec![])?;
    let (inc_id, _) = store.add_op(Operations::Inc, svec![vals[0]])?;

    let inc_before = format!("{:?}", store.get_op(inc_id));
    store.replace_val_use(vals[0], vals[0]); // Should be no-op
    let inc_after = format!("{:?}", store.get_op(inc_id));

    assert_eq!(inc_before, inc_after);
    store.check_ir(
        "
        %0 : Int = int_input<pos: 0>();
        %1 : Int = inc(%0);
    ",
    );
    Ok(())
}

/// Tests using same value multiple times in single operation
#[test]
fn test_same_value_multiple_args() -> Result<(), IRError<TestDialect>> {
    let mut store: IR<TestDialect> = IR::empty();
    let (_, vals) = store.add_op(Operations::IntInput { pos: 0 }, svec![])?;
    let (add_id, _) = store.add_op(Operations::Add, svec![vals[0], vals[0]])?;

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
    Ok(())
}

/// Tests diamond dependency pattern (A→B, A→C, B→D, C→D)
#[test]
fn test_diamond_dependencies() -> Result<(), IRError<TestDialect>> {
    let mut store: IR<TestDialect> = IR::empty();
    let (_, a_vals) = store.add_op(Operations::IntInput { pos: 0 }, svec![])?; // A
    let (_b_id, b_vals) = store.add_op(Operations::Inc, svec![a_vals[0]])?; // B depends on A
    let (_c_id, c_vals) = store.add_op(Operations::Inc, svec![a_vals[0]])?; // C depends on A
    let (d_id, _) = store.add_op(Operations::Add, svec![b_vals[0], c_vals[0]])?; // D depends on B,C

    let a_val = store.get_val(a_vals[0]);
    let d_op = store.get_op(d_id);

    // A should have 2 users (B and C)
    assert_eq!(a_val.get_users_iter().count(), 2);
    // D should be at depth 3 (A:1 → B,C:2 → D:3)
    assert_eq!(d_op.get_depth(), 3);
    Ok(())
}

/// Tests multiple independent subgraphs in same IR
#[test]
fn test_independent_subgraphs() -> Result<(), IRError<TestDialect>> {
    let mut store: IR<TestDialect> = IR::empty();

    // Subgraph 1: input1 → inc1
    let (_, vals1) = store.add_op(Operations::IntInput { pos: 0 }, svec![])?;
    let (inc1_id, _) = store.add_op(Operations::Inc, svec![vals1[0]])?;

    // Subgraph 2: input2 → inc2
    let (_, vals2) = store.add_op(Operations::IntInput { pos: 1 }, svec![])?;
    let (inc2_id, _) = store.add_op(Operations::Inc, svec![vals2[0]])?;

    let inc1 = store.get_op(inc1_id);
    let inc2 = store.get_op(inc2_id);

    // Neither should reach the other
    assert!(!inc1.reaches(inc2.clone()));
    assert!(!inc2.reaches(inc1.clone()));
    Ok(())
}

/// Tests operations with multi-return where returns have different user patterns
#[test]
fn test_multi_return_different_users() -> Result<(), IRError<TestDialect>> {
    let mut store: IR<TestDialect> = IR::empty();
    let (_, inp1) = store.add_op(Operations::IntInput { pos: 0 }, svec![])?;
    let (_, inp2) = store.add_op(Operations::IntInput { pos: 1 }, svec![])?;
    let (_, div_vals) = store.add_op(Operations::DivRem, svec![inp1[0], inp2[0]])?;

    // Use first return in one op, second return in another
    let (_, _) = store.add_op(Operations::Inc, svec![div_vals[0]])?; // Use quotient
    let (_, _) = store.add_op(Operations::Inc, svec![div_vals[1]])?; // Use remainder

    let quot = store.get_val(div_vals[0]);
    let rem = store.get_val(div_vals[1]);

    assert_eq!(quot.get_users_iter().count(), 1);
    assert_eq!(rem.get_users_iter().count(), 1);
    assert_ne!(
        quot.get_users_iter().next().unwrap().get_id(),
        rem.get_users_iter().next().unwrap().get_id()
    );
    Ok(())
}

/// Tests iteration behavior over IR with deleted elements
#[test]
fn test_iteration_with_deleted_elements() -> Result<(), IRError<TestDialect>> {
    let mut store: IR<TestDialect> = IR::empty();
    let (op1_id, _) = store.add_op(Operations::IntInput { pos: 0 }, svec![])?;
    let (op2_id, _) = store.add_op(Operations::IntInput { pos: 1 }, svec![])?;
    let (op3_id, _) = store.add_op(Operations::IntInput { pos: 2 }, svec![])?;

    store.delete_op(op2_id); // Delete middle operation

    let active_ops = store.ops_iter().map(|op| op.get_id()).covect();

    // Should only see active operations
    assert_eq!(active_ops.len(), 2);
    assert!(active_ops.contains(&op1_id));
    assert!(!active_ops.contains(&op2_id));
    assert!(active_ops.contains(&op3_id));

    // Raw iterator should see all operations
    let all_ops = store.raw_ops_iter().map(|op| op.get_id()).covect();
    assert_eq!(all_ops.len(), 3);
    Ok(())
}

/// Tests that user lists remain consistent after deletions
#[test]
fn test_user_consistency_after_deletion() -> Result<(), IRError<TestDialect>> {
    let mut store: IR<TestDialect> = IR::empty();
    let (_inp_id, vals) = store.add_op(Operations::IntInput { pos: 0 }, svec![])?;
    let (inc1_id, _) = store.add_op(Operations::Inc, svec![vals[0]])?;
    let (_inc2_id, _) = store.add_op(Operations::Inc, svec![vals[0]])?;

    // Value should have 2 users
    assert_eq!(store.get_val(vals[0]).get_users_iter().count(), 2);

    store.delete_op(inc1_id);

    // Value should still have 1 user, but the deleted op shouldn't appear in iteration
    let remaining_users: Vec<_> = store
        .get_val(vals[0])
        .raw_get_users_iter()
        .map(|op| op.get_id())
        .collect();
    assert_eq!(remaining_users.len(), 2); // Raw users list still contains deleted op

    // But active users should only show the remaining one
    let active_users: Vec<_> = store
        .get_val(vals[0])
        .get_users_iter()
        .filter(|op| op.is_active())
        .collect();
    assert_eq!(active_users.len(), 1);
    Ok(())
}

/// Tests replacement cascade affecting multiple dependency levels
#[test]
fn test_replacement_cascade_multiple_levels() -> Result<(), IRError<TestDialect>> {
    let mut store: IR<TestDialect> = IR::empty();

    // Create: inp1 → inc1 → inc2 → inc3
    //         inp2 (unused initially)
    let (_, inp1) = store.add_op(Operations::IntInput { pos: 0 }, svec![])?;
    let (_, inp2) = store.add_op(Operations::IntInput { pos: 1 }, svec![])?;
    let (inc1_id, inc1_vals) = store.add_op(Operations::Inc, svec![inp1[0]])?;
    let (inc2_id, inc2_vals) = store.add_op(Operations::Inc, svec![inc1_vals[0]])?;
    let (inc3_id, _) = store.add_op(Operations::Inc, svec![inc2_vals[0]])?;

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
    Ok(())
}

/// Tests replacement creating deeper dependency chain
#[test]
fn test_replacement_deeper_chain() -> Result<(), IRError<TestDialect>> {
    let mut store: IR<TestDialect> = IR::empty();

    // Create: inp1, inp2 → inc1, inp2 → inc2
    let (_, inp1) = store.add_op(Operations::IntInput { pos: 0 }, svec![])?;
    let (_, inp2) = store.add_op(Operations::IntInput { pos: 1 }, svec![])?;
    let (_, inc1_vals) = store.add_op(Operations::Inc, svec![inp2[0]])?;
    let (inc2_id, _) = store.add_op(Operations::Inc, svec![inp1[0]])?; // Initially uses inp1

    assert_eq!(store.get_op(inc2_id).get_depth(), 2);

    // Replace inp1 with inc1's output, making inc2 deeper
    store.replace_val_use(inp1[0], inc1_vals[0]);

    assert_eq!(store.get_op(inc2_id).get_depth(), 3); // Now inp2→inc1→inc2
    Ok(())
}

/// Tests has_opid/has_valid behavior with deleted elements
#[test]
fn test_has_id_with_deleted_elements() -> Result<(), IRError<TestDialect>> {
    let mut store: IR<TestDialect> = IR::empty();
    let (op_id, vals) = store.add_op(Operations::IntInput { pos: 0 }, svec![])?;

    assert!(store.has_opid(op_id));
    assert!(store.has_valid(vals[0]));

    store.delete_op(op_id);

    // Public API should return false for deleted elements
    assert!(!store.has_opid(op_id));
    assert!(!store.has_valid(vals[0]));

    // Raw API should still return true
    assert!(store.raw_has_opid(op_id));
    assert!(store.raw_has_valid(vals[0]));
    Ok(())
}

/// Tests operations on empty IR
#[test]
fn test_empty_ir_operations() -> Result<(), IRError<TestDialect>> {
    let store: IR<TestDialect> = IR::empty();

    assert_eq!(store.n_ops(), 0);
    assert_eq!(store.n_vals(), 0);
    assert_eq!(store.ops_iter().count(), 0);

    // Check that topological order works on empty IR
    let topo_ops: Vec<_> = store.raw_get_topological_order().collect();
    assert_eq!(topo_ops.len(), 0);
    Ok(())
}

/// Tests topological ordering with deleted operations
#[test]
fn test_topological_order_with_deletions() -> Result<(), IRError<TestDialect>> {
    let mut store: IR<TestDialect> = IR::empty();
    let (op1_id, vals1) = store.add_op(Operations::IntInput { pos: 0 }, svec![])?;
    let (op2_id, vals2) = store.add_op(Operations::Inc, svec![vals1[0]])?;
    let (op3_id, _vals3) = store.add_op(Operations::Inc, svec![vals2[0]])?;
    let (op4_id, _) = store.add_op(Operations::Inc, svec![vals1[0]])?; // Independent branch

    // Delete operations in reverse dependency order (leaf first)
    store.delete_op(op3_id); // Delete op3 first since it depends on op2
    store.delete_op(op2_id); // Now safe to delete op2

    // Topological order should include deleted operations in raw iterator
    let all_topo: Vec<_> = store
        .raw_topological_ops_iter()
        .map(|op| op.get_id())
        .collect();
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
    Ok(())
}

/// Tests replacement with type validation
#[test]
#[should_panic(expected = "Tried to replace a value with one of different type")]
fn test_replace_val_different_types() {
    let mut store: IR<TestDialect> = IR::empty();
    let (_, int_vals) = store
        .add_op(Operations::IntInput { pos: 0 }, svec![])
        .expect("Bad add_op");
    let (_, bool_vals) = store
        .add_op(Operations::BoolConstant { val: true }, svec![])
        .expect("Bad add_op");

    // Try to replace int value with bool value
    store.replace_val_use(int_vals[0], bool_vals[0]);
}

/// Tests operations that become unreachable after replacement but aren't deleted
#[test]
fn test_unreachable_after_replacement() -> Result<(), IRError<TestDialect>> {
    let mut store: IR<TestDialect> = IR::empty();
    let (_, inp1) = store.add_op(Operations::IntInput { pos: 0 }, svec![])?;
    let (_, inp2) = store.add_op(Operations::IntInput { pos: 1 }, svec![])?;
    let (inc1_id, inc1_vals) = store.add_op(Operations::Inc, svec![inp1[0]])?;
    let (_ret_id, _) = store.add_op(Operations::Return, svec![inc1_vals[0]])?;

    // Replace return's input with inp2, making inc1 unreachable
    store.replace_val_use(inc1_vals[0], inp2[0]);

    // inc1 should still exist and be active, but have no users
    assert!(store.has_opid(inc1_id));
    assert_eq!(store.get_val(inc1_vals[0]).get_users_iter().count(), 0);

    // Should be able to delete it now
    store.delete_op(inc1_id);
    assert!(!store.has_opid(inc1_id));
    Ok(())
}

#[test]
fn test_is_effect() -> Result<(), IRError<TestDialect>> {
    let mut ir = IR::<TestDialect>::empty();
    let (_, inp1) = ir.add_op(Operations::IntInput { pos: 0 }, svec![])?;
    let (_, inp2) = ir.add_op(Operations::IntInput { pos: 1 }, svec![])?;
    let (add_op, add) = ir.add_op(Operations::Add, svec![inp1[0], inp2[0]])?;
    let (ret_op, _) = ir.add_op(Operations::Return, svec![add[0]])?;

    assert!(ir.get_op(ret_op).is_effect());
    assert!(!ir.get_op(add_op).is_effect());
    Ok(())
}

#[test]
fn test_signature_consistency() -> Result<(), IRError<TestDialect>> {
    let mut ir = IR::<TestDialect>::empty();
    let (_, inp1) = ir.add_op(Operations::IntInput { pos: 0 }, svec![])?;
    let (_, inp2) = ir.add_op(Operations::IntInput { pos: 1 }, svec![])?;
    let (add_op, _) = ir.add_op(Operations::Add, svec![inp1[0], inp2[0]])?;
    let op_ref = ir.get_op(add_op);

    // Verify cached signature matches operation signature
    assert_eq!(op_ref.signature, &op_ref.operation.get_signature());
    Ok(())
}

#[test]
fn test_batch_delete_empty() -> Result<(), IRError<TestDialect>> {
    let mut ir = IR::<TestDialect>::empty();
    let (op1, _) = ir.add_op(Operations::IntInput { pos: 0 }, svec![])?;

    // Empty batch should be no-op
    ir.batch_delete_op(std::iter::empty());

    assert!(ir.has_opid(op1));
    assert_eq!(ir.n_ops(), 1);
    Ok(())
}

#[test]
fn test_batch_delete_single() -> Result<(), IRError<TestDialect>> {
    let mut ir = IR::<TestDialect>::empty();
    let (op1, vals1) = ir.add_op(Operations::IntInput { pos: 0 }, svec![])?;
    let (op2, _) = ir.add_op(Operations::Return, vals1)?;

    ir.batch_delete_op(std::iter::once(op2));

    assert!(ir.has_opid(op1));
    assert!(!ir.has_opid(op2));
    assert_eq!(ir.n_ops(), 1);
    Ok(())
}

#[test]
fn test_batch_delete_dependency_chain() -> Result<(), IRError<TestDialect>> {
    let mut ir = IR::<TestDialect>::empty();
    let (op1, vals1) = ir.add_op(Operations::IntInput { pos: 0 }, svec![])?;
    let (op2, vals2) = ir.add_op(Operations::Inc, vals1)?;
    let (op3, vals3) = ir.add_op(Operations::Inc, vals2)?;
    let (op4, _) = ir.add_op(Operations::Return, vals3)?;

    // Delete the entire chain
    ir.batch_delete_op([op2, op3, op4].into_iter());

    assert!(ir.has_opid(op1));
    assert!(!ir.has_opid(op2));
    assert!(!ir.has_opid(op3));
    assert!(!ir.has_opid(op4));
    assert_eq!(ir.n_ops(), 1);
    Ok(())
}

#[test]
fn test_batch_delete_order_independence() -> Result<(), IRError<TestDialect>> {
    let mut ir = IR::<TestDialect>::empty();
    let (op1, vals1) = ir.add_op(Operations::IntInput { pos: 0 }, svec![])?;
    let (op2, vals2) = ir.add_op(Operations::Inc, vals1)?;
    let (op3, _) = ir.add_op(Operations::Return, vals2)?;

    // Delete in reverse dependency order - should still work
    ir.batch_delete_op([op2, op3].into_iter());

    assert!(ir.has_opid(op1));
    assert!(!ir.has_opid(op2));
    assert!(!ir.has_opid(op3));
    assert_eq!(ir.n_ops(), 1);
    Ok(())
}

#[test]
fn test_batch_delete_diamond_pattern() -> Result<(), IRError<TestDialect>> {
    let mut ir = IR::<TestDialect>::empty();
    let (op1, vals1) = ir.add_op(Operations::IntInput { pos: 0 }, svec![])?;
    let (op2, vals2) = ir.add_op(Operations::Inc, vals1.clone())?;
    let (op3, vals3) = ir.add_op(Operations::Inc, vals1)?;
    let (op4, _) = ir.add_op(Operations::Add, svec![vals2[0], vals3[0]])?;

    // Delete the diamond (op2, op3, op4) but leave op1
    ir.batch_delete_op([op2, op3, op4].into_iter());

    assert!(ir.has_opid(op1));
    assert!(!ir.has_opid(op2));
    assert!(!ir.has_opid(op3));
    assert!(!ir.has_opid(op4));
    assert_eq!(ir.n_ops(), 1);
    Ok(())
}

#[test]
fn test_batch_delete_independent_operations() -> Result<(), IRError<TestDialect>> {
    let mut ir = IR::<TestDialect>::empty();
    let (op1, vals1) = ir.add_op(Operations::IntInput { pos: 0 }, svec![])?;
    let (op2, vals2) = ir.add_op(Operations::IntInput { pos: 1 }, svec![])?;
    let (op3, _) = ir.add_op(Operations::Return, vals1)?;
    let (op4, _) = ir.add_op(Operations::Return, vals2)?;

    // Delete two independent subgraphs
    ir.batch_delete_op([op3, op4].into_iter());

    assert!(ir.has_opid(op1));
    assert!(ir.has_opid(op2));
    assert!(!ir.has_opid(op3));
    assert!(!ir.has_opid(op4));
    assert_eq!(ir.n_ops(), 2);
    Ok(())
}

#[test]
#[should_panic(expected = "Tried to delete an operation whose return values are still in use")]
fn test_batch_delete_with_external_users() {
    let mut ir = IR::<TestDialect>::empty();
    let (_op1, vals1) = ir
        .add_op(Operations::IntInput { pos: 0 }, svec![])
        .expect("Bad add_op");
    let (op2, vals2) = ir.add_op(Operations::Inc, vals1).expect("Bad add_op");
    let (op3, _) = ir
        .add_op(Operations::Return, vals2.clone())
        .expect("Bad add_op");
    let (_op4, _) = ir
        .add_op(Operations::Return, vals2) // op4 also uses vals?2
        .expect("Bad add_op");

    // Try to delete op2 and op3, but op4 still uses vals2 from op2
    ir.batch_delete_op([op2, op3].into_iter());
}

#[test]
fn test_batch_delete_partial_dependency_closure() -> Result<(), IRError<TestDialect>> {
    let mut ir = IR::<TestDialect>::empty();
    let (op1, vals1) = ir.add_op(Operations::IntInput { pos: 0 }, svec![])?;
    let (op2, vals2) = ir.add_op(Operations::Inc, vals1)?;
    let (op3, vals3) = ir.add_op(Operations::Inc, vals2.clone())?;
    let (op4, _) = ir.add_op(Operations::Return, vals3)?;
    let (op5, _) = ir.add_op(Operations::Return, vals2)?; // Also uses vals2

    // Can delete op3 and op4 (leaves op2 and op5 intact)
    ir.batch_delete_op([op3, op4].into_iter());

    assert!(ir.has_opid(op1));
    assert!(ir.has_opid(op2));
    assert!(!ir.has_opid(op3));
    assert!(!ir.has_opid(op4));
    assert!(ir.has_opid(op5));
    assert_eq!(ir.n_ops(), 3);
    Ok(())
}

#[test]
#[should_panic(expected = "Tried to get a dead op")]
fn test_batch_delete_already_deleted() {
    let mut ir = IR::<TestDialect>::empty();
    let (_, vals1) = ir
        .add_op(Operations::IntInput { pos: 0 }, svec![])
        .expect("Bad add_op");
    let (op2, _) = ir.add_op(Operations::Return, vals1).expect("Bad add_op");

    ir.delete_op(op2); // Delete normally first

    // Try to batch delete already deleted operation
    ir.batch_delete_op(std::iter::once(op2));
}
