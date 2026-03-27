use zhc_utils::small::SmallVec;

use super::{Depth, Dialect, Signature, State, ValId};

/// Represents an operation within the intermediate representation.
///
/// An operation combines a dialect-specific operation with its type signature,
/// arguments, return values, execution state, and depth information. Operations
/// are the fundamental computational units in the IR graph.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Op<D: Dialect> {
    /// The dialect-specific instruction being performed.
    pub instruction: D::InstructionSet,
    /// Type signature specifying argument and return types.
    pub signature: Signature<D::TypeSystem>,
    /// Input values consumed by this operation.
    pub args: SmallVec<ValId>,
    /// Output values produced by this operation.
    pub returns: SmallVec<ValId>,
    /// Current execution state of the operation.
    pub state: State,
    /// Scheduling depth for operation ordering.
    pub depth: Depth,
    /// Optional comment for debugging purposes.
    pub comment: Option<String>,
}
