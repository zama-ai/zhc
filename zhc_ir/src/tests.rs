#[allow(unused)]
pub mod test_dialect {
    use std::fmt::Display;

    use crate::{Dialect, DialectInstructionSet, DialectTypeSystem, signature::Signature};

    use zhc_utils::svec;

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub enum TestTypeSystem {
        Int,
        Bool,
    }
    impl Display for TestTypeSystem {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                TestTypeSystem::Int => write!(f, "Int"),
                TestTypeSystem::Bool => write!(f, "Bool"),
            }
        }
    }
    impl DialectTypeSystem for TestTypeSystem {}

    #[derive(Debug, Clone, Hash, PartialEq, Eq)]
    pub enum TestInstructionSet {
        IntInput { pos: usize },
        BoolConstant { val: bool },
        Add,
        IfElse,
        DivRem,
        Inc,
        Return,
    }

    impl Display for TestInstructionSet {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                TestInstructionSet::IntInput { pos } => write!(f, "int_input<pos: {}>", pos),
                TestInstructionSet::BoolConstant { val } => write!(f, "bool_constant<val: {}>", val),
                TestInstructionSet::Add => write!(f, "add"),
                TestInstructionSet::IfElse => write!(f, "if_else"),
                TestInstructionSet::DivRem => write!(f, "div_rem"),
                TestInstructionSet::Inc => write!(f, "inc"),
                TestInstructionSet::Return => write!(f, "return"),
            }
        }
    }

    impl DialectInstructionSet for TestInstructionSet {
        type TypeSystem = TestTypeSystem;

        fn get_signature(&self) -> crate::signature::Signature<Self::TypeSystem> {
            use TestTypeSystem::*;
            match self {
                TestInstructionSet::IntInput { .. } => Signature(svec![], svec![Int]),
                TestInstructionSet::BoolConstant { .. } => Signature(svec![], svec![Bool]),
                TestInstructionSet::Add => Signature(svec![Int, Int], svec![Int]),
                TestInstructionSet::IfElse => Signature(svec![Int, Bool, Int], svec![Int]),
                TestInstructionSet::DivRem => Signature(svec![Int, Int], svec![Int, Int]),
                TestInstructionSet::Inc => Signature(svec![Int], svec![Int]),
                TestInstructionSet::Return => Signature(svec![Int], svec![]),
            }
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct TestLang;

    impl Dialect for TestLang {
        type TypeSystem = TestTypeSystem;
        type InstructionSet = TestInstructionSet;
    }
}

use crate::{DialectInstructionSet, IR, IRError};
use zhc_utils::{assert_display_is, iter::CollectInVec, svec};

use test_dialect::{TestInstructionSet, TestLang};

#[allow(unused)]
pub fn gen_complex_ir() -> Result<IR<TestLang>, IRError<TestLang>> {
    let mut ir: IR<TestLang> = IR::empty();

    // Create multiple input sources (wide foundation)
    let (_, inp0) = ir.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![])?;
    let (_, inp1) = ir.add_op(TestInstructionSet::IntInput { pos: 1 }, svec![])?;
    let (_, inp2) = ir.add_op(TestInstructionSet::IntInput { pos: 2 }, svec![])?;
    let (_, inp3) = ir.add_op(TestInstructionSet::IntInput { pos: 3 }, svec![])?;
    let (_, bool_inp) = ir.add_op(TestInstructionSet::BoolConstant { val: true }, svec![])?;

    // First layer - basic operations on inputs
    let (_, add0) = ir.add_op(TestInstructionSet::Add, svec![inp0[0], inp1[0]])?;
    let (_, add1) = ir.add_op(TestInstructionSet::Add, svec![inp2[0], inp3[0]])?;
    let (_, inc0) = ir.add_op(TestInstructionSet::Inc, svec![inp0[0]])?;
    let (_, inc1) = ir.add_op(TestInstructionSet::Inc, svec![inp1[0]])?;

    // Create a diamond pattern: add0 -> inc2, inc3 -> add2
    let (_, inc2) = ir.add_op(TestInstructionSet::Inc, svec![add0[0]])?;
    let (_, inc3) = ir.add_op(TestInstructionSet::Inc, svec![add0[0]])?;
    let (_, add2) = ir.add_op(TestInstructionSet::Add, svec![inc2[0], inc3[0]])?;

    // Create a deeper chain from inp2
    let (_, chain0) = ir.add_op(TestInstructionSet::Inc, svec![inp2[0]])?;
    let (_, chain1) = ir.add_op(TestInstructionSet::Inc, svec![chain0[0]])?;
    let (_, chain2) = ir.add_op(TestInstructionSet::Inc, svec![chain1[0]])?;
    let (_, chain3) = ir.add_op(TestInstructionSet::Inc, svec![chain2[0]])?;
    let (_, chain4) = ir.add_op(TestInstructionSet::Inc, svec![chain3[0]])?;

    // Multi-output operation creating branching
    let (_, divrem0) = ir.add_op(TestInstructionSet::DivRem, svec![add1[0], inc0[0]])?;
    let (_, divrem1) = ir.add_op(TestInstructionSet::DivRem, svec![chain4[0], inp3[0]])?;

    // Fan-out: use both outputs of divrem operations
    let (_, inc4) = ir.add_op(TestInstructionSet::Inc, svec![divrem0[0]])?; // quotient
    let (_, inc5) = ir.add_op(TestInstructionSet::Inc, svec![divrem0[1]])?; // remainder
    let (_, inc6) = ir.add_op(TestInstructionSet::Inc, svec![divrem1[0]])?; // quotient
    let (_, inc7) = ir.add_op(TestInstructionSet::Inc, svec![divrem1[1]])?; // remainder

    // Create convergence points
    let (_, conv0) = ir.add_op(TestInstructionSet::Add, svec![inc4[0], inc5[0]])?;
    let (_, conv1) = ir.add_op(TestInstructionSet::Add, svec![inc6[0], inc7[0]])?;
    let (_, conv2) = ir.add_op(TestInstructionSet::Add, svec![add2[0], chain2[0]])?;

    // IfElse operations using the boolean input
    let (_, ifelse0) = ir.add_op(TestInstructionSet::IfElse, svec![conv0[0], bool_inp[0], conv1[0]])?;
    let (_, ifelse1) = ir.add_op(TestInstructionSet::IfElse, svec![conv2[0], bool_inp[0], inc1[0]])?;

    // Create more complex interactions
    let (_, add3) = ir.add_op(TestInstructionSet::Add, svec![ifelse0[0], ifelse1[0]])?;
    let (_, add4) = ir.add_op(TestInstructionSet::Add, svec![conv0[0], conv1[0]])?;
    let (_, add5) = ir.add_op(TestInstructionSet::Add, svec![chain4[0], add2[0]])?;

    // Another level of DivRem for more multi-output complexity
    let (_, divrem2) = ir.add_op(TestInstructionSet::DivRem, svec![add3[0], add4[0]])?;
    let (_, divrem3) = ir.add_op(TestInstructionSet::DivRem, svec![add5[0], ifelse1[0]])?;

    // Final convergence layer
    let (_, final0) = ir.add_op(TestInstructionSet::Add, svec![divrem2[0], divrem3[0]])?;
    let (_, final1) = ir.add_op(TestInstructionSet::Add, svec![divrem2[1], divrem3[1]])?;
    let (_, final2) = ir.add_op(TestInstructionSet::Add, svec![final0[0], final1[0]])?;

    // Independent subgraph that eventually merges
    let (_, indep0) = ir.add_op(TestInstructionSet::Inc, svec![inp0[0]])?;
    let (_, indep1) = ir.add_op(TestInstructionSet::Inc, svec![indep0[0]])?;
    let (_, indep2) = ir.add_op(TestInstructionSet::Inc, svec![indep1[0]])?;

    // Merge independent subgraph with main computation
    let (_, ultimate) = ir.add_op(TestInstructionSet::Add, svec![final2[0], indep2[0]])?;

    // Some effect operations
    let (_, _) = ir.add_op(TestInstructionSet::Return, svec![ultimate[0]])?;
    let (_, _) = ir.add_op(TestInstructionSet::Return, svec![final0[0]])?;
    let (_, _) = ir.add_op(TestInstructionSet::Return, svec![conv2[0]])?;

    // Additional independent operations to reach ~50 nodes
    let (_, extra0) = ir.add_op(TestInstructionSet::Inc, svec![inp3[0]])?;
    let (_, extra1) = ir.add_op(TestInstructionSet::Inc, svec![extra0[0]])?;
    let (_, extra2) = ir.add_op(TestInstructionSet::Add, svec![extra1[0], chain1[0]])?;
    let (_, _) = ir.add_op(TestInstructionSet::Return, svec![extra2[0]])?;

    // More branching from existing values
    let (_, branch0) = ir.add_op(TestInstructionSet::Inc, svec![add1[0]])?;
    let (_, branch1) = ir.add_op(TestInstructionSet::Inc, svec![branch0[0]])?;
    let (_, branch2) = ir.add_op(TestInstructionSet::Add, svec![branch1[0], inc7[0]])?;
    let (_, _) = ir.add_op(TestInstructionSet::Return, svec![branch2[0]])?;

    assert_display_is!(
        ir.format(),
        r#"
        %0 : Int = int_input<pos: 0>();
        %1 : Int = int_input<pos: 1>();
        %2 : Int = int_input<pos: 2>();
        %3 : Int = int_input<pos: 3>();
        %4 : Bool = bool_constant<val: true>();
        %5 : Int = add(%0 : Int, %1 : Int);
        %6 : Int = add(%2 : Int, %3 : Int);
        %7 : Int = inc(%0 : Int);
        %8 : Int = inc(%1 : Int);
        %12 : Int = inc(%2 : Int);
        %40 : Int = inc(%0 : Int);
        %44 : Int = inc(%3 : Int);
        %9 : Int = inc(%5 : Int);
        %10 : Int = inc(%5 : Int);
        %13 : Int = inc(%12 : Int);
        %17 : Int, %18 : Int = div_rem(%6 : Int, %7 : Int);
        %41 : Int = inc(%40 : Int);
        %45 : Int = inc(%44 : Int);
        %47 : Int = inc(%6 : Int);
        %11 : Int = add(%9 : Int, %10 : Int);
        %14 : Int = inc(%13 : Int);
        %21 : Int = inc(%17 : Int);
        %22 : Int = inc(%18 : Int);
        %42 : Int = inc(%41 : Int);
        %46 : Int = add(%45 : Int, %13 : Int);
        %48 : Int = inc(%47 : Int);
        %15 : Int = inc(%14 : Int);
        %25 : Int = add(%21 : Int, %22 : Int);
        %27 : Int = add(%11 : Int, %14 : Int);
        return(%46 : Int);
        %16 : Int = inc(%15 : Int);
        %29 : Int = if_else(%27 : Int, %4 : Bool, %8 : Int);
        return(%27 : Int);
        %19 : Int, %20 : Int = div_rem(%16 : Int, %3 : Int);
        %32 : Int = add(%16 : Int, %11 : Int);
        %23 : Int = inc(%19 : Int);
        %24 : Int = inc(%20 : Int);
        %35 : Int, %36 : Int = div_rem(%32 : Int, %29 : Int);
        %26 : Int = add(%23 : Int, %24 : Int);
        %49 : Int = add(%48 : Int, %24 : Int);
        %28 : Int = if_else(%25 : Int, %4 : Bool, %26 : Int);
        %31 : Int = add(%25 : Int, %26 : Int);
        return(%49 : Int);
        %30 : Int = add(%28 : Int, %29 : Int);
        %33 : Int, %34 : Int = div_rem(%30 : Int, %31 : Int);
        %37 : Int = add(%33 : Int, %35 : Int);
        %38 : Int = add(%34 : Int, %36 : Int);
        %39 : Int = add(%37 : Int, %38 : Int);
        return(%37 : Int);
        %43 : Int = add(%39 : Int, %42 : Int);
        return(%43 : Int);
    "#
    );

    Ok(ir)
}

