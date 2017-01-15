//! Problem 2
//! Even Fibonacci numbers
//!
//! Each new term in the Fibonacci sequence is generated by adding the previous
//! two terms. By starting with 1 and 2, the first 10 terms will be:
//!
//! 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, ...
//!
//! By considering the terms in the Fibonacci sequence whose values do not
//! exceed four million, find the sum of the even-valued terms.


struct Fib {
    n1: u32,
    n2: u32,
    max: u32,
}

impl Fib {
    fn new(max: u32) -> Fib {
        Fib { n1: 0, n2: 1, max: max }
    }
}

impl Iterator for Fib {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        let n1 = self.n1;
        let n2 = self.n2;

        self.n1 = n2;
        self.n2 = n1 + n2;

        if self.n2 <= self.max {
            Some(self.n2)
        } else {
            None
        }
    }
}

/// Solves the problem for fib numbers minor than 4M
pub fn solve() -> u32 {
    let sol = Fib::new(4*1000*1000)
        .filter(|&x| x % 2 == 0)
        .fold(0, |sum, x| sum + x);
    sol
}