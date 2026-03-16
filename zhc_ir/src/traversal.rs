//! Operation walker verification utilities.
//!
//! Provides [`OpWalkerVerifier`], an extension trait on operation-ID iterators
//! that checks whether the iteration order respects topological dependency
//! constraints within an [`IR`].

use crate::{Dialect, IR, OpId};

/// An operation walker that can verify topological ordering properties.
pub trait OpWalkerVerifier: Iterator<Item = OpId> {
    /// Checks if the walker respects topological ordering of dependencies.
    ///
    /// Verifies that all operations appear after their dependencies in the
    /// walk sequence. Returns `true` if the walker maintains correct
    /// dependency ordering, `false` otherwise.
    fn is_topo_sorted<D: Dialect>(self, ir: &IR<D>) -> bool;
}

impl<T> OpWalkerVerifier for T
where
    T: Iterator<Item = OpId>,
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
