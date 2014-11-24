#![warn(bad_style,
        unused, unused_extern_crates, unused_import_braces,
        unused_qualifications, unused_results, unused_typecasts)]

extern crate common;
extern crate cont_frac;
extern crate num;

use std::iter::AdditiveIterator;
use common::Solver;
use num::BigUint;

fn napier_seq(i: uint) -> uint {
    match i {
        0 => 2,
        i if i % 3 == 2 => 2 * (i + 1) / 3,
        _ => 1
    }
}

fn solve() -> String {
    let len = 100;
    let napier = range(0, len).map(napier_seq);
    let (n, _d) = cont_frac::fold::<BigUint, _>(napier);
    n.to_string()
        .chars()
        .filter_map(|c| c.to_digit(10))
        .sum()
        .to_string()
}

fn main() { Solver::new("272", solve).run(); }

