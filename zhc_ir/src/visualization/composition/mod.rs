//! Layout engine for diagram elements with CSS-like styling and constraint solving.
//!
//! This module provides a complete layout system for rendering hierarchical diagrams.
//! The core architecture revolves around the `Element` trait, which defines a two-phase
//! layout process: size calculation followed by frame positioning.
//!
//! ## Layout Process
//!
//! Elements progress through three states tracked by `Solution`:
//! - Fresh: newly created, no layout computed
//! - Sized: intrinsic size calculated based on content and styling
//! - Framed: positioned within available space with final bounds
//!
//! The layout containers (`VStack`, `HStack`, `V3`) implement automatic spacing
//! and positioning of child elements. Each element can be styled through the
//! `StyleSheet` system using CSS-like classes for typography, spacing, and alignment.
//!
//! ## Element Hierarchy
//!
//! Primitive elements (`TextBox`, `Empty`) compute their own intrinsic sizes,
//! while container elements (`VStack`, `HStack`, `V3`) aggregate child layouts.
//! The `D2` enum allows runtime polymorphism between different element types.
//!
//! Type aliases provide semantic naming for diagram components: `Operation` combines
//! inputs, body, and outputs in a vertical layout, while `Layer` and `Diagram`
//! create the overall hierarchical structure.

mod annotation;
mod element;
mod model;
mod primitives;
mod solution;
mod solver;
mod stylesheet;

pub use annotation::*;
pub use element::*;
pub use model::*;
pub use primitives::*;
pub use solution::*;
pub use solver::*;
pub use stylesheet::*;
