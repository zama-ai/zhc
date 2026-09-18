//! Position tracking in `ValOrigin` (which return slot) and `ValUse` (which argument slot).

use crate::IR;
use crate::tests::testlang::{TestInstructionSet, TestLang};
use zhc_utils::svec;

/// Tests that ValOrigin correctly tracks the position of values within multi-return operations.
///
/// This verifies that when an operation returns multiple values, each value knows not just
/// which operation produced it, but also which return position it came from.
#[test]
fn test_val_origin_position_tracking() {
    let mut store: IR<TestLang> = IR::empty();

    // Create an operation with multiple return values
    let (_input_id, v_input) = store.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![]);
    let (divrem_id, v_divrem) =
        store.add_op(TestInstructionSet::DivRem, svec![v_input[0], v_input[0]]);

    // Test that each return value has the correct position in its origin
    let quotient = store.get_val(v_divrem[0]);
    let remainder = store.get_val(v_divrem[1]);

    // Verify the origin operation is correct
    assert_eq!(quotient.get_origin().opref.get_id(), divrem_id);
    assert_eq!(remainder.get_origin().opref.get_id(), divrem_id);

    // Verify the positions are correct - quotient should be position 0, remainder position 1
    assert_eq!(quotient.get_origin().position, 0);
    assert_eq!(remainder.get_origin().position, 1);
}

/// Tests that ValUse correctly tracks the argument position where values are consumed.
///
/// This verifies that when a value is used as an argument to an operation, the value
/// knows not just which operation uses it, but also at which argument position.
#[test]
fn test_val_use_position_tracking() {
    let mut store: IR<TestLang> = IR::empty();

    // Create values to be used as arguments
    let (_input1_id, v1) = store.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![]);
    let (_input2_id, v2) = store.add_op(TestInstructionSet::IntInput { pos: 1 }, svec![]);

    // Create an operation that uses both values as arguments
    let (add_id, _v_add) = store.add_op(TestInstructionSet::Add, svec![v1[0], v2[0]]);

    // Test that values track their usage positions correctly
    let val1 = store.get_val(v1[0]);
    let val2 = store.get_val(v2[0]);

    // Each value should have exactly one user (the add operation)
    assert_eq!(val1.users.len(), 1);
    assert_eq!(val2.users.len(), 1);

    // Check the user operation ID is correct
    assert_eq!(val1.users[0].opid, add_id);
    assert_eq!(val2.users[0].opid, add_id);

    // Check the positions - v1[0] should be at position 0, v2[0] at position 1
    assert_eq!(val1.users[0].position, 0);
    assert_eq!(val2.users[0].position, 1);
}

/// Tests position tracking when the same value is used multiple times in different positions.
///
/// This verifies that when a value is used as arguments in multiple operations and positions,
/// each usage is tracked with the correct operation ID and argument position.
#[test]
fn test_position_tracking_with_multiple_uses() {
    let mut store: IR<TestLang> = IR::empty();

    // Create a value that will be used multiple times
    let (_input_id, v_input) = store.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![]);

    // Use the same value in different positions of different operations
    let (add1_id, _v_add1) = store.add_op(TestInstructionSet::Add, svec![v_input[0], v_input[0]]);
    let (divrem_id, _v_divrem) =
        store.add_op(TestInstructionSet::DivRem, svec![v_input[0], v_input[0]]);

    let input_val = store.get_val(v_input[0]);

    // The input value should be used 4 times total (twice in each operation)
    assert_eq!(input_val.users.len(), 4);

    // Collect the usage information for verification
    let mut uses = input_val.users.iter().collect::<Vec<_>>();
    uses.sort_by_key(|u| (u.opid, u.position));

    // Verify the usage positions are tracked correctly
    let add1_uses: Vec<_> = uses.iter().filter(|u| u.opid == add1_id).collect();
    let divrem_uses: Vec<_> = uses.iter().filter(|u| u.opid == divrem_id).collect();

    assert_eq!(add1_uses.len(), 2);
    assert_eq!(divrem_uses.len(), 2);

    // Each operation should use the value at both position 0 and 1
    assert!(add1_uses.iter().any(|u| u.position == 0));
    assert!(add1_uses.iter().any(|u| u.position == 1));
    assert!(divrem_uses.iter().any(|u| u.position == 0));
    assert!(divrem_uses.iter().any(|u| u.position == 1));
}

