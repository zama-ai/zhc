use zhc_utils::iter::CollectInVec;

use crate::{Dialect, IR, OpId, OpIdRaw, OpMap, ValId, ValMap, traversal::OpWalkerVerifier};

pub fn reschedule<'a, D: Dialect>(
    ir: &'a IR<D>,
    walker: impl Iterator<Item = OpId>,
) -> (IR<D>, OpMap<OpId>, ValMap<ValId>) {
    let mut output = IR::empty();
    let mut valmap = ir.empty_valmap();
    let walker = walker.covec();
    assert_eq!(
        walker.len() as OpIdRaw,
        ir.n_ops(),
        "Tried to schedule with a walker that does not visit all operations."
    );
    assert!(
        walker.iter().copied().is_topo_sorted(ir),
        "Tried to schedule with a walker that is not topo sorted."
    );
    let mut opmap = ir.empty_opmap();
    for op in ir.walk_ops_with(walker.into_iter()) {
        let new_args = op
            .get_arg_valids()
            .iter()
            .map(|a| *valmap.get(a).unwrap())
            .collect();
        let (new_opid, new_rets) = output.add_op(op.get_instruction().clone(), new_args);
        assert!(opmap.insert(op.get_id(), new_opid).is_none());
        op.get_return_valids()
            .iter()
            .copied()
            .zip(new_rets.into_iter())
            .for_each(|(old, new)| assert!(valmap.insert(old, new).is_none()));
    }
    (output, opmap, valmap)
}
