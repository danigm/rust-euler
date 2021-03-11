pub fn is_prime(n: u64) -> bool {
    let mut max: u64 = f64::sqrt(n as f64).trunc() as u64;
    max += 1;
    for i in 2..max {
        if n % i == 0 {
            return false;
        }
    }
    true
}

pub fn prime_factors(n: u64) -> Vec<u64> {
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

/// list of number divisors
pub fn divisors(n: i32) -> Vec<i32> {
    let mut v = vec!();
    let nn = (n / 2) + 1;
    for i in 1..nn {
        if n % i == 0 {
            v.push(i);
            v.push(n / i);
        }
    }
    v
}

/// greatest common denominator
///
/// Examples
///
/// ```
/// assert_eq!(6, problems::basic_maths::gcd(54, 24));
/// assert_eq!(6, problems::basic_maths::gcd(18, 84));
/// assert_eq!(2, problems::basic_maths::gcd(4, 6));
/// ```
fn gcd(n1: i32, n2: i32) -> i32 {
    // algorithm getted from the book "Why Rust?", by Jim Blandy
    let mut n = n1;
    let mut m = n2;
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m; m = n; n = t;
        }
        m = m % n;
    }
    n
}

/// least common multiple
///
/// Examples
///
/// ```
/// assert_eq!(12, problems::basic_maths::lcm(4, 6));
/// ```
pub fn lcm(n1: i32, n2: i32) -> i32 {
    if n2 > n1 {
        return n2 / gcd(n1, n2) * n1;
    }
    n1 / gcd(n1, n2) * n2
}

pub fn gcdv(v: &Vec<i32>) -> i32 {
    v.iter().fold(1, |g, x| gcd(g, *x))
}

pub fn lcmv(v: &Vec<i32>) -> i32 {
    v.iter().fold(1, |g, x| lcm(g, *x))
}

pub fn eratostenes(n: u64) -> bool {
    match n {
        1 => false,
        i if i < 4 => true, // 2, 3 son primos
        i if i % 2 == 0 => false, // los pares no son primos
        i if i < 9 => true, // 4, 6, 8 ya están excluidos
        i if i % 3 == 0 => false, //multiplos de 3
        _ => {
            let r = f64::sqrt(n as f64) as u64;
            let mut f = 5;
            while f <= r {
                if n % f == 0 {
                    return false;
                }
                if n % (f+2) == 0 {
                    return false;
                }
                f += 6;
            }
            true
        },
    }
}
