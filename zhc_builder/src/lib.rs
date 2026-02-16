//! Provides high-level builders for constructing homomorphic encryption circuits.
//!
//! This crate offers the `Builder` type for creating fully homomorphic encryption
//! operations over encrypted integers, along with specialized integer comparison
//! operations in the `iops` module.

pub mod builder;
pub mod iops;
