use hc_ir::{
    Dialect, DialectInstructionSet,
    cse::{AllowCse, Expr},
};
use hc_utils::iter::CollectInSmallVec;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IopLang;

impl Dialect for IopLang {
    type TypeSystem = super::IopTypeSystem;
    type InstructionSet = super::IopInstructionSet;
}

impl AllowCse for IopLang {
    fn op_to_exprs(
        op: Self::InstructionSet,
        args: impl Iterator<Item = hc_ir::ValueNumber>,
    ) -> impl Iterator<Item = Expr<Self>> {
        use super::IopInstructionSet::*;
        let mut args = args.cosvec();
        let arity = op.get_signature().get_returns_arity();
        let (args, op) = match op {
            AddCt => {
                // In the case of the add ct op, both operands can be commutated.
                args.sort_unstable();
                (args, op)
            }
            _ => (args, op),
        };
        (0..arity).map(move |i| Expr {
            op: op.clone(),
            args: args.clone(),
            ret_pos: i as u8,
        })
    }
}
