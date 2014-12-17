#![warn(bad_style,
        unused, unused_extern_crates, unused_import_braces,
        unused_qualifications, unused_results, unused_typecasts)]

extern crate common;
extern crate num;

use std::iter::{AdditiveIterator, Unfold};
use common::Solver;
use num::{One, BigUint};

fn compute(a: uint, b: uint) -> uint {
    num::range(One::one(), FromPrimitive::from_uint(a).unwrap())
        .map(|a: BigUint| {
            Unfold::new(One::one(), |n| { (*n) = &a * (&*n); Some(n.to_string()) })
                .map(|s| s.chars().filter_map(|c| c.to_digit(10)).sum())
                .take(b)
                .max()
                .unwrap()
        }).max()
        .unwrap()
}

fn solve() -> String {
    compute(100, 100).to_string()
}

fn main() { Solver::new("972", solve).run(); }
