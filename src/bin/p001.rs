#![warn(unused, bad_style,
        unnecessary_qualification, unnecessary_typecast, unused_result)]

extern crate common;

use std::iter::AdditiveIterator;
use common::Solver;

fn solve() -> String {
    range(0u, 1000)
        .filter(|&n| n % 3 == 0 || n % 5 == 0)
        .sum()
        .to_string()
}

fn main() {
    Solver::new("233168", solve).run();
}
