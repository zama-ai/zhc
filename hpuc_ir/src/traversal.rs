use crate::{Dialect, IR, OpId, OpRef, ValId, val_ref::ValRef};

pub trait OpWalker: Iterator<Item = OpId> {}

impl<T> OpWalker for T where T: Iterator<Item = OpId> {}

pub trait OpWalk<'a, D: Dialect>: Iterator<Item = OpRef<'a, D>> {}

impl<'a, D: Dialect, T> OpWalk<'a, D> for T where T: Iterator<Item = OpRef<'a, D>> {}

pub trait ValWalker: Iterator<Item = ValId> {}

impl<T> ValWalker for T where T: Iterator<Item = ValId> {}

pub trait ValWalk<'a, D: Dialect>: Iterator<Item = ValRef<'a, D>> {}

impl<'a, D: Dialect, T> ValWalk<'a, D> for T where T: Iterator<Item = ValRef<'a, D>> {}

pub trait OpWalkerVerifier: OpWalker {
    /// Checks if the walker respects topological ordering of dependencies.
    ///
    /// Verifies that all operations appear after their dependencies in the
    /// walk sequence. Returns `true` if the walker maintains correct
    /// dependency ordering, `false` otherwise.
    fn is_topo_sorted<D: Dialect>(self, ir: &IR<D>) -> bool;
}

impl<T> OpWalkerVerifier for T
where
    T: OpWalker,
{
    fn is_topo_sorted<D: Dialect>(self, ir: &IR<D>) -> bool {
        let mut setmap = ir.empty_opmap();
        for opid in self {
            setmap.insert(opid, ());
            let op = ir.get_op(opid);
            for pred in op.get_predecessors_iter() {
                if setmap.get(&pred.id).is_none() {
                    return false;
                }
            }
        }
        true
    }
}