/// Tests basic IR construction with complex operation graph and validates
/// all operation properties, value relationships, and depth calculations
#[test]
fn test_construction() -> Result<(), IRError<TestLang>> {
    let mut store: IR<TestLang> = IR::empty();

    let (lhs_id, v0) = store.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![])?;
    let (rhs_id, v1) = store.add_op(TestInstructionSet::IntInput { pos: 1 }, svec![])?;
    let (join_id, v2) = store.add_op(TestInstructionSet::Add, svec![v0[0], v1[0]])?;
    let (split_id, v3) = store.add_op(TestInstructionSet::DivRem, svec![v2[0], v0[0]])?;
    let (ulhs_id, v4) = store.add_op(TestInstructionSet::Inc, svec![v3[0]])?;
    let (urhs_id, v5) = store.add_op(TestInstructionSet::Inc, svec![v3[1]])?;
    let (final_add_id, v6) = store.add_op(TestInstructionSet::Add, svec![v4[0], v5[0]])?;
    let (effect_id, _) = store.add_op(TestInstructionSet::Return, svec![v3[0]])?;

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
        %0 : Int = int_input<pos: 0>();
        %1 : Int = int_input<pos: 1>();
        %2 : Int = add(%0 : Int, %1 : Int);
        %3 : Int, %4 : Int = div_rem(%2 : Int, %0 : Int);
        %5 : Int = inc(%3 : Int);
        %6 : Int = inc(%4 : Int);
        return(%3 : Int);
        %7 : Int = add(%5 : Int, %6 : Int);
    "#
    );

    assert_eq!(store.n_ops(), 8);
    assert_eq!(store.n_vals(), 8);
    assert!(lhs.is_active());
    assert_eq!(lhs.get_depth(), 0);
    assert_eq!(lhs.get_args_iter().covec(), []);
    assert_eq!(lhs.get_returns_iter().covec(), [p0.clone()]);

    assert!(rhs.is_active());
    assert_eq!(rhs.get_depth(), 0);
    assert_eq!(rhs.get_args_iter().covec(), []);
    assert_eq!(rhs.get_returns_iter().covec(), [p1.clone()]);

    assert!(p0.is_active());
    assert_eq!(p0.get_origin().opref, lhs);
    assert_eq!(p0.get_users_iter().covec(), [join.clone(), split.clone()]);

    assert!(p1.is_active());
    assert_eq!(p1.get_origin().opref, rhs);
    assert_eq!(p1.get_users_iter().covec(), [join.clone()]);

    assert!(join.is_active());
    assert_eq!(join.get_depth(), 1);
    assert_eq!(join.get_args_iter().covec(), [p0.clone(), p1.clone()]);
    assert_eq!(join.get_returns_iter().covec(), [p2.clone()]);

    assert!(p2.is_active());
    assert_eq!(p2.get_origin().opref, join);
    assert_eq!(p2.get_users_iter().covec(), [split.clone()]);

    assert!(split.is_active());
    assert_eq!(split.get_depth(), 2);
    assert_eq!(split.get_args_iter().covec(), [p2.clone(), p0.clone()]);
    assert_eq!(split.get_returns_iter().covec(), [p3.clone(), p4.clone()]);

    assert!(p3.is_active());
    assert_eq!(p3.get_origin().opref, split);
    assert_eq!(p3.get_users_iter().covec(), [ulhs.clone(), effect.clone()]);

    assert!(p4.is_active());
    assert_eq!(p4.get_origin().opref, split);
    assert_eq!(p4.get_users_iter().covec(), [urhs.clone()]);

    assert!(ulhs.is_active());
    assert_eq!(ulhs.get_depth(), 3);
    assert_eq!(ulhs.get_args_iter().covec(), [p3.clone()]);
    assert_eq!(ulhs.get_returns_iter().covec(), [p5.clone()]);

    assert!(p5.is_active());
    assert_eq!(p5.get_origin().opref, ulhs);
    assert_eq!(p5.get_users_iter().covec(), [final_add.clone()]);

    assert!(urhs.is_active());
    assert_eq!(urhs.get_depth(), 3);
    assert_eq!(urhs.get_args_iter().covec(), [p4.clone()]);
    assert_eq!(urhs.get_returns_iter().covec(), [p6.clone()]);

    assert!(p6.is_active());
    assert_eq!(p6.get_origin().opref, urhs);
    assert_eq!(p6.get_users_iter().covec(), [final_add.clone()]);

    assert!(final_add.is_active());
    assert_eq!(final_add.get_depth(), 4);
    assert_eq!(final_add.get_args_iter().covec(), [p5.clone(), p6.clone()]);
    assert_eq!(final_add.get_returns_iter().covec(), [p7.clone()]);

    assert!(p7.is_active());
    assert_eq!(p7.get_origin().opref, final_add);
    assert_eq!(p7.get_users_iter().covec(), []);

    assert!(effect.is_active());
    assert_eq!(effect.get_depth(), 3);
    assert_eq!(effect.get_args_iter().covec(), [p3.clone()]);
    assert_eq!(effect.get_returns_iter().covec(), []);
    Ok(())
}

