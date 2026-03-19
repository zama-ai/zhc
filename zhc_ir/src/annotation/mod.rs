//! Annotated intermediate representation providing type-safe access to IR elements with custom
//! annotations.
//!
//! This module extends the base IR with annotation capabilities, allowing arbitrary metadata to be
//! attached to operations and values. The core types `AnnOpRef` and `AnnValRef` wrap their base
//! counterparts while providing access to associated annotations through a unified interface.
//!
//! The `AnnIR` container maintains parallel annotation maps alongside the base IR, ensuring
//! type safety and consistent access patterns. All navigation methods preserve annotation
//! context, returning annotated references that combine structural information with metadata.
//!
//! Key design principles:
//! - Annotations are stored in separate maps to avoid IR structure changes
//! - All public references carry both IR data and annotation context
//! - Deref implementations allow transparent access to underlying IR functionality

mod ir;
mod op_ref;
mod traits;
mod val_origin;
mod val_ref;
mod val_use;
mod view;

pub use ir::*;
pub use op_ref::*;
pub use traits::*;
pub use val_origin::*;
pub use val_ref::*;
pub use val_use::*;

/// Tracks whether an annotation slot has been filled during an analysis pass.
///
/// Used as `Analysing<Ann>` inside op/val maps during annotation traversals:
/// slots start as [`Pending`](Analysing::Pending) and transition to
/// [`Analyzed`](Analysing::Analyzed) once the analysis callback produces a
/// result. After the pass completes, all slots are unwrapped via
/// [`unwrap_analyzed`](Analysing::unwrap_analyzed).
#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Analysing<A> {
    Pending,
    Analyzed(A),
}

impl<A> Analysing<A> {
    /// Extracts the inner value.
    ///
    /// # Panics
    ///
    /// Panics if the slot is still [`Pending`](Analysing::Pending).
    pub fn unwrap_analyzed(self) -> A {
        match self {
            Analysing::Pending => panic!("Tried to unwrap a pending analysis."),
            Analysing::Analyzed(a) => a,
        }
    }
}
