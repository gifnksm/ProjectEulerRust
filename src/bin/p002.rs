#![warn(unused, bad_style,
        unused_qualifications, unused_typecasts, unused_results)]

extern crate num;
extern crate common;
extern crate seq;

use std::iter::AdditiveIterator;
use num::Integer;
use common::Solver;
use seq::Fibonacci;

fn compute(bound: uint) -> uint {
    Fibonacci::<uint>::new()
        .take_while(|&f| f < bound)
        .filter(|&f| f.is_even())
        .sum()
}

fn solve() -> String { compute(4000000).to_string() }
fn main() { Solver::new("4613732", solve).run(); }

#[cfg(test)]
mod tests {
    #[test]
    fn sum_below_90() { assert_eq!(44, super::compute(90)); }
}