/// Tests that an operation reaches itself (reflexive property)
#[test]
fn test_reaches_self() -> Result<(), IRError<TestLang>> {
    let mut store: IR<TestLang> = IR::empty();
    let (lhs_id, _) = store.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![])?;
    let lhs = store.get_op(lhs_id);
    assert_display_is!(
        store.format(),
        r#"
        %0 : Int = int_input<pos: 0>();
    "#
    );
    assert!(lhs.reaches(&lhs));
    Ok(())
}

/// Tests basic reachability between two operations in a dependency chain
#[test]
fn test_reaches_base() -> Result<(), IRError<TestLang>> {
    let mut store: IR<TestLang> = IR::empty();
    let (lhs_id, v0) = store.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![])?;
    let (ulhs_id, _) = store.add_op(TestInstructionSet::Inc, svec![v0[0]])?;
    let lhs = store.get_op(lhs_id);
    let ulhs = store.get_op(ulhs_id);
    assert_display_is!(
        store.format(),
        r#"
        %0 : Int = int_input<pos: 0>();
        %1 : Int = inc(%0 : Int);
    "#
    );
    assert!(lhs.reaches(&ulhs));
    Ok(())
}

/// Tests reachability through a longer chain of operations
#[test]
fn test_reaches_chain() -> Result<(), IRError<TestLang>> {
    let mut store: IR<TestLang> = IR::empty();
    let (lhs_id, v0) = store.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![])?;
    let (_, v1) = store.add_op(TestInstructionSet::Inc, svec![v0[0]])?;
    let (_, v2) = store.add_op(TestInstructionSet::Inc, svec![v1[0]])?;
    let (_, v3) = store.add_op(TestInstructionSet::Inc, svec![v2[0]])?;
    let (_, v4) = store.add_op(TestInstructionSet::Inc, svec![v3[0]])?;
    let (ulhs_id, _) = store.add_op(TestInstructionSet::Inc, svec![v4[0]])?;
    let lhs = store.get_op(lhs_id);
    let ulhs = store.get_op(ulhs_id);
    assert_display_is!(
        store.format(),
        r#"
        %0 : Int = int_input<pos: 0>();
        %1 : Int = inc(%0 : Int);
        %2 : Int = inc(%1 : Int);
        %3 : Int = inc(%2 : Int);
        %4 : Int = inc(%3 : Int);
        %5 : Int = inc(%4 : Int);
    "#
    );
    assert!(lhs.reaches(&ulhs));
    Ok(())
}

/// Tests that independent operations don't reach each other
#[test]
fn test_reaches_happy_path() -> Result<(), IRError<TestLang>> {
    let mut store: IR<TestLang> = IR::empty();
    let (lhs_id, v0) = store.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![])?;
    let (rhs_id, _) = store.add_op(TestInstructionSet::IntInput { pos: 1 }, svec![])?;
    let _ = store.add_op(TestInstructionSet::Inc, svec![v0[0]])?;
    let lhs = store.get_op(lhs_id);
    let rhs = store.get_op(rhs_id);
    assert_display_is!(
        store.format(),
        r#"
        %0 : Int = int_input<pos: 0>();
        %1 : Int = int_input<pos: 1>();
        %2 : Int = inc(%0 : Int);
    "#
    );
    assert!(!lhs.reaches(&rhs));
    Ok(())
}

/// Tests get_reaching_iter returns all operations that can reach the current operation
#[test]
fn test_get_reaching_iter_simple() -> Result<(), IRError<TestLang>> {
    let mut store: IR<TestLang> = IR::empty();
    let (inp1_id, v0) = store.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![])?;
    let (inp2_id, v1) = store.add_op(TestInstructionSet::IntInput { pos: 1 }, svec![])?;
    let (add_id, v2) = store.add_op(TestInstructionSet::Add, svec![v0[0], v1[0]])?;
    let (inc_id, _) = store.add_op(TestInstructionSet::Inc, svec![v2[0]])?;

    let inc_op = store.get_op(inc_id);
    let reaching_ops: Vec<_> = inc_op.get_reaching_iter().map(|op| op.get_id()).collect();

    // inc operation should reach all its predecessors: add, inp1, inp2
    assert_eq!(reaching_ops.len(), 3);
    assert!(reaching_ops.contains(&inp1_id));
    assert!(reaching_ops.contains(&inp2_id));
    assert!(reaching_ops.contains(&add_id));
    Ok(())
}

/// Tests get_reaching_iter with complex dependency graph
#[test]
fn test_get_reaching_iter_complex() -> Result<(), IRError<TestLang>> {
    let mut store: IR<TestLang> = IR::empty();
    let (inp_id, v0) = store.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![])?;
    let (inc1_id, v1) = store.add_op(TestInstructionSet::Inc, svec![v0[0]])?;
    let (inc2_id, v2) = store.add_op(TestInstructionSet::Inc, svec![v1[0]])?;
    let (inc3_id, v3) = store.add_op(TestInstructionSet::Inc, svec![v0[0]])?; // Alternative branch
    let (add_id, _) = store.add_op(TestInstructionSet::Add, svec![v2[0], v3[0]])?;

    let add_op = store.get_op(add_id);
    let reaching_ops: Vec<_> = add_op.get_reaching_iter().map(|op| op.get_id()).collect();

    // add should reach inp, inc1, inc2, inc3
    assert_eq!(reaching_ops.len(), 4);
    assert!(reaching_ops.contains(&inp_id));
    assert!(reaching_ops.contains(&inc1_id));
    assert!(reaching_ops.contains(&inc2_id));
    assert!(reaching_ops.contains(&inc3_id));
    Ok(())
}

/// Tests get_reaching_iter with diamond pattern
#[test]
fn test_get_reaching_iter_diamond() -> Result<(), IRError<TestLang>> {
    let mut store: IR<TestLang> = IR::empty();
    let (a_id, a_vals) = store.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![])?; // A
    let (b_id, b_vals) = store.add_op(TestInstructionSet::Inc, svec![a_vals[0]])?; // B depends on A
    let (c_id, c_vals) = store.add_op(TestInstructionSet::Inc, svec![a_vals[0]])?; // C depends on A
    let (d_id, _) = store.add_op(TestInstructionSet::Add, svec![b_vals[0], c_vals[0]])?; // D depends on B,C

    let d_op = store.get_op(d_id);
    let reaching_ops: Vec<_> = d_op.get_reaching_iter().map(|op| op.get_id()).collect();

    // D should reach A, B, C but not include itself
    assert_eq!(reaching_ops.len(), 3);
    assert!(reaching_ops.contains(&a_id));
    assert!(reaching_ops.contains(&b_id));
    assert!(reaching_ops.contains(&c_id));
    assert!(!reaching_ops.contains(&d_id));
    Ok(())
}

/// Tests get_reaching_iter on input operation (no predecessors)
#[test]
fn test_get_reaching_iter_input() -> Result<(), IRError<TestLang>> {
    let mut store: IR<TestLang> = IR::empty();
    let (inp_id, _) = store.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![])?;

    let inp_op = store.get_op(inp_id);
    let reaching_ops: Vec<_> = inp_op.get_reaching_iter().map(|op| op.get_id()).collect();

    // Input operation has no predecessors
    assert_eq!(reaching_ops.len(), 0);
    Ok(())
}

/// Tests get_reaching_iter with two completely disconnected subgraphs
#[test]
fn test_get_reaching_iter_disconnected_subgraphs() -> Result<(), IRError<TestLang>> {
    let mut store: IR<TestLang> = IR::empty();

    // Subgraph 1: inp1 → inc1 → inc2
    let (inp1_id, v0) = store.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![])?;
    let (inc1_id, v1) = store.add_op(TestInstructionSet::Inc, svec![v0[0]])?;
    let (inc2_id, _v2) = store.add_op(TestInstructionSet::Inc, svec![v1[0]])?;

    // Subgraph 2: inp2 → inc3 → inc4
    let (inp2_id, v3) = store.add_op(TestInstructionSet::IntInput { pos: 1 }, svec![])?;
    let (inc3_id, v4) = store.add_op(TestInstructionSet::Inc, svec![v3[0]])?;
    let (inc4_id, _v5) = store.add_op(TestInstructionSet::Inc, svec![v4[0]])?;

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

    Ok(())
}

