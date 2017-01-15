//! Problem 6
//! Sum square difference
//!
//! The sum of the squares of the first ten natural numbers is,
//! 1^2 + 2^2 + ... + 10^2 = 385
//!
//! The square of the sum of the first ten natural numbers is,
//! (1 + 2 + ... + 10)^2 = 55^2 = 3025
//!
//! Hence the difference between the sum of the squares of the first ten
//! natural numbers and the square of the sum is 3025 − 385 = 2640.
//!
//! Find the difference between the sum of the squares of the first one hundred
//! natural numbers and the square of the sum.

fn sum_squares(v: &Vec<i32>) -> i32 {
    v.iter().fold(0, |sum, x| sum + x*x)
}

fn square_sum(v: &Vec<i32>) -> i32 {
    let n = v.iter().fold(0, |sum, x| sum + x);
    n * n
}

/// Solves the problem
///
/// Examples
///
/// ```
/// extern crate problems;
///
/// fn main() {
///     assert_eq!(25164150, problems::p6::solve());
/// }
/// ```
pub fn solve() -> i32 {
    let v: Vec<i32> = (1..101).collect();
    square_sum(&v) - sum_squares(&v)
}
