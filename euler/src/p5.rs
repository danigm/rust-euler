//! Problem 5
//! Smallest multiple
//!
//! 2520 is the smallest number that can be divided by each of the numbers from
//! 1 to 10 without any remainder.
//!
//! What is the smallest positive number that is evenly divisible by all of the
//! numbers from 1 to 20?

use basic_maths;

/// Solves the problem
///
/// Examples
///
/// ```
/// extern crate problems;
///
/// fn main() {
///     assert_eq!(232792560, problems::p5::solve());
/// }
/// ```
pub fn solve() -> i32 {
    let v: Vec<i32> = (1..21).collect();
    basic_maths::lcmv(&v)
}
