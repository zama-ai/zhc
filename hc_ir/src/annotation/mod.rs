//! Annotated intermediate representation providing type-safe access to IR elements with custom annotations.
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

mod traits;
mod ir;
mod op_ref;
mod val_ref;
mod val_origin;
mod val_use;

pub use traits::*;
pub use ir::*;
pub use op_ref::*;
pub use val_ref::*;
pub use val_origin::*;
pub use val_use::*;