/// Tests comprehensive position tracking with operations that have multiple returns and uses.
///
/// This combines multi-return operations with multi-argument operations to verify that
/// both origin positions (which return value) and use positions (which argument) are
/// correctly tracked throughout the dataflow graph.
#[test]
fn test_position_tracking_with_multi_return_multi_use() {
    let mut store: IR<TestLang> = IR::empty();

    // Create initial values
    let (_input1_id, v1) = store.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![]);
    let (_input2_id, v2) = store.add_op(TestInstructionSet::IntInput { pos: 1 }, svec![]);

    // Create an operation with multiple returns
    let (divrem_id, v_divrem) = store.add_op(TestInstructionSet::DivRem, svec![v1[0], v2[0]]);

    // Use both return values in different positions of a new operation
    let (add_id, _v_add) = store.add_op(TestInstructionSet::Add, svec![v_divrem[0], v_divrem[1]]);

    // Test origin position tracking for the multi-return operation
    let quotient = store.get_val(v_divrem[0]);
    let remainder = store.get_val(v_divrem[1]);

    assert_eq!(quotient.get_origin().opref.get_id(), divrem_id);
    assert_eq!(quotient.get_origin().position, 0);
    assert_eq!(remainder.get_origin().opref.get_id(), divrem_id);
    assert_eq!(remainder.get_origin().position, 1);

    // Test use position tracking for values used by the add operation
    assert_eq!(quotient.users.len(), 1);
    assert_eq!(remainder.users.len(), 1);

    assert_eq!(quotient.users[0].opid, add_id);
    assert_eq!(quotient.users[0].position, 0);
    assert_eq!(remainder.users[0].opid, add_id);
    assert_eq!(remainder.users[0].position, 1);
}

/// Tests that position information remains consistent after value replacement operations.
///
/// This verifies that when replace_val_use is called, the position tracking is updated
/// correctly - the new value should be tracked at the same argument position where the
/// old value was used.
#[test]
fn test_position_consistency_after_replacement() {
    let mut store: IR<TestLang> = IR::empty();

    // Create values
    let (_input1_id, v1) = store.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![]);
    let (_input2_id, v2) = store.add_op(TestInstructionSet::IntInput { pos: 1 }, svec![]);
    let (_input3_id, v3) = store.add_op(TestInstructionSet::IntInput { pos: 2 }, svec![]);

    // Create an operation using the first value
    let (add_id, _v_add) = store.add_op(TestInstructionSet::Add, svec![v1[0], v2[0]]);

    // Replace the first argument with a different value
    store.replace_val_use(v1[0], v3[0]);

    // Verify position tracking is maintained after replacement
    let val3 = store.get_val(v3[0]);
    let val1 = store.get_val(v1[0]);

    // val3 should now be used at position 0 of the add operation
    assert!(
        val3.users
            .iter()
            .any(|u| u.opid == add_id && u.position == 0)
    );

    // val1 should no longer be used by the add operation
    assert!(!val1.users.iter().any(|u| u.opid == add_id));
}

/// Tests position tracking with single-argument operations.
///
/// This verifies that position tracking works correctly for operations that take
/// only one argument, ensuring the position is tracked as 0.
#[test]
fn test_position_tracking_single_argument_operation() {
    let mut store: IR<TestLang> = IR::empty();

    // Create a value and use it in a single-argument operation
    let (_input_id, v_input) = store.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![]);
    let (inc_id, _v_inc) = store.add_op(TestInstructionSet::Inc, svec![v_input[0]]);

    // Verify position tracking for single-argument operation
    let input_val = store.get_val(v_input[0]);

    assert_eq!(input_val.users.len(), 1);
    assert_eq!(input_val.users[0].opid, inc_id);
    assert_eq!(input_val.users[0].position, 0);
}

/// Tests position tracking with single-return operations.
///
/// This verifies that position tracking works correctly for operations that return
/// only one value, ensuring the return position is tracked as 0.
#[test]
fn test_position_tracking_single_return_operation() {
    let mut store: IR<TestLang> = IR::empty();

    // Create operations with single return values
    let (_input_id, v_input) = store.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![]);
    let (add_id, v_add) = store.add_op(TestInstructionSet::Add, svec![v_input[0], v_input[0]]);

    // Verify position tracking for single-return operation
    let result_val = store.get_val(v_add[0]);

    assert_eq!(result_val.get_origin().opref.get_id(), add_id);
    assert_eq!(result_val.get_origin().position, 0);
}

