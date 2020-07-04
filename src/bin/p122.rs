//! [Problem 122](https://projecteuler.net/problem=122) solver.

#![warn(
    bad_style,
    unused,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

use std::u32;

fn backtrack(power: u32, depth: u32, limit: u32, cost: &mut [u32], path: &mut [u32]) {
    if power > limit || depth > cost[power as usize] {
        return;
    }

    cost[power as usize] = depth;
    path[depth as usize] = power;

    for i in (0..(depth + 1)).rev() {
        backtrack(power + path[i as usize], depth + 1, limit, cost, path);
    }
}

fn compute_cost(limit: u32) -> Vec<u32> {
    let mut cost = vec![u32::MAX; (limit + 1) as usize];
    let mut path = vec![0; (limit + 1) as usize];

    backtrack(1, 0, limit, &mut cost, &mut path);

    cost
}

fn solve() -> String {
    let limit = 200;
    compute_cost(limit)[1..(limit as usize) + 1]
        .iter()
        .sum::<u32>()
        .to_string()
}

common::problem!("1582", solve);

#[cfg(test)]
mod tests {
    #[test]
    fn m15() {
        let limit = 15;
        let cost = super::compute_cost(limit);
        assert_eq!(5, cost[limit as usize]);
    }
}
