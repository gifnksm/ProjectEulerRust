//! [Problem 56](https://projecteuler.net/problem=56) solver.

#![warn(
    bad_style,
    unused,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

use num_bigint::BigUint;
use num_traits::{FromPrimitive, One};

fn compute(a: u32, b: u32) -> u32 {
    num_iter::range(BigUint::one(), FromPrimitive::from_u32(a).unwrap())
        .map(|a| {
            itertools::unfold(One::one(), |n: &mut BigUint| {
                (*n) = &a * (&*n);
                Some(n.to_string())
            })
            .map(|s| s.chars().filter_map(|c| c.to_digit(10)).sum())
            .take(b as usize)
            .max()
            .unwrap()
        })
        .max()
        .unwrap()
}

fn solve() -> String {
    compute(100, 100).to_string()
}

common::problem!("972", solve);
