#![warn(unused, bad_style,
        unnecessary_qualification, unnecessary_typecast, unused_result)]

extern crate common;

use std::iter::AdditiveIterator;
use common::Solver;

fn compute(bound: uint) -> uint {
    range(1, bound)
        .filter(|&n| n % 3 == 0 || n % 5 == 0)
        .sum()
}

fn solve() -> String {
    compute(1000).to_string()
}

fn main() {
    Solver::new("233168", solve).run();
}

#[cfg(test)]
mod tests {
    #[test]
    fn sum_below_ten() { assert_eq!(23, super::compute(10)); }
}
