// 17_34_test_coverage_gaps.rs

// The function we are testing
pub fn divide(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        None
    } else {
        Some(a / b)
    }
}

fn main() {
    println!("To run these tests, execute: cargo test");
}

// --- The Pitfall: Insufficient Tests ---
// This test suite only checks the "happy path".
// If logic regarding negatives or zeros changes, this suite might not catch it.
#[cfg(test)]
mod tests_insufficient {
    use super::*;

    #[test]
    fn test_basic_division() {
        assert_eq!(divide(10, 2), Some(5));
    }
    // Missing: Division by zero, negatives, truncation...
}

// --- The Fix: Comprehensive Tests ---
// We explicitly test edge cases, error conditions, and boundary values.
#[cfg(test)]
mod tests_comprehensive {
    use super::*;

    #[test]
    fn test_basic_division() {
        assert_eq!(divide(10, 2), Some(5));
    }

    #[test]
    fn test_division_by_zero() {
        // Critical edge case: preventing panic
        assert_eq!(divide(10, 0), None);
    }

    #[test]
    fn test_division_with_negative_numbers() {
        // Checking sign logic
        assert_eq!(divide(-10, 2), Some(-5));
        assert_eq!(divide(10, -2), Some(-5));
        assert_eq!(divide(-10, -2), Some(5));
    }

    #[test]
    fn test_division_resulting_in_zero() {
        // Checking integer truncation behavior
        assert_eq!(divide(1, 2), Some(0)); 
    }

    #[test]
    fn test_division_of_zero() {
        // Zero numerator logic
        assert_eq!(divide(0, 5), Some(0));
    }
}