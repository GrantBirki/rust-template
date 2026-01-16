#![forbid(unsafe_code)]

pub mod lib {
    pub mod math;
}

// Re-export functions from the math module for easier use
pub use lib::math::add;
pub use lib::math::subtract;
