//! [Problem 27](https://projecteuler.net/problem=27) solver.

#![warn(
    bad_style,
    unused,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

use prime::PrimeSet;

// p(n) = n^2 + an + b is prime for n = 0 .. N
// p(0) = b         => b must be prime
// p(1) = 1 + a + b => a > -(1+b)
// p(2) = 4 + 2a + b
fn get_limit_n(ps: &PrimeSet, a: i32, b: i32) -> u32 {
    (0..)
        .take_while(|&n| {
            let val = n * n + a * n + b;
            val >= 0 && ps.contains(val as u64)
        })
        .last()
        .unwrap() as u32
}

fn compute(limit: u64) -> i32 {
    let ps = PrimeSet::new();
    let (a, b, _len) = ps
        .iter()
        .take_while(|&p| p < limit)
        .filter_map(|p| {
            let b = p as i32;
            (-b..1000)
                .map(|a| (a, b, get_limit_n(&ps, a, b)))
                .max_by_key(|&(_a, _b, len)| len)
        })
        .max_by_key(|&(_a, _b, len)| len)
        .unwrap();
    a * b
}

fn solve() -> String {
    compute(1000).to_string()
}

common::problem!("-59231", solve);

#[cfg(test)]
mod tests {
    use prime::PrimeSet;

    #[test]
    fn primes() {
        let ps = PrimeSet::new();
        assert_eq!(39, super::get_limit_n(&ps, 1, 41));
        assert_eq!(79, super::get_limit_n(&ps, -79, 1601))
    }
}
