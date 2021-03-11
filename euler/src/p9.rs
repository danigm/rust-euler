//! Problem 9
//!
//! A Pythagorean triplet is a set of three natural numbers, a < b < c, for which,
//! a**2 + b**2 = c**2
//!
//! For example, 3**2 + 4**2 = 9 + 16 = 25 = 5**2.
//!
//! There exists exactly one Pythagorean triplet for which a + b + c = 1000.  Find the product abc.

/// Solves the problem
///
/// Examples
///
/// ```
/// extern crate problems;
///
/// fn main() {
///     assert_eq!(31875000, problems::p9::solve());
/// }
/// ```
pub fn solve() -> u32 {
    let mut a: u32 = 1;
    let mut b: u32 = 2;
    let mut c: u32 = 997;

    for i in 1..333 {
        for j in (i+1)..499 {
            for k in ((j+1)..(1000 - i - j + 1)).rev() {
                if i + j + k != 1000 {
                    break;
                }

                if pytagorean_triplet(i, j, k) {
                    a = i;
                    b = j;
                    c = k;
                    break;
                }
            }
        }
    }

    a * b * c
}

fn pytagorean_triplet(a: u32, b: u32, c: u32) -> bool {
    a < b && b < c && a.pow(2) + b.pow(2) == c.pow(2)
}
