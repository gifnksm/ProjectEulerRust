//! [Problem 14](https://projecteuler.net/problem=14) solver.

#![warn(bad_style,
        unused, unused_extern_crates, unused_import_braces,
        unused_qualifications, unused_results, unused_typecasts)]

#![feature(phase)]

#[phase(plugin, link)] extern crate common;
extern crate seq;

use std::collections::HashMap;
use seq::Collatz;

fn compute_len(map: &mut HashMap<uint, uint>, n: uint) -> uint {
    if let Some(&x) = map.get(&n) {
        return x
    }

    let mut it = Collatz::new(n);
    let _ = it.next();
    let x = compute_len(map, it.next().unwrap()) + 1;
    let _ = map.insert(n, x);
    x
}

fn compute(limit: uint) -> uint {
    let mut map = HashMap::with_capacity(limit);
    let _ = map.insert(1, 1u);

    range(2u, limit)
        .max_by(|&n| compute_len(&mut map, n))
        .unwrap()
}

fn solve() -> String { compute(1000000).to_string() }

problem!("837799", solve);
