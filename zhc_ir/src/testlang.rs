//! Minimal dialect for exercising `zhc_ir` machinery in tests.
//!
//! Provides two scalar types (`Int`, `Bool`) and a small instruction
//! set covering nullary inputs, unary/binary arithmetic, conditional
//! selection, multi-return, and a terminal sink. Rich enough to build
//! non-trivial dataflow graphs (diamonds, fan-out, multi-return) while
//! remaining trivial to construct by hand.
use crate::{Dialect, DialectInstructionSet, DialectTypeSystem, IR, signature::Signature};
use std::fmt::Display;
use zhc_utils::{assert_display_is, svec};

/// Type system with two scalar types for testing.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TestTypeSystem {
    /// Integer scalar.
    Int,
    /// Boolean scalar.
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

/// Test instruction set.
///
/// Covers the signature shapes needed to exercise IR construction,
/// traversal, and analysis: nullary producers (`IntInput`,
/// `BoolConstant`), binary arithmetic (`Add`), ternary conditional
/// (`IfElse`), multi-return (`DivRem`), unary (`Inc`), and a
/// terminal sink (`Return`).
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum TestInstructionSet {
    /// Produces an `Int` from positional input `pos`. `() → (Int)`
    IntInput { pos: usize },
    /// Produces a `Bool` constant. `() → (Bool)`
    BoolConstant { val: bool },
    /// Integer addition. `(Int, Int) → (Int)`
    Add,
    /// Conditional select: returns the first or third operand
    /// depending on the boolean second operand.
    /// `(Int, Bool, Int) → (Int)`
    IfElse,
    /// Integer division with remainder. `(Int, Int) → (Int, Int)`
    DivRem,
    /// Increment. `(Int) → (Int)`
    Inc,
    /// Terminal sink consuming one integer. `(Int) → ()`
    Return,
}

impl Display for TestInstructionSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TestInstructionSet::IntInput { pos } => write!(f, "int_input<pos: {}>", pos),
            TestInstructionSet::BoolConstant { val } => {
                write!(f, "bool_constant<val: {}>", val)
            }
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

/// Dialect tag binding [`TestTypeSystem`] and [`TestInstructionSet`].
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TestLang;

impl Dialect for TestLang {
    type TypeSystem = TestTypeSystem;
    type InstructionSet = TestInstructionSet;
}

