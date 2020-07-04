//! [Problem 65](https://projecteuler.net/problem=65) solver.

#![warn(
    bad_style,
    unused,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

use num_bigint::BigUint;

fn napier_seq(i: u32) -> u32 {
    match i {
        0 => 2,
        i if i % 3 == 2 => 2 * (i + 1) / 3,
        _ => 1,
    }
}

fn solve() -> String {
    let len = 100;
    let napier = (0u32..len).map(napier_seq);
    let (n, _d) = cont_frac::fold::<BigUint, _>(napier);
    n.to_string()
        .chars()
        .filter_map(|c| c.to_digit(10))
        .sum::<u32>()
        .to_string()
}

common::problem!("272", solve);
