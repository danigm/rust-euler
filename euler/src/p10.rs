//! Problem 10
//!
//! The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.
//! Find the sum of all the primes below two million.
//!

/// Solves the problem
///
/// Examples
///
/// ```
/// extern crate problems;
///
/// fn main() {
///     assert_eq!(142913828922, problems::p10::solve());
/// }
/// ```

use crate::basic_maths;

pub fn solve() -> u64 {
    let limit = 2_000_000;
    let mut sum = 2;
    let mut i = 3;

    while i < limit {
        if basic_maths::eratostenes(i) {
            sum += i;
        }
        i += 2;
    }

    sum
}