/// Tests position tracking with zero-argument operations.
///
/// This verifies that operations with no arguments (like input operations) still
/// have their return values properly tracked with position 0.
#[test]
fn test_position_tracking_zero_argument_operation() {
    let mut store: IR<TestLang> = IR::empty();

    // Create an operation with no arguments (like IntInput)
    let (input_id, v_input) = store.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![]);

    // Verify the value's origin tracking
    let input_val = store.get_val(v_input[0]);

    assert_eq!(input_val.get_origin().opref.get_id(), input_id);
    assert_eq!(input_val.get_origin().position, 0);
    assert_eq!(input_val.users.len(), 0); // No users initially
}

/// Tests that position tracking is properly maintained after operation deletion.
///
/// This verifies that when operations are deleted, the position tracking information
/// behaves correctly - raw users lists retain deleted operations, but filtered
/// iteration excludes inactive operations.
#[test]
fn test_position_tracking_after_deletion() {
    let mut store: IR<TestLang> = IR::empty();

    // Create a chain of operations
    let (_input1_id, v1) = store.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![]);
    let (_input2_id, v2) = store.add_op(TestInstructionSet::IntInput { pos: 1 }, svec![]);
    let (_add1_id, v_add1) = store.add_op(TestInstructionSet::Add, svec![v1[0], v2[0]]);
    let (add2_id, _v_add2) = store.add_op(TestInstructionSet::Add, svec![v_add1[0], v1[0]]);

    // Verify initial position tracking
    let intermediate_val = store.get_val(v_add1[0]);
    assert_eq!(intermediate_val.users.len(), 1);
    assert_eq!(intermediate_val.users[0].opid, add2_id);
    assert_eq!(intermediate_val.users[0].position, 0);

    // Delete the second add operation
    store.delete_op(add2_id);

    // Verify the intermediate value still has the deleted operation in raw users list
    let intermediate_val_after = store.get_val(v_add1[0]);
    assert_eq!(intermediate_val_after.users.len(), 1); // Raw list still contains deleted op

    // But filtered iteration should show no active users
    let active_users: Vec<_> = intermediate_val_after.get_users_iter().collect();
    assert_eq!(active_users.len(), 0);
}

/// Tests basic equality and functionality of ValOrigin and ValUse types.
///
/// This verifies that the ValOrigin and ValUse structs have proper equality
/// semantics and can be constructed and compared correctly.
#[test]
fn test_val_origin_and_val_use_equality() {
    use crate::{ValOrigin, ValUse};

    let mut store: IR<TestLang> = IR::empty();

    // Create some operations to get valid OpIds
    let (op1_id, _v1) = store.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![]);
    let (op2_id, _v2) = store.add_op(TestInstructionSet::IntInput { pos: 1 }, svec![]);

    // Test ValOrigin equality
    let origin1 = ValOrigin {
        opid: op1_id,
        position: 0,
    };
    let origin2 = ValOrigin {
        opid: op1_id,
        position: 0,
    };
    let origin3 = ValOrigin {
        opid: op1_id,
        position: 1,
    };
    let origin4 = ValOrigin {
        opid: op2_id,
        position: 0,
    };

    assert_eq!(origin1, origin2);
    assert_ne!(origin1, origin3);
    assert_ne!(origin1, origin4);

    // Test ValUse equality
    let use1 = ValUse {
        opid: op1_id,
        position: 0,
    };
    let use2 = ValUse {
        opid: op1_id,
        position: 0,
    };
    let use3 = ValUse {
        opid: op1_id,
        position: 1,
    };
    let use4 = ValUse {
        opid: op2_id,
        position: 0,
    };

    assert_eq!(use1, use2);
    assert_ne!(use1, use3);
    assert_ne!(use1, use4);

    // Test cloning
    let origin_clone = origin1.clone();
    let use_clone = use1.clone();

    assert_eq!(origin1, origin_clone);
    assert_eq!(use1, use_clone);
}

