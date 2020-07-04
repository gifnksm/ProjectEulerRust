//! [Problem 9](https://projecteuler.net/problem=9) solver.

#![warn(
    bad_style,
    unused,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

use integer::Integer;
use std::{cmp, iter};

fn compute(sum: u32) -> u32 {
    (2..sum - 1)
        .flat_map(|c| {
            let a_max = cmp::min((sum - c) / 2, (c * c / 2).sqrt());
            (1..a_max).zip(iter::repeat(c))
        })
        .map(|(a, c)| (a, sum - c - a, c))
        .find(|&(a, b, c)| a * a + b * b == c * c)
        .map(|(a, b, c)| a * b * c)
        .unwrap()
}

fn solve() -> String {
    compute(1000).to_string()
}

common::problem!("31875000", solve);
