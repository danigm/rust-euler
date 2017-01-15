//! Problem 3
//! Largest prime factor
//!
//! The prime factors of 13195 are 5, 7, 13 and 29.
//! What is the largest prime factor of the number 600851475143 ?

fn is_prime(n: u64) -> bool {
    let mut max: u64 = f64::sqrt(n as f64).trunc() as u64;
    max += 1;
    for i in 2..max {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn factorize(n: u64) -> Vec<u64> {
    let mut factors: Vec<u64> = vec!();
    let mut max: u64 = f64::sqrt(n as f64).trunc() as u64;
    max += 1;
    for i in 2..max {
        if n % i == 0 && is_prime(i) {
            factors.push(i);
        }
    }
    factors
}

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
    let sol = factorize(600851475143).iter()
        .fold(0 as u64, |max, x| if x > &max { *x } else { max });
    sol
}
