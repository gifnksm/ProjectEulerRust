//! [Problem 46](https://projecteuler.net/problem=46) solver.

#![warn(bad_style,
        unused, unused_extern_crates, unused_import_braces,
        unused_qualifications, unused_results, unused_typecasts)]

#[macro_use(problem)] extern crate common;
extern crate integer;
extern crate prime;

use std::iter;
use integer::Integer;
use prime::PrimeSet;

fn is_goldbach(ps: &PrimeSet, n: u64) -> bool {
    for s in range(1, (n / 2).sqrt() + 1) {
        let sq = s * s * 2;
        if sq > n { return false }
        if ps.contains(n - sq) { return true }
    }
    false
}

fn solve() -> String {
    let ps = PrimeSet::new();
    iter::count(3, 2)
        .filter(|&n| !ps.contains(n))
        .skip_while(|&n| is_goldbach(&ps, n))
        .next()
        .unwrap()
        .to_string()
}

problem!("5777", solve);
