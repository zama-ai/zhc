use crate::{Dialect, IR, OpId, OpRef, ValId, val_ref::ValRef};

/// An iterator that yields operation IDs in some order.
///
/// This trait is automatically implemented for any iterator that yields `OpId` values.
/// Operation walkers define the traversal order for visiting operations in an IR.
pub trait OpWalker: Iterator<Item = OpId> {}

impl<T> OpWalker for T where T: Iterator<Item = OpId> {}

/// An iterator that yields operation references in some order.
///
/// This trait is automatically implemented for any iterator that yields `OpRef` values.
/// Operation walks provide access to operation data during IR traversal.
pub trait OpWalk<'a, D: Dialect>: Iterator<Item = OpRef<'a, D>> {}

impl<'a, D: Dialect, T> OpWalk<'a, D> for T where T: Iterator<Item = OpRef<'a, D>> {}

/// An iterator that yields value IDs in some order.
///
/// This trait is automatically implemented for any iterator that yields `ValId` values.
/// Value walkers define the traversal order for visiting values in an IR.
pub trait ValWalker: Iterator<Item = ValId> {}

impl<T> ValWalker for T where T: Iterator<Item = ValId> {}

/// An iterator that yields value references in some order.
///
/// This trait is automatically implemented for any iterator that yields `ValRef` values.
/// Value walks provide access to value data during IR traversal.
pub trait ValWalk<'a, D: Dialect>: Iterator<Item = ValRef<'a, D>> {}

impl<'a, D: Dialect, T> ValWalk<'a, D> for T where T: Iterator<Item = ValRef<'a, D>> {}

/// An operation walker that can verify topological ordering properties.
///
/// This trait extends `OpWalker` with methods to validate that the traversal
/// order respects dependency constraints in the IR graph.
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
