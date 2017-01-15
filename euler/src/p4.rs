//! Problem 4
//!  Largest palindrome product
//!
//! A palindromic number reads the same both ways. The largest palindrome made
//! from the product of two 2-digit numbers is 9009 = 91 × 99.
//!
//! Find the largest palindrome made from the product of two 3-digit numbers.

/// Returns true if `n` is a palindromic number
///
/// Examples
///
/// ```
/// assert_eq!(true, problems::p4::is_palindromic(9009));
/// assert_eq!(false, problems::p4::is_palindromic(9019));
/// ```
pub fn is_palindromic(n: i32) -> bool {
    let s1 = format!("{}", n);
    let mut s2 = s1.clone();
    unsafe {
        let vec = s2.as_mut_vec();
        vec.reverse();
    }

    s1 == s2
}

/// Solves the problem
///
/// Examples
///
/// ```
/// extern crate problems;
///
/// fn main() {
///     assert_eq!(906609, problems::p4::solve());
/// }
/// ```
pub fn solve() -> i32 {
    let mut max = 0;
    for i in 100..1000 {
        for j in 100..1000 {
            let p = i * j;
            if is_palindromic(p) && p > max {
                max = p;
            }
        }
    }
    max
}
