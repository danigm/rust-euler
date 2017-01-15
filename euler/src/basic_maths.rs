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
pub fn gcd(n1: i32, n2: i32) -> i32 {
    let mut ds = divisors(n1);
    let ds2 = ds.clone();
    let d = divisors(n2);

    for j in ds2.iter() {
        if !d.contains(j) {
            let p = ds.iter().position(|x| x == j);
            if p.is_some() {
                ds.remove(p.unwrap());
            }
        }
    }

    match ds.iter().max() {
        Some(x) => return *x,
        None => return 1,
    }
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
