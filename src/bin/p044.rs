//! [Problem 44](https://projecteuler.net/problem=44) solver.

#![warn(bad_style,
        unused, unused_extern_crates, unused_import_braces,
        unused_qualifications, unused_results, unused_typecasts)]

#![feature(phase, slicing_syntax)]

#[phase(plugin, link)] extern crate common;

use std::iter;

fn nth_pentagonal(i: uint) -> uint {
    let n = i + 1;
    n * (3 * n - 1) / 2
}

fn is_pentagonal(n: uint, table: &[uint]) -> bool {
    if *table.last().unwrap() < n { panic!() }
    table.binary_search_elem(&n).found().is_some()
}

// P[k] + P[j] = P[m]
// P[k] - P[j] = P[n]
//
// 2*P[k] = P[m] + P[n] > 0
// 2*P[j] = P[m] - P[n] > 0
//
// find minimum n, where n < m
fn solve() -> String {
    let limit = 10000;
    let pentagonals = Vec::from_fn(limit, nth_pentagonal);

    for m in iter::count(0, 1) {
        let pm = pentagonals[m];
        for n in range(0, m) {
            let pn = pentagonals[n];
            if (pm - pn) % 2 != 0 { continue }
            if is_pentagonal(pm - pn, pentagonals[]) {
                if is_pentagonal(pm + pn, pentagonals[]) {
                    return (pm - pn).to_string()
                }
            }
        }
    }
    unreachable!()
}

problem!("5482660", solve);
