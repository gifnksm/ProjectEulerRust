//! [Problem 21](https://projecteuler.net/problem=21) solver.

#![warn(bad_style,
        unused, unused_extern_crates, unused_import_braces,
        unused_qualifications, unused_results, unused_typecasts)]

#![feature(phase)]

#[phase(plugin, link)] extern crate common;
extern crate prime;

use std::iter::AdditiveIterator;
use prime::{PrimeSet, Factorize};

fn compute(limit: uint) -> uint {
    let ps = PrimeSet::new();

    let sum_of_div = Vec::from_fn(limit, |n| n.sum_of_proper_divisor(&ps));

    sum_of_div
        .iter()
        .map(|&n| n)
        .enumerate()
        .filter(|&(n, div)| div < n)
        .filter(|&(n, div)| sum_of_div[div] == n)
        .map(|(a, b)| a + b)
        .sum()
}

fn solve() -> String { compute(10000).to_string() }

problem!("31626", solve);
