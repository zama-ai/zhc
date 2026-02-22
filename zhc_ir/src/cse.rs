//! Common subexpression elimination pass.
//!
//! Uses local value numbering to identify operations that compute identical
//! expressions. When a duplicate is found, all uses of the redundant value are
//! replaced with the original, and dead code elimination removes the now-unused
//! operation. Dialects opt in via [`AllowCse`], which also provides a hook
//! for normalizing commutative argument orderings so that permuted operands
//! are recognized as equivalent.

use zhc_utils::{FastMap, Store, small::SmallVec};

use super::{Dialect, DialectInstructionSet, IR, ValId, ValueNumber, dce::eliminate_dead_code};

/// This trait extends `Dialect` to enable Common Subexpression Elimination (CSE).
///
/// ## Handling Commutative Operations
///
/// This trait enables proper handling of commutative operations. For example, in an integer
/// dialect, `add(%1, %0)` and `add(%0, %1)` represent the same value. By default, argument
/// values are stored in their original order within expressions, causing these equivalent
/// operations to be treated as different values.
///
/// To handle commutativity correctly, argument values should be normalized (e.g., sorted)
/// before being stored in the expression, ensuring the result is independent of input order.
///
/// Different operations may require different normalization strategies depending on their
/// specific commutativity properties.
pub trait AllowCse: Dialect {
    /// Converts an operation and its argument value numbers into expressions for each return value.
    ///
    /// Given an operation and an iterator over argument value numbers (in order), this method
    /// returns an iterator over the expressions associated with each return value (in order).
    ///
    /// ## Customizing Commutativity Behavior
    ///
    /// This method allows customization of CSE behavior for commutative operations. The default
    /// implementation treats arguments in their original order, so commuted variants are not
    /// recognized as equivalent. Override this method to implement custom argument normalization
    /// for commutative operations.
    fn op_to_exprs(
        op: Self::InstructionSet,
        args: impl Iterator<Item = ValueNumber>,
    ) -> impl Iterator<Item = Expr<Self>> {
        let args = args.collect::<SmallVec<_>>();
        (0..op.get_signature().get_returns_arity()).map(move |i| Expr {
            op: op.clone(),
            args: args.clone(),
            ret_pos: i as u8,
        })
    }
}

/// An expression representing a computed value in CSE analysis.
///
/// Captures the operation, its argument value numbers, and which return value
/// position this expression represents. Two expressions are considered equivalent
/// if they have the same operation and argument value numbers.
#[derive(Hash, PartialEq, Eq)]
pub struct Expr<D: Dialect> {
    /// The operation that computes this expression.
    pub op: D::InstructionSet,
    /// The value numbers of the operation's arguments.
    pub args: SmallVec<ValueNumber>,
    /// Which return value position (0-based) this expression represents.
    pub ret_pos: u8,
}

/// A replacement opportunity identified by CSE analysis.
///
/// Indicates that the `old` value can be replaced by the `new` value because
/// they represent equivalent expressions.
pub struct CanReplace {
    /// The value ID that should be replaced.
    old: ValId,
    /// The value ID that should replace the old value.
    new: ValId,
}

/// Analysis result containing common subexpression elimination opportunities.
///
/// This structure holds the results of CSE analysis, identifying which values
/// can be replaced with equivalent previously computed values.
pub struct CommonSubexpressionAnalysis {
    replacements: SmallVec<CanReplace>,
}

