#![crate_name = "prob0122"]
#![crate_type = "rlib"]

use std::{iter, uint};

pub static EXPECTED_ANSWER: &'static str = "1582";

fn backtrack(power: uint, depth: uint, limit: uint, cost: &mut [uint], path: &mut [uint]) {
    if power > limit || depth > cost[power] { return }

    cost[power] = depth;
    path[depth] = power;

    for i in iter::range_inclusive(0, depth).rev() {
        backtrack(power + path[i], depth + 1, limit, cost, path);
    }
}

pub fn solve() -> String {
    let limit = 200;
    let mut cost = Vec::from_elem(limit + 1, uint::MAX);
    let mut path = Vec::from_elem(limit + 1, 0u);

    backtrack(1, 0, limit, cost.as_mut_slice(), path.as_mut_slice());

    cost.slice(1, limit + 1)
        .iter()
        .fold(0, |x, &y| x + y)
        .to_str()
}