/// Builds a ~50-node IR graph exercising diamonds, fan-out,
/// multi-return, convergence, and independent subgraphs.
pub fn gen_complex_ir() -> IR<TestLang> {
    let mut ir: IR<TestLang> = IR::empty();

    // Create multiple input sources (wide foundation)
    let (_, inp0) = ir.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![]);
    let (_, inp1) = ir.add_op(TestInstructionSet::IntInput { pos: 1 }, svec![]);
    let (_, inp2) = ir.add_op(TestInstructionSet::IntInput { pos: 2 }, svec![]);
    let (_, inp3) = ir.add_op(TestInstructionSet::IntInput { pos: 3 }, svec![]);
    let (_, bool_inp) = ir.add_op(TestInstructionSet::BoolConstant { val: true }, svec![]);

    // First layer - basic operations on inputs
    let (_, add0) = ir.add_op(TestInstructionSet::Add, svec![inp0[0], inp1[0]]);
    let (_, add1) = ir.add_op(TestInstructionSet::Add, svec![inp2[0], inp3[0]]);
    let (_, inc0) = ir.add_op(TestInstructionSet::Inc, svec![inp0[0]]);
    let (_, inc1) = ir.add_op(TestInstructionSet::Inc, svec![inp1[0]]);

    // Create a diamond pattern: add0 -> inc2, inc3 -> add2
    let (_, inc2) = ir.add_op(TestInstructionSet::Inc, svec![add0[0]]);
    let (_, inc3) = ir.add_op(TestInstructionSet::Inc, svec![add0[0]]);
    let (_, add2) = ir.add_op(TestInstructionSet::Add, svec![inc2[0], inc3[0]]);

    // Create a deeper chain from inp2
    let (_, chain0) = ir.add_op(TestInstructionSet::Inc, svec![inp2[0]]);
    let (_, chain1) = ir.add_op(TestInstructionSet::Inc, svec![chain0[0]]);
    let (_, chain2) = ir.add_op(TestInstructionSet::Inc, svec![chain1[0]]);
    let (_, chain3) = ir.add_op(TestInstructionSet::Inc, svec![chain2[0]]);
    let (_, chain4) = ir.add_op(TestInstructionSet::Inc, svec![chain3[0]]);

    // Multi-output operation creating branching
    let (_, divrem0) = ir.add_op(TestInstructionSet::DivRem, svec![add1[0], inc0[0]]);
    let (_, divrem1) = ir.add_op(TestInstructionSet::DivRem, svec![chain4[0], inp3[0]]);

    // Fan-out: use both outputs of divrem operations
    let (_, inc4) = ir.add_op(TestInstructionSet::Inc, svec![divrem0[0]]); // quotient
    let (_, inc5) = ir.add_op(TestInstructionSet::Inc, svec![divrem0[1]]); // remainder
    let (_, inc6) = ir.add_op(TestInstructionSet::Inc, svec![divrem1[0]]); // quotient
    let (_, inc7) = ir.add_op(TestInstructionSet::Inc, svec![divrem1[1]]); // remainder

    // Create convergence points
    let (_, conv0) = ir.add_op(TestInstructionSet::Add, svec![inc4[0], inc5[0]]);
    let (_, conv1) = ir.add_op(TestInstructionSet::Add, svec![inc6[0], inc7[0]]);
    let (_, conv2) = ir.add_op(TestInstructionSet::Add, svec![add2[0], chain2[0]]);

    // IfElse operations using the boolean input
    let (_, ifelse0) = ir.add_op(
        TestInstructionSet::IfElse,
        svec![conv0[0], bool_inp[0], conv1[0]],
    );
    let (_, ifelse1) = ir.add_op(
        TestInstructionSet::IfElse,
        svec![conv2[0], bool_inp[0], inc1[0]],
    );

    // Create more complex interactions
    let (_, add3) = ir.add_op(TestInstructionSet::Add, svec![ifelse0[0], ifelse1[0]]);
    let (_, add4) = ir.add_op(TestInstructionSet::Add, svec![conv0[0], conv1[0]]);
    let (_, add5) = ir.add_op(TestInstructionSet::Add, svec![chain4[0], add2[0]]);

    // Another level of DivRem for more multi-output complexity
    let (_, divrem2) = ir.add_op(TestInstructionSet::DivRem, svec![add3[0], add4[0]]);
    let (_, divrem3) = ir.add_op(TestInstructionSet::DivRem, svec![add5[0], ifelse1[0]]);

    // Final convergence layer
    let (_, final0) = ir.add_op(TestInstructionSet::Add, svec![divrem2[0], divrem3[0]]);
    let (_, final1) = ir.add_op(TestInstructionSet::Add, svec![divrem2[1], divrem3[1]]);
    let (_, final2) = ir.add_op(TestInstructionSet::Add, svec![final0[0], final1[0]]);

    // Independent subgraph that eventually merges
    let (_, indep0) = ir.add_op(TestInstructionSet::Inc, svec![inp0[0]]);
    let (_, indep1) = ir.add_op(TestInstructionSet::Inc, svec![indep0[0]]);
    let (_, indep2) = ir.add_op(TestInstructionSet::Inc, svec![indep1[0]]);

    // Merge independent subgraph with main computation
    let (_, ultimate) = ir.add_op(TestInstructionSet::Add, svec![final2[0], indep2[0]]);

    // Some effect operations
    let (_, _) = ir.add_op(TestInstructionSet::Return, svec![ultimate[0]]);
    let (_, _) = ir.add_op(TestInstructionSet::Return, svec![final0[0]]);
    let (_, _) = ir.add_op(TestInstructionSet::Return, svec![conv2[0]]);

    // Additional independent operations to reach ~50 nodes
    let (_, extra0) = ir.add_op(TestInstructionSet::Inc, svec![inp3[0]]);
    let (_, extra1) = ir.add_op(TestInstructionSet::Inc, svec![extra0[0]]);
    let (_, extra2) = ir.add_op(TestInstructionSet::Add, svec![extra1[0], chain1[0]]);
    let (_, _) = ir.add_op(TestInstructionSet::Return, svec![extra2[0]]);

    // More branching from existing values
    let (_, branch0) = ir.add_op(TestInstructionSet::Inc, svec![add1[0]]);
    let (_, branch1) = ir.add_op(TestInstructionSet::Inc, svec![branch0[0]]);
    let (_, branch2) = ir.add_op(TestInstructionSet::Add, svec![branch1[0], inc7[0]]);
    let (_, _) = ir.add_op(TestInstructionSet::Return, svec![branch2[0]]);

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

    ir
}
