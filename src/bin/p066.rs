#![warn(bad_style,
        unused, unused_extern_crates, unused_import_braces,
        unused_qualifications, unused_results, unused_typecasts)]

extern crate common;
extern crate cont_frac;
extern crate "iter" as iter_crate;
extern crate num;

use std::iter;
use common::Solver;
use iter_crate::Difference;
use num::BigUint;

fn solve() -> String {
    let ns = iter::count(1u, 1);
    let sq = iter::count(1u, 1).map(|x| x*x);

    Difference::new(ns, sq)
        .take_while(|&d| d <= 1000)
        .max_by(|&d| cont_frac::solve_pel::<BigUint>(d).val0())
        .unwrap()
        .to_string()
}

fn main() { Solver::new("661", solve).run(); }

