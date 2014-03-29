#![crate_id = "prob0122"]
#![crate_id = "prob0122"]
#![crate_type = "rlib"]
#![crate_type = "rlib"]

use std::{iter, uint, slice};

pub static EXPECTED_ANSWER: &'static str = "1582";

fn backtrack(power: uint, depth: uint, limit: uint, cost: &mut [uint], path: &mut [uint]) {
    if power > limit || depth > cost[power] { return }

    cost[power] = depth;
    path[depth] = power;

    for i in iter::range_inclusive(0, depth).rev() {
        backtrack(power + path[i], depth + 1, limit, cost, path);
    }
}

pub fn solve() -> ~str {
    let limit = 200;
    let mut cost = slice::from_elem(limit + 1, uint::MAX);
    let mut path = slice::from_elem(limit + 1, 0u);

    backtrack(1, 0, limit, cost, path);

    cost.slice(1, limit + 1)
        .iter()
        .fold(0, |x, &y| x + y)
        .to_str()
}
