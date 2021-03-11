//! Problem 7
//! 10001st prime
//!
//! By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see
//! that the 6th prime is 13.
//!
//! What is the 10 001st prime number?

/// Solves the problem
///
/// Examples
///
/// ```
/// extern crate problems;
///
/// fn main() {
///     assert_eq!(104743, problems::p7::solve());
/// }
/// ```
pub fn solve() -> i32 {
    let mut n = 3;
    let mut primes = vec!(2);
    let mut isprime = true;
    let x = 10001;
    let mut nprimes = 1;
    let mut sqrt;
    while nprimes < x {
        sqrt = (n as f32).sqrt() as i32;
        for p in &primes {
            if p > &sqrt {
                isprime = true;
                break;
            }
            if n % p == 0 {
                isprime = false;
                break;
            }
        }
        if isprime {
            nprimes += 1;
            primes.push(n);
        }
        n += 2;
        isprime = true;
    }
    primes[x - 1]
}

pub fn solve1() -> i32 {
    let mut n = 3;
    let mut primes = vec!(2);
    let mut isprime = true;
    let x = 10001;
    while primes.len() < x {
        for p in &primes {
            if n % p == 0 {
                isprime = false;
                break;
            }
        }
        if isprime {
            primes.push(n);
        }
        n += 2;
        isprime = true;
    }
    primes[x - 1]
}

