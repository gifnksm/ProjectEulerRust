//! [Problem 14](https://projecteuler.net/problem=14) solver.

#![warn(
    bad_style,
    unused,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

use seq::Collatz;
use std::collections::HashMap;

fn compute_len(map: &mut HashMap<u64, u64>, n: u64) -> u64 {
    if let Some(&x) = map.get(&n) {
        return x;
    }

    let mut it = Collatz::new(n);
    let _ = it.next();
    let x = compute_len(map, it.next().unwrap()) + 1;
    let _ = map.insert(n, x);
    x
}

fn compute(limit: u64) -> u64 {
    let mut map = HashMap::with_capacity(limit as usize);
    let _ = map.insert(1, 1);

    (2..limit)
        .max_by_key(|&n| compute_len(&mut map, n))
        .unwrap()
}

fn solve() -> String {
    compute(1000000).to_string()
}

common::problem!("837799", solve);