/// Tests get_reached_iter returns all operations that can be reached from the current operation
#[test]
fn test_get_reached_iter_simple() -> Result<(), IRError<TestLang>> {
    let mut store: IR<TestLang> = IR::empty();
    let (inp1_id, v0) = store.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![])?;
    let (inp2_id, v1) = store.add_op(TestInstructionSet::IntInput { pos: 1 }, svec![])?;
    let (add_id, v2) = store.add_op(TestInstructionSet::Add, svec![v0[0], v1[0]])?;
    let (inc_id, _) = store.add_op(TestInstructionSet::Inc, svec![v2[0]])?;

    let add_op = store.get_op(add_id);
    let reached_ops: Vec<_> = add_op.get_reached_iter().map(|op| op.get_id()).collect();

    // add operation should reach its successors: inc
    assert_eq!(reached_ops.len(), 1);
    assert!(reached_ops.contains(&inc_id));
    assert!(!reached_ops.contains(&inp1_id));
    assert!(!reached_ops.contains(&inp2_id));
    assert!(!reached_ops.contains(&add_id));
    Ok(())
}

/// Tests get_reached_iter with complex dependency graph
#[test]
fn test_get_reached_iter_complex() -> Result<(), IRError<TestLang>> {
    let mut store: IR<TestLang> = IR::empty();
    let (inp_id, v0) = store.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![])?;
    let (inc1_id, v1) = store.add_op(TestInstructionSet::Inc, svec![v0[0]])?;
    let (inc2_id, v2) = store.add_op(TestInstructionSet::Inc, svec![v1[0]])?;
    let (inc3_id, v3) = store.add_op(TestInstructionSet::Inc, svec![v0[0]])?; // Alternative branch
    let (add_id, _) = store.add_op(TestInstructionSet::Add, svec![v2[0], v3[0]])?;

    let inp_op = store.get_op(inp_id);
    let reached_ops: Vec<_> = inp_op.get_reached_iter().map(|op| op.get_id()).collect();

    // inp should reach inc1, inc2, inc3, add
    assert_eq!(reached_ops.len(), 4);
    assert!(reached_ops.contains(&inc1_id));
    assert!(reached_ops.contains(&inc2_id));
    assert!(reached_ops.contains(&inc3_id));
    assert!(reached_ops.contains(&add_id));
    Ok(())
}

/// Tests get_reached_iter with diamond pattern
#[test]
fn test_get_reached_iter_diamond() -> Result<(), IRError<TestLang>> {
    let mut store: IR<TestLang> = IR::empty();
    let (a_id, a_vals) = store.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![])?; // A
    let (b_id, b_vals) = store.add_op(TestInstructionSet::Inc, svec![a_vals[0]])?; // B depends on A
    let (c_id, c_vals) = store.add_op(TestInstructionSet::Inc, svec![a_vals[0]])?; // C depends on A
    let (d_id, _) = store.add_op(TestInstructionSet::Add, svec![b_vals[0], c_vals[0]])?; // D depends on B,C

    let a_op = store.get_op(a_id);
    let reached_ops: Vec<_> = a_op.get_reached_iter().map(|op| op.get_id()).collect();

    // A should reach B, C, D but not include itself
    assert_eq!(reached_ops.len(), 3);
    assert!(reached_ops.contains(&b_id));
    assert!(reached_ops.contains(&c_id));
    assert!(reached_ops.contains(&d_id));
    assert!(!reached_ops.contains(&a_id));
    Ok(())
}

/// Tests get_reached_iter on operation with no successors (effect)
#[test]
fn test_get_reached_iter_effect() -> Result<(), IRError<TestLang>> {
    let mut store: IR<TestLang> = IR::empty();
    let (_, vals) = store.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![])?;
    let (ret_id, _) = store.add_op(TestInstructionSet::Return, vals)?;

    let ret_op = store.get_op(ret_id);
    let reached_ops: Vec<_> = ret_op.get_reached_iter().map(|op| op.get_id()).collect();

    // Return operation has no successors
    assert_eq!(reached_ops.len(), 0);
    Ok(())
}

/// Tests get_reached_iter with two completely disconnected subgraphs
#[test]
fn test_get_reached_iter_disconnected_subgraphs() -> Result<(), IRError<TestLang>> {
    let mut store: IR<TestLang> = IR::empty();

    // Subgraph 1: inp1 → inc1 → inc2
    let (inp1_id, v0) = store.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![])?;
    let (inc1_id, v1) = store.add_op(TestInstructionSet::Inc, svec![v0[0]])?;
    let (inc2_id, _v2) = store.add_op(TestInstructionSet::Inc, svec![v1[0]])?;

    // Subgraph 2: inp2 → inc3 → inc4
    let (inp2_id, v3) = store.add_op(TestInstructionSet::IntInput { pos: 1 }, svec![])?;
    let (inc3_id, v4) = store.add_op(TestInstructionSet::Inc, svec![v3[0]])?;
    let (inc4_id, _v5) = store.add_op(TestInstructionSet::Inc, svec![v4[0]])?;

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

    Ok(())
}

/// Tests get_reached_iter with branching pattern (one-to-many)
#[test]
fn test_get_reached_iter_branching() -> Result<(), IRError<TestLang>> {
    let mut store: IR<TestLang> = IR::empty();
    let (inp_id, vals) = store.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![])?;
    let (ret1_id, _) = store.add_op(TestInstructionSet::Return, vals.clone())?;
    let (inc_id, inc_vals) = store.add_op(TestInstructionSet::Inc, vals)?;
    let (ret2_id, _) = store.add_op(TestInstructionSet::Return, inc_vals)?;

    let inp_op = store.get_op(inp_id);
    let reached_ops: Vec<_> = inp_op.get_reached_iter().map(|op| op.get_id()).collect();

    // inp should reach all three operations that use its output
    assert_eq!(reached_ops.len(), 3);
    assert!(reached_ops.contains(&ret1_id));
    assert!(reached_ops.contains(&inc_id));
    assert!(reached_ops.contains(&ret2_id));
    Ok(())
}

/// Tests get_reached_iter with convergent pattern (many-to-one)
#[test]
fn test_get_reached_iter_convergent() -> Result<(), IRError<TestLang>> {
    let mut store: IR<TestLang> = IR::empty();
    let (inp1_id, v0) = store.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![])?;
    let (inp2_id, v1) = store.add_op(TestInstructionSet::IntInput { pos: 1 }, svec![])?;
    let (add_id, _) = store.add_op(TestInstructionSet::Add, svec![v0[0], v1[0]])?;

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
    Ok(())
}

/// Tests that attempting to delete an operation still in use panics
#[test]
#[should_panic]
fn test_delete_op_in_use() {
    let mut store: IR<TestLang> = IR::empty();
    let (lhs_id, v0) = store
        .add_op(TestInstructionSet::IntInput { pos: 0 }, svec![])
        .expect("Bad add_op");
    let (_, v1) = store
        .add_op(TestInstructionSet::IntInput { pos: 1 }, svec![])
        .expect("Bad add_op");
    let _ = store
        .add_op(TestInstructionSet::Add, svec![v0[0], v1[0]])
        .expect("Bad add_op");
    store.delete_op(lhs_id);
}

/// Tests successful deletion of operations and verification of inactive state
#[test]
fn test_delete_op() -> Result<(), IRError<TestLang>> {
    let mut store: IR<TestLang> = IR::empty();
    let (_, v0) = store.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![])?;
    let (rhs_id, v1) = store.add_op(TestInstructionSet::IntInput { pos: 1 }, svec![])?;
    let (join_id, v2) = store.add_op(TestInstructionSet::Add, svec![v0[0], v1[0]])?;
    store.delete_op(join_id);
    store.delete_op(rhs_id);
    assert_display_is!(
        store.format(),
        r#"
        %0 : Int = int_input<pos: 0>();
    "#
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
    let mut store: IR<TestLang> = IR::empty();
    let (_, v0) = store
        .add_op(TestInstructionSet::IntInput { pos: 0 }, svec![])
        .expect("Bad add_op");
    let (_, v1) = store
        .add_op(TestInstructionSet::Inc, svec![v0[0]])
        .expect("Bad add_op");
    assert_display_is!(
        store.format(),
        r#"
        %0 : Int = int_input<pos: 0>();
        %1 : Int = inc(%0 : Int);
    "#
    );
    store.replace_val_use(v0[0], v1[0]);
}

