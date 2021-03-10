//! Problem 3
//! Largest prime factor
//!
//! The prime factors of 13195 are 5, 7, 13 and 29.
//! What is the largest prime factor of the number 600851475143 ?

use crate::basic_maths;

/// Solves the problem for 600851475143
///
/// Examples
///
/// ```
/// extern crate problems;
///
/// fn main() {
///     assert_eq!(6857, problems::p3::solve());
/// }
/// ```
pub fn solve() -> u64 {
    let sol = basic_maths::prime_factors(600851475143).iter()
        .fold(0 as u64, |max, x| if x > &max { *x } else { max });
    sol
}
