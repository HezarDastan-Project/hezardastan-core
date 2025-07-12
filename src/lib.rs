// src/lib.rs
// This file will eventually contain common utilities and core logic
// that can be tested independently and used by other modules (like main.rs).

/// Adds two integers together.
/// # Examples
/// ```
/// assert_eq!(hezardastan_core::add(2, 3), 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)] // This attribute tells Rust to compile this module only when running tests.
mod tests {
    use super::*; // Import everything from the parent module (our `add` function).

    #[test] // This attribute marks a function as a test function.
    fn test_add_two_numbers() {
        // Test case 1: Positive numbers
        assert_eq!(add(2, 3), 5); // Checks if add(2, 3) equals 5

        // Test case 2: Zero
        assert_eq!(add(0, 7), 7);

        // Test case 3: Negative numbers
        assert_eq!(add(-1, -4), -5);

        // Test case 4: Positive and negative
        assert_eq!(add(10, -3), 7);
    }

    #[test]
    fn test_add_large_numbers() {
        // Test case for larger numbers
        assert_eq!(add(1000, 2000), 3000);
    }
}