/// Tests that ValOrigin and ValUse maintain their Debug trait implementation.
///
/// This ensures that these types can be printed for debugging purposes
/// and that their debug output contains the expected information.
#[test]
fn test_val_origin_and_val_use_debug() {
    use crate::{ValOrigin, ValUse};

    let mut store: IR<TestLang> = IR::empty();
    let (op_id, _v) = store.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![]);

    let origin = ValOrigin {
        opid: op_id,
        position: 2,
    };
    let use_val = ValUse {
        opid: op_id,
        position: 3,
    };

    let origin_debug = format!("{:?}", origin);
    let use_debug = format!("{:?}", use_val);

    // Verify that debug output contains the expected fields
    assert!(origin_debug.contains("ValOrigin"));
    assert!(origin_debug.contains("opid"));
    assert!(origin_debug.contains("position"));
    assert!(origin_debug.contains("2"));

    assert!(use_debug.contains("ValUse"));
    assert!(use_debug.contains("opid"));
    assert!(use_debug.contains("position"));
    assert!(use_debug.contains("3"));
}

/// Tests integration of position tracking with IR iterator methods.
///
/// This verifies that the position tracking works correctly when accessed through
/// the standard IR iteration methods like get_args_iter() and get_users_iter().
#[test]
fn test_position_tracking_with_iterators() {
    let mut store: IR<TestLang> = IR::empty();

    // Build a more complex graph to test iterator integration
    let (_input1_id, v1) = store.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![]);
    let (_input2_id, v2) = store.add_op(TestInstructionSet::IntInput { pos: 1 }, svec![]);
    let (divrem_id, v_divrem) = store.add_op(TestInstructionSet::DivRem, svec![v1[0], v2[0]]);
    let (add_id, _v_add) = store.add_op(TestInstructionSet::Add, svec![v_divrem[0], v_divrem[1]]);

    // Test that get_args_iter provides values with correct position tracking
    let add_op = store.get_op(add_id);
    let args: Vec<_> = add_op.get_args_iter().collect();

    assert_eq!(args.len(), 2);

    // First argument should be the quotient (position 0 from divrem)
    assert_eq!(args[0].get_origin().opref.get_id(), divrem_id);
    assert_eq!(args[0].get_origin().position, 0);

    // Second argument should be the remainder (position 1 from divrem)
    assert_eq!(args[1].get_origin().opref.get_id(), divrem_id);
    assert_eq!(args[1].get_origin().position, 1);

    // Test that get_returns_iter provides values with correct position tracking
    let divrem_op = store.get_op(divrem_id);
    let returns: Vec<_> = divrem_op.get_returns_iter().collect();

    assert_eq!(returns.len(), 2);
    assert_eq!(returns[0].get_origin().position, 0);
    assert_eq!(returns[1].get_origin().position, 1);

    // Test that get_users_iter reflects correct position usage
    let quotient_users: Vec<_> = returns[0].get_users_iter().collect();
    let remainder_users: Vec<_> = returns[1].get_users_iter().collect();

    assert_eq!(quotient_users.len(), 1);
    assert_eq!(remainder_users.len(), 1);
    assert_eq!(quotient_users[0].get_id(), add_id);
    assert_eq!(remainder_users[0].get_id(), add_id);

    // Test get_uses_iter to verify position information
    let quotient_uses: Vec<_> = returns[0].get_uses_iter().collect();
    let remainder_uses: Vec<_> = returns[1].get_uses_iter().collect();

    assert_eq!(quotient_uses.len(), 1);
    assert_eq!(remainder_uses.len(), 1);

    // Verify position tracking in the use references
    assert_eq!(quotient_uses[0].opref.get_id(), add_id);
    assert_eq!(quotient_uses[0].position, 0);

    assert_eq!(remainder_uses[0].opref.get_id(), add_id);
    assert_eq!(remainder_uses[0].position, 1);

    // Verify the actual position tracking in the users lists
    assert_eq!(returns[0].users.len(), 1);
    assert_eq!(returns[0].users[0].opid, add_id);
    assert_eq!(returns[0].users[0].position, 0);

    assert_eq!(returns[1].users.len(), 1);
    assert_eq!(returns[1].users[0].opid, add_id);
    assert_eq!(returns[1].users[0].position, 1);
}