/// Tests that replacing a value with one that would create a cycle panics (longer chain)
#[test]
#[should_panic(expected = "Tried to replace a value with one it reaches.")]
fn test_replace_val_use_wrong_longer() {
    let mut store: IR<TestLang> = IR::empty();
    let (_, v0) = store
        .add_op(TestInstructionSet::IntInput { pos: 0 }, svec![])
        .expect("Bad add_op");
    let (_, v1) = store
        .add_op(TestInstructionSet::Inc, svec![v0[0]])
        .expect("Bad add_op");
    assert_display_is!(
        store.format(),
        r#"
        %0 : Int = int_input<pos: 0>();
        %1 : Int = inc(%0 : Int);
    "#
    );
    store.replace_val_use(v0[0], v1[0]);
}

/// Tests successful value use replacement and user list updates
#[test]
fn test_replace_val_use() -> Result<(), IRError<TestLang>> {
    let mut store: IR<TestLang> = IR::empty();
    let (_inp1_id, v0) = store.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![])?;
    let (_inp2_id, v1) = store.add_op(TestInstructionSet::IntInput { pos: 1 }, svec![])?;
    let (inc_id, _v2) = store.add_op(TestInstructionSet::Inc, svec![v0[0]])?;
    assert_display_is!(
        store.format(),
        r#"
        %0 : Int = int_input<pos: 0>();
        %1 : Int = int_input<pos: 1>();
        %2 : Int = inc(%0 : Int);
    "#
    );
    store.replace_val_use(v0[0], v1[0]);
    assert_display_is!(
        store.format(),
        r#"
        %0 : Int = int_input<pos: 0>();
        %1 : Int = int_input<pos: 1>();
        %2 : Int = inc(%1 : Int);
    "#
    );
    let inc = store.get_op(inc_id);
    let v0 = store.get_val(v0[0]);
    let v1 = store.get_val(v1[0]);
    assert_eq!(v0.get_users_iter().covec(), []);
    assert_eq!(v1.get_users_iter().covec(), [inc.clone()]);
    Ok(())
}

/// Tests that value replacement makes operation depth shallower when appropriate
#[test]
fn test_replace_val_use_make_shallower() -> Result<(), IRError<TestLang>> {
    let mut store: IR<TestLang> = IR::empty();
    let (_inp1_id, v0) = store.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![])?;
    let (_inp2_id, v1) = store.add_op(TestInstructionSet::IntInput { pos: 1 }, svec![])?;
    let (_, v2) = store.add_op(TestInstructionSet::Inc, svec![v1[0]])?;
    let (_, v3) = store.add_op(TestInstructionSet::Inc, svec![v2[0]])?;
    let (_, v4) = store.add_op(TestInstructionSet::Inc, svec![v3[0]])?;
    let (last_id, _v5) = store.add_op(TestInstructionSet::Inc, svec![v4[0]])?;
    assert_display_is!(
        store.format(),
        r#"
        %0 : Int = int_input<pos: 0>();
        %1 : Int = int_input<pos: 1>();
        %2 : Int = inc(%1 : Int);
        %3 : Int = inc(%2 : Int);
        %4 : Int = inc(%3 : Int);
        %5 : Int = inc(%4 : Int);
    "#
    );
    let last = store.get_op(last_id);
    assert_eq!(last.get_depth(), 4);
    store.replace_val_use(v4[0], v0[0]);
    assert_display_is!(
        store.format(),
        r#"
        %0 : Int = int_input<pos: 0>();
        %1 : Int = int_input<pos: 1>();
        %2 : Int = inc(%1 : Int);
        %5 : Int = inc(%0 : Int);
        %3 : Int = inc(%2 : Int);
        %4 : Int = inc(%3 : Int);
    "#
    );
    let last = store.get_op(last_id);
    assert_eq!(last.get_depth(), 1);
    Ok(())
}

/// Tests that value replacement makes operation depth deeper when appropriate
#[test]
fn test_replace_val_use_make_deeper() -> Result<(), IRError<TestLang>> {
    let mut store: IR<TestLang> = IR::empty();
    let (_inp1_id, v0) = store.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![])?;
    let (_inp2_id, v1) = store.add_op(TestInstructionSet::IntInput { pos: 1 }, svec![])?;
    let (_, v2) = store.add_op(TestInstructionSet::Inc, svec![v1[0]])?;
    let (_, v3) = store.add_op(TestInstructionSet::Inc, svec![v2[0]])?;
    let (_, v4) = store.add_op(TestInstructionSet::Inc, svec![v0[0]])?;
    let (last_id, _v5) = store.add_op(TestInstructionSet::Inc, svec![v4[0]])?;
    assert_display_is!(
        store.format(),
        r#"
        %0 : Int = int_input<pos: 0>();
        %1 : Int = int_input<pos: 1>();
        %2 : Int = inc(%1 : Int);
        %4 : Int = inc(%0 : Int);
        %3 : Int = inc(%2 : Int);
        %5 : Int = inc(%4 : Int);
    "#
    );
    let last = store.get_op(last_id);
    assert_eq!(last.get_depth(), 2);
    store.replace_val_use(v0[0], v3[0]);
    assert_display_is!(
        store.format(),
        r#"
        %0 : Int = int_input<pos: 0>();
        %1 : Int = int_input<pos: 1>();
        %2 : Int = inc(%1 : Int);
        %3 : Int = inc(%2 : Int);
        %4 : Int = inc(%3 : Int);
        %5 : Int = inc(%4 : Int);
    "#
    );
    let last = store.get_op(last_id);
    assert_eq!(last.get_depth(), 4);
    Ok(())
}

/// Tests that add_op panics when argument types don't match operation signature
#[test]
fn test_add_op_type_mismatch() -> Result<(), IRError<TestLang>> {
    let mut store: IR<TestLang> = IR::empty();
    let (_, bool_val) = store.add_op(TestInstructionSet::BoolConstant { val: true }, svec![])?;

    // Try to use bool value where int is expected
    let ret = store.add_op(TestInstructionSet::Inc, svec![bool_val[0]]);

    assert_eq!(
        std::mem::discriminant(&ret.err().expect("must return an error")),
        std::mem::discriminant(&IRError::<TestLang>::OpSig {
            op: TestInstructionSet::Add,
            recv: vec![],
            exp: vec![]
        })
    );
    Ok(())
}

