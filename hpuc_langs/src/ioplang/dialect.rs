use hpuc_ir::{
    Dialect, DialectOperations,
    cse::{AllowCse, Expr},
};
use hpuc_utils::CollectInSmallVec;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Ioplang;

impl Dialect for Ioplang {
    type Types = super::types::Types;
    type Operations = super::operations::Operations;
}

impl AllowCse for Ioplang {
    fn op_to_exprs(
        op: Self::Operations,
        args: impl Iterator<Item = hpuc_ir::ValueNumber>,
    ) -> impl Iterator<Item = Expr<Self>> {
        use super::Operations::*;
        let mut args = args.cosvec();
        let arity = op.get_signature().get_returns_arity();
        let (args, op) = match op {
            AddCt => {
                // In the case of the add ct op, both operands can be commutated.
                args.sort_unstable();
                (args, op)
            }
            _ => {
                (args, op)
            }
        };
        (0..arity).map(move |i| Expr {
            op: op.clone(),
            args: args.clone(),
            ret_pos: i as u8,
        })
    }
}
