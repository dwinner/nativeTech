//! # rustdoc-example
//!
//! A simple project demonstrating the use of rustdoc with the function
//! [`mult`].
//!
//! # Example
//!
//! ```
//! use rustdoc_example::mult;
//! assert_eq!(mult(10, 10), 100);
//! ```

#![warn(missing_docs)]

/// Multiply
///
/// # Arguments
///
/// * `left`: left argument
/// * `right`: right argument
///
/// returns: u64 result
///
/// # Examples
///
/// ```
/// use rustdoc_example::mult;
/// let result = mult(10, 11);
/// ```
pub fn mult(left: u64, right: u64) -> u64
{
   left * right
}

#[cfg(test)]
mod tests
{
   use super::*;

   #[test]
   fn it_works()
   {
      let result = mult(2, 2);
      assert_eq!(result, 4);
   }
}