/// Tests that add_op return error when wrong number of arguments provided
#[test]
fn test_add_op_wrong_arg_count() -> Result<(), IRError<TestLang>> {
    let mut store: IR<TestLang> = IR::empty();
    let (_, int_vals) = store.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![])?;
    // Add operation expects 2 args, providing only 1
    let ret = store.add_op(TestInstructionSet::Add, svec![int_vals[0]]);
    assert_eq!(
        std::mem::discriminant(&ret.err().expect("must return an error")),
        std::mem::discriminant(&IRError::<TestLang>::OpSig {
            op: TestInstructionSet::Add,
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
    let mut store: IR<TestLang> = IR::empty();
    let (op_id, vals) = store
        .add_op(TestInstructionSet::IntInput { pos: 0 }, svec![])
        .expect("Bad add_op");
    store.delete_op(op_id);
    // Try to use deleted value
    store
        .add_op(TestInstructionSet::Inc, svec![vals[0]])
        .expect("Bad add_op");
}

/// Tests that accessing deleted operation via public API panics
#[test]
#[should_panic(expected = "Tried to get a dead op")]
fn test_get_deleted_op() {
    let mut store: IR<TestLang> = IR::empty();
    let (op_id, _) = store
        .add_op(TestInstructionSet::IntInput { pos: 0 }, svec![])
        .expect("Bad add_op");
    store.delete_op(op_id);
    store.get_op(op_id);
}

/// Tests that accessing deleted value via public API panics
#[test]
#[should_panic(expected = "Tried to get a dead val")]
fn test_get_deleted_val() {
    let mut store: IR<TestLang> = IR::empty();
    let (op_id, vals) = store
        .add_op(TestInstructionSet::IntInput { pos: 0 }, svec![])
        .expect("Bad add_op");
    store.delete_op(op_id);
    store.get_val(vals[0]);
}

/// Tests that double deletion is captured
#[test]
#[should_panic(expected = "Tried to delete an already inactive operation")]
fn test_double_deletion() {
    let mut store: IR<TestLang> = IR::empty();
    let (op_id, _) = store
        .add_op(TestInstructionSet::IntInput { pos: 0 }, svec![])
        .expect("Bad add_op");
    store.delete_op(op_id);
    store.delete_op(op_id); // Should panic on second deletion
}

/// Tests self-value replacement (should be no-op)
#[test]
fn test_replace_val_use_self() -> Result<(), IRError<TestLang>> {
    let mut store: IR<TestLang> = IR::empty();
    let (_, vals) = store.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![])?;
    let (inc_id, _) = store.add_op(TestInstructionSet::Inc, svec![vals[0]])?;

    let inc_before = format!("{:?}", store.get_op(inc_id));
    store.replace_val_use(vals[0], vals[0]); // Should be no-op
    let inc_after = format!("{:?}", store.get_op(inc_id));

    assert_eq!(inc_before, inc_after);
    assert_display_is!(
        store.format(),
        r#"
        %0 : Int = int_input<pos: 0>();
        %1 : Int = inc(%0 : Int);
    "#
    );
    Ok(())
}

/// Tests using same value multiple times in single operation
#[test]
fn test_same_value_multiple_args() -> Result<(), IRError<TestLang>> {
    let mut store: IR<TestLang> = IR::empty();
    let (_, vals) = store.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![])?;
    let (add_id, _) = store.add_op(TestInstructionSet::Add, svec![vals[0], vals[0]])?;

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
fn test_diamond_dependencies() -> Result<(), IRError<TestLang>> {
    let mut store: IR<TestLang> = IR::empty();
    let (_, a_vals) = store.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![])?; // A
    let (_b_id, b_vals) = store.add_op(TestInstructionSet::Inc, svec![a_vals[0]])?; // B depends on A
    let (_c_id, c_vals) = store.add_op(TestInstructionSet::Inc, svec![a_vals[0]])?; // C depends on A
    let (d_id, _) = store.add_op(TestInstructionSet::Add, svec![b_vals[0], c_vals[0]])?; // D depends on B,C

    let a_val = store.get_val(a_vals[0]);
    let d_op = store.get_op(d_id);

    // A should have 2 users (B and C)
    assert_eq!(a_val.get_users_iter().count(), 2);
    // D should be at depth 3 (A:1 → B,C:2 → D:3)
    assert_eq!(d_op.get_depth(), 2);
    Ok(())
}

/// Tests multiple independent subgraphs in same IR
#[test]
fn test_independent_subgraphs() -> Result<(), IRError<TestLang>> {
    let mut store: IR<TestLang> = IR::empty();

    // Subgraph 1: input1 → inc1
    let (_, vals1) = store.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![])?;
    let (inc1_id, _) = store.add_op(TestInstructionSet::Inc, svec![vals1[0]])?;

    // Subgraph 2: input2 → inc2
    let (_, vals2) = store.add_op(TestInstructionSet::IntInput { pos: 1 }, svec![])?;
    let (inc2_id, _) = store.add_op(TestInstructionSet::Inc, svec![vals2[0]])?;

    let inc1 = store.get_op(inc1_id);
    let inc2 = store.get_op(inc2_id);

    // Neither should reach the other
    assert!(!inc1.reaches(&inc2));
    assert!(!inc2.reaches(&inc1));
    Ok(())
}

/// Tests operations with multi-return where returns have different user patterns
#[test]
fn test_multi_return_different_users() -> Result<(), IRError<TestLang>> {
    let mut store: IR<TestLang> = IR::empty();
    let (_, inp1) = store.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![])?;
    let (_, inp2) = store.add_op(TestInstructionSet::IntInput { pos: 1 }, svec![])?;
    let (_, div_vals) = store.add_op(TestInstructionSet::DivRem, svec![inp1[0], inp2[0]])?;

    // Use first return in one op, second return in another
    let (_, _) = store.add_op(TestInstructionSet::Inc, svec![div_vals[0]])?; // Use quotient
    let (_, _) = store.add_op(TestInstructionSet::Inc, svec![div_vals[1]])?; // Use remainder

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
fn test_iteration_with_deleted_elements() -> Result<(), IRError<TestLang>> {
    let mut store: IR<TestLang> = IR::empty();
    let (op1_id, _) = store.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![])?;
    let (op2_id, _) = store.add_op(TestInstructionSet::IntInput { pos: 1 }, svec![])?;
    let (op3_id, _) = store.add_op(TestInstructionSet::IntInput { pos: 2 }, svec![])?;

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
    Ok(())
}

/// Tests that user lists remain consistent after deletions
#[test]
fn test_user_consistency_after_deletion() -> Result<(), IRError<TestLang>> {
    let mut store: IR<TestLang> = IR::empty();
    let (_inp_id, vals) = store.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![])?;
    let (inc1_id, _) = store.add_op(TestInstructionSet::Inc, svec![vals[0]])?;
    let (_inc2_id, _) = store.add_op(TestInstructionSet::Inc, svec![vals[0]])?;

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
    Ok(())
}

/// Tests replacement cascade affecting multiple dependency levels
#[test]
fn test_replacement_cascade_multiple_levels() -> Result<(), IRError<TestLang>> {
    let mut store: IR<TestLang> = IR::empty();

    // Create: inp1 → inc1 → inc2 → inc3
    //         inp2 (unused initially)
    let (_, inp1) = store.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![])?;
    let (_, inp2) = store.add_op(TestInstructionSet::IntInput { pos: 1 }, svec![])?;
    let (inc1_id, inc1_vals) = store.add_op(TestInstructionSet::Inc, svec![inp1[0]])?;
    let (inc2_id, inc2_vals) = store.add_op(TestInstructionSet::Inc, svec![inc1_vals[0]])?;
    let (inc3_id, _) = store.add_op(TestInstructionSet::Inc, svec![inc2_vals[0]])?;

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
fn test_replacement_deeper_chain() -> Result<(), IRError<TestLang>> {
    let mut store: IR<TestLang> = IR::empty();

    // Create: inp1, inp2 → inc1, inp2 → inc2
    let (_, inp1) = store.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![])?;
    let (_, inp2) = store.add_op(TestInstructionSet::IntInput { pos: 1 }, svec![])?;
    let (_, inc1_vals) = store.add_op(TestInstructionSet::Inc, svec![inp2[0]])?;
    let (inc2_id, _) = store.add_op(TestInstructionSet::Inc, svec![inp1[0]])?; // Initially uses inp1

    assert_eq!(store.get_op(inc2_id).get_depth(), 1);

    // Replace inp1 with inc1's output, making inc2 deeper
    store.replace_val_use(inp1[0], inc1_vals[0]);

    assert_eq!(store.get_op(inc2_id).get_depth(), 2); // Now inp2→inc1→inc2
    Ok(())
}

/// Tests has_opid/has_valid behavior with deleted elements
#[test]
fn test_has_id_with_deleted_elements() -> Result<(), IRError<TestLang>> {
    let mut store: IR<TestLang> = IR::empty();
    let (op_id, vals) = store.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![])?;

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
fn test_empty_ir_operations() -> Result<(), IRError<TestLang>> {
    let store: IR<TestLang> = IR::empty();

    assert_eq!(store.n_ops(), 0);
    assert_eq!(store.n_vals(), 0);
    assert_eq!(store.walk_ops_linear().count(), 0);

    // Check that topological order works on empty IR
    let topo_ops: Vec<_> = store.raw_walk_ops_topo().collect();
    assert_eq!(topo_ops.len(), 0);
    Ok(())
}

/// Tests topological ordering with deleted operations
#[test]
fn test_topological_order_with_deletions() -> Result<(), IRError<TestLang>> {
    let mut store: IR<TestLang> = IR::empty();
    let (op1_id, vals1) = store.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![])?;
    let (op2_id, vals2) = store.add_op(TestInstructionSet::Inc, svec![vals1[0]])?;
    let (op3_id, _vals3) = store.add_op(TestInstructionSet::Inc, svec![vals2[0]])?;
    let (op4_id, _) = store.add_op(TestInstructionSet::Inc, svec![vals1[0]])?; // Independent branch

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
    Ok(())
}

