//! Problem 1
//! Multiples of 3 and 5
//!
//! If we list all the natural numbers below 10 that are multiples of 3 or
//! 5, we get 3, 5, 6 and 9. The sum of these multiples is 23. Find the
//! sum of all the multiples of 3 or 5 below 1000.


/// Returns the sum of 3 and 5 multiples below `n`
///
/// # Examples
///
/// ```
/// extern crate problems;
///
/// fn main() {
///     assert_eq!(233168, problems::p1::multiples(1000));
/// }
/// ```
pub fn multiples(n: i32) -> i32 {
    let sol = (1..n)
        .filter(|&x| x % 3 == 0 || x % 5 == 0)
        .fold(0, |sum, x| sum + x);
    sol
}

/// Solves the problem for 1000
pub fn solve() -> i32 {
    multiples(1000)
}
