//! [Problem 41](https://projecteuler.net/problem=41) solver.

#![warn(
    bad_style,
    unused,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

use integer::Integer;
use iter::Permutations;
use prime::PrimeSet;

// 1 + 2 + ... + 9 = 45 (dividable by 9 => 9-pandigimal number is dividable by 9)
// 1 + 2 + ... + 8 = 36 (dividable by 9 => 9-pandigimal number is dividable by 9)
// 7-pandigimal may be the largest pandigimal prime.

fn compute() -> u64 {
    let radix = 10;
    let ps = PrimeSet::new();
    for (perm, _) in Permutations::new(&[7, 6, 5, 4, 3, 2, 1], 7) {
        let n = Integer::from_digits(perm.iter().rev().copied(), radix);
        if ps.contains(n) {
            return n;
        }
    }
    unreachable!()
}

fn solve() -> String {
    compute().to_string()
}

common::problem!("7652413", solve);
