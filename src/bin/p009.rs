#![warn(bad_style,
        unused, unused_extern_crates, unused_import_braces,
        unused_qualifications, unused_results, unused_typecasts)]

extern crate common;
extern crate integer;

use std::cmp;
use std::iter::Repeat;
use common::Solver;
use integer::Integer;

fn compute(sum: uint) -> uint {
    range(2, sum - 1)
        .flat_map(|c| {
            let a_max = cmp::min((sum - c) / 2, (c * c / 2).sqrt());
            range(1, a_max).zip(Repeat::new(c))
        }).map(|(a, c)| (a, sum - c - a, c))
        .find(|&(a, b, c)| a * a + b * b == c * c)
        .map(|(a, b, c)| a * b * c)
        .unwrap()
}

fn solve() -> String { compute(1000).to_string() }

fn main() { Solver::new("31875000", solve).run(); }