impl CommonSubexpressionAnalysis {
    /// Performs common subexpression elimination analysis on the given IR.
    ///
    /// Uses local value numbering to identify expressions that compute the same
    /// values. Operations are processed in topological order to ensure all
    /// dependencies are analyzed before their users.
    pub fn from_ir<D: AllowCse>(ir: &IR<D>) -> Self {
        // We follow the classic Local Value Numbering approach to perform this analysis.
        let mut replacements = SmallVec::new();
        let mut vn_to_valid: Store<ValueNumber, ValId> = Store::empty();
        let mut valid_to_vn: FastMap<ValId, ValueNumber> = FastMap::new();
        let mut expr_to_vn: FastMap<Expr<D>, ValueNumber> = FastMap::new();

        // We iterate following the topological order.
        for op in ir.walk_ops_topological() {
            // We retrieve the vns of the arguments of the operation. It is valid to do so since we
            // are iterating in topological order. For this reasons, the `valid_to_vn` map is
            // already populated for the arguments.
            let arg_vns: SmallVec<_> = op
                .get_args_iter()
                .map(|a| valid_to_vn.get(&a.get_id()).unwrap().to_owned())
                .collect();

            // We compute an expr for each return value of the op and iterate.
            for (expr, val) in
                D::op_to_exprs(op.get_instruction(), arg_vns.into_iter()).zip(op.get_returns_iter())
            {
                // We get the vn for the current val.
                let vn = if expr_to_vn.contains_key(&expr) {
                    // The expression has already been computed. As such a vn is already associated
                    // with it. We retrieve this vn and issue a replacement.
                    let vn = expr_to_vn.get(&expr).unwrap();
                    let vn_valid = vn_to_valid[vn];
                    replacements.push(CanReplace {
                        old: val.get_id(),
                        new: vn_valid,
                    });
                    vn.to_owned()
                } else {
                    // The expression has not yet been computed. We insert the expr in the map.
                    let vn = vn_to_valid.push(val.get_id());
                    expr_to_vn.insert(expr, vn);
                    vn
                };

                // We insert the vn associated to the val. This ensure that at next iteration, the
                // arg_vns can be retrieved.
                valid_to_vn.insert(val.get_id(), vn);
            }
        }

        CommonSubexpressionAnalysis { replacements }
    }

    /// Consumes the analysis and returns an iterator over replacement opportunities.
    pub fn into_iter(self) -> impl Iterator<Item = CanReplace> {
        self.replacements.into_iter()
    }
}

/// Eliminates common subexpressions from the IR.
///
/// Performs CSE analysis to identify redundant computations, replaces uses of
/// redundant values with their equivalent predecessors, and runs dead code
/// elimination to clean up unused operations.
pub fn eliminate_common_subexpressions<D: AllowCse>(ir: &mut IR<D>) {
    let analysis = CommonSubexpressionAnalysis::from_ir(ir);
    for CanReplace { old, new } in analysis.into_iter() {
        ir.replace_val_use(old, new);
    }
    eliminate_dead_code(ir);
}

#[cfg(test)]
mod test {
    use zhc_utils::{assert_display_is, svec};

    use super::ValueNumber;
    use super::*;
    use crate::{testlang::*, *};

    // For the commutative Add test we normalize Add arguments by sorting the value numbers.
    impl AllowCse for TestLang {
        fn op_to_exprs(
            op: Self::InstructionSet,
            args: impl Iterator<Item = ValueNumber>,
        ) -> impl Iterator<Item = Expr<Self>> {
            let args = args.collect::<SmallVec<_>>();
            let args_norm = match op {
                TestInstructionSet::Add => {
                    let mut a = args.clone();
                    a.sort_unstable();
                    a
                }
                _ => args.clone(),
            };
            let arity = op.get_signature().get_returns_arity();
            let exprs = (0..arity)
                .map(move |i| Expr {
                    op: op.clone(),
                    args: args_norm.clone(),
                    ret_pos: i as u8,
                })
                .collect::<Vec<_>>();
            exprs.into_iter()
        }
    }

    #[test]
    fn test_empty_cse() {
        let mut ir = IR::<TestLang>::empty();
        eliminate_common_subexpressions(&mut ir);
        assert_eq!(ir.n_ops(), 0);
        assert_display_is!(ir.format(), r#""#);
    }

    #[test]
    fn test_duplicate_inc() {
        let mut ir = IR::<TestLang>::empty();
        let (_input_op, input_vals) = ir.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![]);
        let (inc1, _inc1_vals) = ir.add_op(TestInstructionSet::Inc, input_vals.clone());
        let (inc2, inc2_vals) = ir.add_op(TestInstructionSet::Inc, input_vals.clone());
        let (_ret, _ret_vals) = ir.add_op(TestInstructionSet::Return, inc2_vals);

        assert_display_is!(
            ir.format(),
            r#"
            %0 : Int = int_input<pos: 0>();
            %1 : Int = inc(%0 : Int);
            %2 : Int = inc(%0 : Int);
            return(%2 : Int);
        "#
        );

        eliminate_common_subexpressions(&mut ir);

        // inc2 should be replaced by inc1 and removed by DCE.
        assert!(ir.has_opid(inc1));
        assert!(!ir.has_opid(inc2));
        assert_eq!(ir.n_ops(), 3);
        assert_display_is!(
            ir.format(),
            r#"
            %0 : Int = int_input<pos: 0>();
            %1 : Int = inc(%0 : Int);
            return(%1 : Int);
        "#
        );
    }

