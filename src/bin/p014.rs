#![warn(unused, bad_style,
        unused_qualifications, unused_typecasts, unused_results)]

extern crate common;
extern crate seq;

use std::collections::HashMap;
use common::Solver;
use seq::Collatz;

fn compute_len(map: &mut HashMap<uint, uint>, n: uint) -> uint {
    match map.find(&n) {
        Some(&x) => return x,
        None => {}
    }

    let mut it = Collatz::new(n);
    let _ = it.next();
    let x = compute_len(map, it.next().unwrap()) + 1;
    map.insert(n, x);
    x
}

fn compute(limit: uint) -> uint {
    let mut map = HashMap::with_capacity(limit);
    map.insert(1, 1u);

    range(2u, limit)
        .max_by(|&n| compute_len(&mut map, n))
        .unwrap()
}

fn solve() -> String { compute(1000000).to_string() }

fn main() { Solver::new("837799", solve).run(); }
