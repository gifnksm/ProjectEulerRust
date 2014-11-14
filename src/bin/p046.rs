#![warn(unused, bad_style,
        unused_qualifications, unused_typecasts, unused_results)]

extern crate common;
extern crate integer;
extern crate prime;

use std::iter;
use common::Solver;
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

fn main() { Solver::new("5777", solve).run(); }