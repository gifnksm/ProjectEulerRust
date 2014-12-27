#![warn(bad_style,
        unused, unused_extern_crates, unused_import_braces,
        unused_qualifications, unused_results, unused_typecasts)]

#![feature(phase, slicing_syntax)]

#[phase(plugin, link)] extern crate common;

use std::{iter, uint};

fn backtrack(power: uint, depth: uint, limit: uint, cost: &mut [uint], path: &mut [uint]) {
    if power > limit || depth > cost[power] { return }

    cost[power] = depth;
    path[depth] = power;

    for i in iter::range_inclusive(0, depth).rev() {
        backtrack(power + path[i], depth + 1, limit, cost, path);
    }
}

fn compute_cost(limit: uint) -> Vec<uint> {
    let mut cost = Vec::from_elem(limit + 1, uint::MAX);
    let mut path = Vec::from_elem(limit + 1, 0u);

    backtrack(1, 0, limit, cost.as_mut_slice(), path.as_mut_slice());

    cost
}

fn solve() -> String {
    let limit = 200;
    compute_cost(200)[1 .. limit + 1]
        .iter()
        .fold(0, |x, &y| x + y)
        .to_string()
    }

problem!("1582", solve);

#[cfg(test)]
mod tests {
    #[test]
    fn m15() {
        let limit = 15;
        let cost = super::compute_cost(limit);
        assert_eq!(5, cost[limit]);
    }
}
