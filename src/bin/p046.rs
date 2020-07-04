//! [Problem 46](https://projecteuler.net/problem=46) solver.

#![warn(
    bad_style,
    unused,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

use integer::Integer;
use prime::PrimeSet;

fn is_goldbach(ps: &PrimeSet, n: u64) -> bool {
    for s in 1..((n / 2).sqrt() + 1) {
        let sq = s * s * 2;
        if sq > n {
            return false;
        }
        if ps.contains(n - sq) {
            return true;
        }
    }
    false
}

fn solve() -> String {
    let ps = PrimeSet::new();
    (3..)
        .step_by(2)
        .filter(|&n| !ps.contains(n))
        .find(|&n| !is_goldbach(&ps, n))
        .unwrap()
        .to_string()
}

common::problem!("5777", solve);
