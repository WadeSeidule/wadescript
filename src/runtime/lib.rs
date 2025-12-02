//! WadeScript Runtime Library
//!
//! Provides C-compatible functions for list and dictionary operations.
//! This is compiled as a static library and linked with generated WadeScript programs.

#![allow(dead_code)]

pub mod list;
pub mod dict;

// Re-export the functions to ensure they're available for linking
pub use list::*;
pub use dict::*;
