#![forbid(unsafe_code)]

pub mod math;

// Re-export functions from the math module for easier use.
pub use math::{add, subtract};
