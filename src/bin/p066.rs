//! [Problem 66](https://projecteuler.net/problem=66) solver.

#![warn(bad_style,
        unused, unused_extern_crates, unused_import_braces,
        unused_qualifications, unused_results)]

#![feature(iter_cmp)]

#[macro_use(problem)] extern crate common;
extern crate cont_frac;
extern crate iter;
extern crate num;

use iter::Difference;
use num::BigUint;

fn solve() -> String {
    let ns = 1..;
    let sq = (1..).map(|x| x * x);

    Difference::new(ns, sq)
        .take_while(|&d| d <= 1000)
        .max_by(|&d| cont_frac::solve_pel::<BigUint>(d).0)
        .unwrap()
        .to_string()
}

problem!("661", solve);

