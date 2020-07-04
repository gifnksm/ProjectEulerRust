//! [Problem 123](https://projecteuler.net/problem=123) solver.

#![warn(
    bad_style,
    unused,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

use num_integer::Integer;
use prime::PrimeSet;

// from problem 120
// f(n) := (p[n]-1)^n + (p[n]+1)^n
//
// if n is even:
//   f(n) ≡ 1  f   mod p[n]^2
// if n is odd:
//   f(n) ≡ 2np[n] mod p[n]^2

fn get_mod(n: u64, pn: u64) -> u64 {
    if n.is_even() {
        1
    } else {
        (2 * n * pn) % (pn * pn)
    }
}

fn compute(limit: u64) -> u64 {
    let ps = PrimeSet::new();
    (1..)
        .zip(ps.iter())
        .find(|&(n, pn)| get_mod(n, pn) > limit)
        .unwrap()
        .0
}

fn solve() -> String {
    compute(10u64.pow(10)).to_string()
}

common::problem!("21035", solve);

#[cfg(test)]
mod tests {
    #[test]
    fn pn() {
        assert_eq!(5, super::get_mod(3, 5));
    }

    #[test]
    fn exceeds_10_9() {
        assert_eq!(7037, super::compute(10u64.pow(9)));
    }
}
