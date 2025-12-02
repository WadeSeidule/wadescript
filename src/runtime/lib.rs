//! WadeScript Runtime Library
//!
//! Provides C-compatible functions for list, dictionary, and string operations.
//! This is compiled as a static library and linked with generated WadeScript programs.

#![allow(dead_code)]

pub mod list;
pub mod dict;
pub mod string;

// Re-export the functions to ensure they're available for linking
pub use list::*;
pub use dict::*;
pub use string::*;
