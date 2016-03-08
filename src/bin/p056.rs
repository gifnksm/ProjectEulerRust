//! [Problem 56](https://projecteuler.net/problem=56) solver.

#![warn(bad_style,
        unused, unused_extern_crates, unused_import_braces,
        unused_qualifications, unused_results)]

#![feature(iter_arith)]

#[macro_use(problem)]
extern crate common;
extern crate num;
extern crate itertools;

use num::{BigUint, FromPrimitive, One};
use itertools::Unfold;

fn compute(a: u32, b: u32) -> u32 {
    num::range(One::one(), FromPrimitive::from_u32(a).unwrap())
        .map(|a: BigUint| {
            Unfold::new(One::one(), |n| {
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

problem!("972", solve);