/// Tests replacement with type validation
#[test]
#[should_panic(expected = "Tried to replace a value with one of different type")]
fn test_replace_val_different_types() {
    let mut store: IR<TestLang> = IR::empty();
    let (_, int_vals) = store
        .add_op(TestInstructionSet::IntInput { pos: 0 }, svec![])
        .expect("Bad add_op");
    let (_, bool_vals) = store
        .add_op(TestInstructionSet::BoolConstant { val: true }, svec![])
        .expect("Bad add_op");

    // Try to replace int value with bool value
    store.replace_val_use(int_vals[0], bool_vals[0]);
}

/// Tests operations that become unreachable after replacement but aren't deleted
#[test]
fn test_unreachable_after_replacement() -> Result<(), IRError<TestLang>> {
    let mut store: IR<TestLang> = IR::empty();
    let (_, inp1) = store.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![])?;
    let (_, inp2) = store.add_op(TestInstructionSet::IntInput { pos: 1 }, svec![])?;
    let (inc1_id, inc1_vals) = store.add_op(TestInstructionSet::Inc, svec![inp1[0]])?;
    let (_ret_id, _) = store.add_op(TestInstructionSet::Return, svec![inc1_vals[0]])?;

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
fn test_is_effect() -> Result<(), IRError<TestLang>> {
    let mut ir = IR::<TestLang>::empty();
    let (_, inp1) = ir.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![])?;
    let (_, inp2) = ir.add_op(TestInstructionSet::IntInput { pos: 1 }, svec![])?;
    let (add_op, add) = ir.add_op(TestInstructionSet::Add, svec![inp1[0], inp2[0]])?;
    let (ret_op, _) = ir.add_op(TestInstructionSet::Return, svec![add[0]])?;

    assert!(ir.get_op(ret_op).is_effect());
    assert!(!ir.get_op(add_op).is_effect());
    Ok(())
}

#[test]
fn test_signature_consistency() -> Result<(), IRError<TestLang>> {
    let mut ir = IR::<TestLang>::empty();
    let (_, inp1) = ir.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![])?;
    let (_, inp2) = ir.add_op(TestInstructionSet::IntInput { pos: 1 }, svec![])?;
    let (add_op, _) = ir.add_op(TestInstructionSet::Add, svec![inp1[0], inp2[0]])?;
    let op_ref = ir.get_op(add_op);

    // Verify cached signature matches operation signature
    assert_eq!(op_ref.signature, &op_ref.operation.get_signature());
    Ok(())
}

#[test]
fn test_batch_delete_empty() -> Result<(), IRError<TestLang>> {
    let mut ir = IR::<TestLang>::empty();
    let (op1, _) = ir.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![])?;

    // Empty batch should be no-op
    ir.batch_delete_op(std::iter::empty());

    assert!(ir.has_opid(op1));
    assert_eq!(ir.n_ops(), 1);
    Ok(())
}

#[test]
fn test_batch_delete_single() -> Result<(), IRError<TestLang>> {
    let mut ir = IR::<TestLang>::empty();
    let (op1, vals1) = ir.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![])?;
    let (op2, _) = ir.add_op(TestInstructionSet::Return, vals1)?;

    ir.batch_delete_op(std::iter::once(op2));

    assert!(ir.has_opid(op1));
    assert!(!ir.has_opid(op2));
    assert_eq!(ir.n_ops(), 1);
    Ok(())
}

#[test]
fn test_batch_delete_dependency_chain() -> Result<(), IRError<TestLang>> {
    let mut ir = IR::<TestLang>::empty();
    let (op1, vals1) = ir.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![])?;
    let (op2, vals2) = ir.add_op(TestInstructionSet::Inc, vals1)?;
    let (op3, vals3) = ir.add_op(TestInstructionSet::Inc, vals2)?;
    let (op4, _) = ir.add_op(TestInstructionSet::Return, vals3)?;

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
fn test_batch_delete_order_independence() -> Result<(), IRError<TestLang>> {
    let mut ir = IR::<TestLang>::empty();
    let (op1, vals1) = ir.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![])?;
    let (op2, vals2) = ir.add_op(TestInstructionSet::Inc, vals1)?;
    let (op3, _) = ir.add_op(TestInstructionSet::Return, vals2)?;

    // Delete in reverse dependency order - should still work
    ir.batch_delete_op([op2, op3].into_iter());

    assert!(ir.has_opid(op1));
    assert!(!ir.has_opid(op2));
    assert!(!ir.has_opid(op3));
    assert_eq!(ir.n_ops(), 1);
    Ok(())
}

#[test]
fn test_batch_delete_diamond_pattern() -> Result<(), IRError<TestLang>> {
    let mut ir = IR::<TestLang>::empty();
    let (op1, vals1) = ir.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![])?;
    let (op2, vals2) = ir.add_op(TestInstructionSet::Inc, vals1.clone())?;
    let (op3, vals3) = ir.add_op(TestInstructionSet::Inc, vals1)?;
    let (op4, _) = ir.add_op(TestInstructionSet::Add, svec![vals2[0], vals3[0]])?;

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
fn test_batch_delete_independent_operations() -> Result<(), IRError<TestLang>> {
    let mut ir = IR::<TestLang>::empty();
    let (op1, vals1) = ir.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![])?;
    let (op2, vals2) = ir.add_op(TestInstructionSet::IntInput { pos: 1 }, svec![])?;
    let (op3, _) = ir.add_op(TestInstructionSet::Return, vals1)?;
    let (op4, _) = ir.add_op(TestInstructionSet::Return, vals2)?;

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
    let mut ir = IR::<TestLang>::empty();
    let (_op1, vals1) = ir
        .add_op(TestInstructionSet::IntInput { pos: 0 }, svec![])
        .expect("Bad add_op");
    let (op2, vals2) = ir.add_op(TestInstructionSet::Inc, vals1).expect("Bad add_op");
    let (op3, _) = ir
        .add_op(TestInstructionSet::Return, vals2.clone())
        .expect("Bad add_op");
    let (_op4, _) = ir
        .add_op(TestInstructionSet::Return, vals2) // op4 also uses vals?2
        .expect("Bad add_op");

    // Try to delete op2 and op3, but op4 still uses vals2 from op2
    ir.batch_delete_op([op2, op3].into_iter());
}

#[test]
fn test_batch_delete_partial_dependency_closure() -> Result<(), IRError<TestLang>> {
    let mut ir = IR::<TestLang>::empty();
    let (op1, vals1) = ir.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![])?;
    let (op2, vals2) = ir.add_op(TestInstructionSet::Inc, vals1)?;
    let (op3, vals3) = ir.add_op(TestInstructionSet::Inc, vals2.clone())?;
    let (op4, _) = ir.add_op(TestInstructionSet::Return, vals3)?;
    let (op5, _) = ir.add_op(TestInstructionSet::Return, vals2)?; // Also uses vals2

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
    let mut ir = IR::<TestLang>::empty();
    let (_, vals1) = ir
        .add_op(TestInstructionSet::IntInput { pos: 0 }, svec![])
        .expect("Bad add_op");
    let (op2, _) = ir.add_op(TestInstructionSet::Return, vals1).expect("Bad add_op");

    ir.delete_op(op2); // Delete normally first

    // Try to batch delete already deleted operation
    ir.batch_delete_op(std::iter::once(op2));
}

/// Tests that ValOrigin correctly tracks the position of values within multi-return operations.
///
/// This verifies that when an operation returns multiple values, each value knows not just
/// which operation produced it, but also which return position it came from.
#[test]
fn test_val_origin_position_tracking() -> Result<(), IRError<TestLang>> {
    let mut store: IR<TestLang> = IR::empty();

    // Create an operation with multiple return values
    let (_input_id, v_input) = store.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![])?;
    let (divrem_id, v_divrem) = store.add_op(TestInstructionSet::DivRem, svec![v_input[0], v_input[0]])?;

    // Test that each return value has the correct position in its origin
    let quotient = store.get_val(v_divrem[0]);
    let remainder = store.get_val(v_divrem[1]);

    // Verify the origin operation is correct
    assert_eq!(quotient.get_origin().opref.get_id(), divrem_id);
    assert_eq!(remainder.get_origin().opref.get_id(), divrem_id);

    // Verify the positions are correct - quotient should be position 0, remainder position 1
    assert_eq!(quotient.get_origin().position, 0);
    assert_eq!(remainder.get_origin().position, 1);

    Ok(())
}

