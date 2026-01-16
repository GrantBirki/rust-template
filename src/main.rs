#![forbid(unsafe_code)]

use rust_template::{add, subtract};

// This function is excluded from code coverage
#[cfg(not(tarpaulin_include))]
fn main() {
    let a = 5;
    let b = 3;

    println!("Adding {} and {} gives: {}", a, b, add(a, b));
    println!("Subtracting {} from {} gives: {}", b, a, subtract(a, b));
}

// Add test module to improve coverage reporting
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main_imports() {
        // Just verify that imports work properly
        assert_eq!(add(2, 3), 5);
        assert_eq!(subtract(5, 3), 2);
    }
}
