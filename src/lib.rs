//! A module documentation test
//!
/*! This is a really cool module
    that does nothing, but maybe
    nothing is better than an error.
*/
//!
//! # Examples of how my module works
//!

//! ## A simple sum
//! ```
//! use doc_tests::sum;
//! let a = 5;
//! let b = 6;
//! assert_eq!(11, sum(a, b));
//! ```

//! ## A simple multiplication
//! ```
//! use doc_tests::math::multiply;
//! let a = 5;
//! let b = 6;
//! assert_eq!(30, multiply(a, b));
//! ```

//! ## Subtract!
//! ```
//! use doc_tests::math::*;
//! let a = 15;
//! let b = 6;
//! assert_eq!(9, sub(a, b));
//! ```

pub mod math;

pub fn sum(a: i32, b: i32) -> i32 {
    a + b
}
