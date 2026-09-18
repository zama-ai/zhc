//! Unit tests of the [`IR`](crate::IR) container, grouped by topic.

/// The small dialect used by the tests of this crate.
#[allow(unused)]
pub(crate) mod testlang;

mod concat;
mod construction;
mod deletion;
mod positions;
mod reachability;
mod replacement;