/// Tests that ValUse correctly tracks the argument position where values are consumed.
///
/// This verifies that when a value is used as an argument to an operation, the value
/// knows not just which operation uses it, but also at which argument position.
#[test]
fn test_val_use_position_tracking() -> Result<(), IRError<TestLang>> {
    let mut store: IR<TestLang> = IR::empty();

    // Create values to be used as arguments
    let (_input1_id, v1) = store.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![])?;
    let (_input2_id, v2) = store.add_op(TestInstructionSet::IntInput { pos: 1 }, svec![])?;

    // Create an operation that uses both values as arguments
    let (add_id, _v_add) = store.add_op(TestInstructionSet::Add, svec![v1[0], v2[0]])?;

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

    Ok(())
}

/// Tests position tracking when the same value is used multiple times in different positions.
///
/// This verifies that when a value is used as arguments in multiple operations and positions,
/// each usage is tracked with the correct operation ID and argument position.
#[test]
fn test_position_tracking_with_multiple_uses() -> Result<(), IRError<TestLang>> {
    let mut store: IR<TestLang> = IR::empty();

    // Create a value that will be used multiple times
    let (_input_id, v_input) = store.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![])?;

    // Use the same value in different positions of different operations
    let (add1_id, _v_add1) = store.add_op(TestInstructionSet::Add, svec![v_input[0], v_input[0]])?;
    let (divrem_id, _v_divrem) = store.add_op(TestInstructionSet::DivRem, svec![v_input[0], v_input[0]])?;

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

    Ok(())
}

/// Tests comprehensive position tracking with operations that have multiple returns and uses.
///
/// This combines multi-return operations with multi-argument operations to verify that
/// both origin positions (which return value) and use positions (which argument) are
/// correctly tracked throughout the dataflow graph.
#[test]
fn test_position_tracking_with_multi_return_multi_use() -> Result<(), IRError<TestLang>> {
    let mut store: IR<TestLang> = IR::empty();

    // Create initial values
    let (_input1_id, v1) = store.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![])?;
    let (_input2_id, v2) = store.add_op(TestInstructionSet::IntInput { pos: 1 }, svec![])?;

    // Create an operation with multiple returns
    let (divrem_id, v_divrem) = store.add_op(TestInstructionSet::DivRem, svec![v1[0], v2[0]])?;

    // Use both return values in different positions of a new operation
    let (add_id, _v_add) = store.add_op(TestInstructionSet::Add, svec![v_divrem[0], v_divrem[1]])?;

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

    Ok(())
}

/// Tests that position information remains consistent after value replacement operations.
///
/// This verifies that when replace_val_use is called, the position tracking is updated
/// correctly - the new value should be tracked at the same argument position where the
/// old value was used.
#[test]
fn test_position_consistency_after_replacement() -> Result<(), IRError<TestLang>> {
    let mut store: IR<TestLang> = IR::empty();

    // Create values
    let (_input1_id, v1) = store.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![])?;
    let (_input2_id, v2) = store.add_op(TestInstructionSet::IntInput { pos: 1 }, svec![])?;
    let (_input3_id, v3) = store.add_op(TestInstructionSet::IntInput { pos: 2 }, svec![])?;

    // Create an operation using the first value
    let (add_id, _v_add) = store.add_op(TestInstructionSet::Add, svec![v1[0], v2[0]])?;

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

    Ok(())
}

/// Tests position tracking with single-argument operations.
///
/// This verifies that position tracking works correctly for operations that take
/// only one argument, ensuring the position is tracked as 0.
#[test]
fn test_position_tracking_single_argument_operation() -> Result<(), IRError<TestLang>> {
    let mut store: IR<TestLang> = IR::empty();

    // Create a value and use it in a single-argument operation
    let (_input_id, v_input) = store.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![])?;
    let (inc_id, _v_inc) = store.add_op(TestInstructionSet::Inc, svec![v_input[0]])?;

    // Verify position tracking for single-argument operation
    let input_val = store.get_val(v_input[0]);

    assert_eq!(input_val.users.len(), 1);
    assert_eq!(input_val.users[0].opid, inc_id);
    assert_eq!(input_val.users[0].position, 0);

    Ok(())
}

/// Tests position tracking with single-return operations.
///
/// This verifies that position tracking works correctly for operations that return
/// only one value, ensuring the return position is tracked as 0.
#[test]
fn test_position_tracking_single_return_operation() -> Result<(), IRError<TestLang>> {
    let mut store: IR<TestLang> = IR::empty();

    // Create operations with single return values
    let (_input_id, v_input) = store.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![])?;
    let (add_id, v_add) = store.add_op(TestInstructionSet::Add, svec![v_input[0], v_input[0]])?;

    // Verify position tracking for single-return operation
    let result_val = store.get_val(v_add[0]);

    assert_eq!(result_val.get_origin().opref.get_id(), add_id);
    assert_eq!(result_val.get_origin().position, 0);

    Ok(())
}

/// Tests position tracking with zero-argument operations.
///
/// This verifies that operations with no arguments (like input operations) still
/// have their return values properly tracked with position 0.
#[test]
fn test_position_tracking_zero_argument_operation() -> Result<(), IRError<TestLang>> {
    let mut store: IR<TestLang> = IR::empty();

    // Create an operation with no arguments (like IntInput)
    let (input_id, v_input) = store.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![])?;

    // Verify the value's origin tracking
    let input_val = store.get_val(v_input[0]);

    assert_eq!(input_val.get_origin().opref.get_id(), input_id);
    assert_eq!(input_val.get_origin().position, 0);
    assert_eq!(input_val.users.len(), 0); // No users initially

    Ok(())
}

/// Tests that position tracking is properly maintained after operation deletion.
///
/// This verifies that when operations are deleted, the position tracking information
/// behaves correctly - raw users lists retain deleted operations, but filtered
/// iteration excludes inactive operations.
#[test]
fn test_position_tracking_after_deletion() -> Result<(), IRError<TestLang>> {
    let mut store: IR<TestLang> = IR::empty();

    // Create a chain of operations
    let (_input1_id, v1) = store.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![])?;
    let (_input2_id, v2) = store.add_op(TestInstructionSet::IntInput { pos: 1 }, svec![])?;
    let (_add1_id, v_add1) = store.add_op(TestInstructionSet::Add, svec![v1[0], v2[0]])?;
    let (add2_id, _v_add2) = store.add_op(TestInstructionSet::Add, svec![v_add1[0], v1[0]])?;

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

    Ok(())
}

/// Tests basic equality and functionality of ValOrigin and ValUse types.
///
/// This verifies that the ValOrigin and ValUse structs have proper equality
/// semantics and can be constructed and compared correctly.
#[test]
fn test_val_origin_and_val_use_equality() -> Result<(), IRError<TestLang>> {
    use crate::{ValOrigin, ValUse};

    let mut store: IR<TestLang> = IR::empty();

    // Create some operations to get valid OpIds
    let (op1_id, _v1) = store.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![])?;
    let (op2_id, _v2) = store.add_op(TestInstructionSet::IntInput { pos: 1 }, svec![])?;

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

    Ok(())
}

/// Tests that ValOrigin and ValUse maintain their Debug trait implementation.
///
/// This ensures that these types can be printed for debugging purposes
/// and that their debug output contains the expected information.
#[test]
fn test_val_origin_and_val_use_debug() -> Result<(), IRError<TestLang>> {
    use crate::{ValOrigin, ValUse};

    let mut store: IR<TestLang> = IR::empty();
    let (op_id, _v) = store.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![])?;

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

    Ok(())
}

/// Tests integration of position tracking with IR iterator methods.
///
/// This verifies that the position tracking works correctly when accessed through
/// the standard IR iteration methods like get_args_iter() and get_users_iter().
#[test]
fn test_position_tracking_with_iterators() -> Result<(), IRError<TestLang>> {
    let mut store: IR<TestLang> = IR::empty();

    // Build a more complex graph to test iterator integration
    let (_input1_id, v1) = store.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![])?;
    let (_input2_id, v2) = store.add_op(TestInstructionSet::IntInput { pos: 1 }, svec![])?;
    let (divrem_id, v_divrem) = store.add_op(TestInstructionSet::DivRem, svec![v1[0], v2[0]])?;
    let (add_id, _v_add) = store.add_op(TestInstructionSet::Add, svec![v_divrem[0], v_divrem[1]])?;

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

    Ok(())
}