    #[test]
    fn test_commutative_add() {
        let mut ir = IR::<TestLang>::empty();
        let (_in1, v1) = ir.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![]);
        let (_in2, v2) = ir.add_op(TestInstructionSet::IntInput { pos: 1 }, svec![]);
        // Create two adds that differ only by argument order.
        let (add1, _a1_vals) = ir.add_op(TestInstructionSet::Add, svec![v1[0], v2[0]]);
        let (add2, a2_vals) = ir.add_op(TestInstructionSet::Add, svec![v2[0], v1[0]]);
        let (_ret, _ret_vals) = ir.add_op(TestInstructionSet::Return, a2_vals);

        assert_display_is!(
            ir.format(),
            r#"
            %0 : Int = int_input<pos: 0>();
            %1 : Int = int_input<pos: 1>();
            %2 : Int = add(%0 : Int, %1 : Int);
            %3 : Int = add(%1 : Int, %0 : Int);
            return(%3 : Int);
        "#
        );

        eliminate_common_subexpressions(&mut ir);

        // With commutativity-aware normalization add2 should be replaced by add1.
        assert!(ir.has_opid(add1));
        assert!(!ir.has_opid(add2));
        assert_display_is!(
            ir.format(),
            r#"
            %0 : Int = int_input<pos: 0>();
            %1 : Int = int_input<pos: 1>();
            %2 : Int = add(%0 : Int, %1 : Int);
            return(%2 : Int);
        "#
        );
    }

    #[test]
    fn test_multi_return_divrem() {
        // DivRem returns two values. If computed twice with same operands,
        // both return values should be commoned.
        let mut ir = IR::<TestLang>::empty();
        let (_, in1_vals) = ir.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![]);
        let (_, in2_vals) = ir.add_op(TestInstructionSet::IntInput { pos: 1 }, svec![]);
        let (div1_op, div1_vals) =
            ir.add_op(TestInstructionSet::DivRem, svec![in1_vals[0], in2_vals[0]]);
        let (div2_op, div2_vals) =
            ir.add_op(TestInstructionSet::DivRem, svec![in1_vals[0], in2_vals[0]]);
        // Make a consumer that uses both results of div2 so we can ensure replacement happened.
        let (add_op, add_vals) =
            ir.add_op(TestInstructionSet::Add, svec![div2_vals[0], div1_vals[1]]);
        let (_, _) = ir.add_op(TestInstructionSet::Return, add_vals);

        // Pre-check
        assert!(ir.has_opid(div1_op));
        assert!(ir.has_opid(div2_op));

        assert_display_is!(
            ir.format(),
            r#"
            %0 : Int = int_input<pos: 0>();
            %1 : Int = int_input<pos: 1>();
            %2 : Int, %3 : Int = div_rem(%0 : Int, %1 : Int);
            %4 : Int, %5 : Int = div_rem(%0 : Int, %1 : Int);
            %6 : Int = add(%4 : Int, %3 : Int);
            return(%6 : Int);
        "#
        );

        eliminate_common_subexpressions(&mut ir);

        // div2 should be removed
        assert!(ir.has_opid(div1_op));
        assert!(!ir.has_opid(div2_op));

        // The add consumer originally used div2's two results; they should both have been replaced
        // by div1's values. Find the add op's arguments and check they equal div1_vals.
        let args: Vec<_> = ir
            .get_op(add_op)
            .get_args_iter()
            .map(|v| v.get_id())
            .collect();
        assert_eq!(args.len(), 2);
        assert_eq!(args[0], div1_vals[0]);
        assert_eq!(args[1], div1_vals[1]);

        assert_display_is!(
            ir.format(),
            r#"
            %0 : Int = int_input<pos: 0>();
            %1 : Int = int_input<pos: 1>();
            %2 : Int, %3 : Int = div_rem(%0 : Int, %1 : Int);
            %6 : Int = add(%2 : Int, %3 : Int);
            return(%6 : Int);
        "#
        );
    }
}
