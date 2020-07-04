//! [Problem 64](https://projecteuler.net/problem=64) solver.

#![warn(
    bad_style,
    unused,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

use num_integer::Integer;

fn solve() -> String {
    (1u32..10001)
        .map(cont_frac::sqrt)
        .map(|(_a0, an)| an.len())
        .filter(|an| an.is_odd())
        .count()
        .to_string()
}

common::problem!("1322", solve);
