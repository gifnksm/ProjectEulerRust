//! [Problem 75](https://projecteuler.net/problem=75) solver.

#![warn(bad_style,
        unused, unused_extern_crates, unused_import_braces,
        unused_qualifications, unused_results, unused_typecasts)]

#![feature(core)]

#[macro_use(problem)] extern crate common;
extern crate integer;
extern crate seq;

use std::iter;
use integer::Integer;
use seq::PrimitivePythagoreans;

fn solve() -> String {
    let limit = 1500000u64;
    let mut v = vec![0; (limit + 1) as usize];

    for m in (2 .. (limit / 2).sqrt()) {
        for (a, b, c) in PrimitivePythagoreans::new(m) {
            let sum = a + b + c;
            for s in iter::range_step(sum, limit + 1, sum) {
                v[s as usize] += 1;
            }
        }
    }

    v.iter()
        .filter(|&x| x == &1)
        .count()
        .to_string()
}

problem!("161667", solve);

