#![warn(unused, bad_style,
        unnecessary_qualification, unnecessary_typecast, unused_result)]

extern crate num;
extern crate common;

use std::char;
use std::iter::AdditiveIterator;
use num::bigint::BigInt;
use common::Solver;

fn compute(base: uint, exp: uint) -> uint {
    let base: BigInt = FromPrimitive::from_uint(base).unwrap();
    std::num::pow(base, exp)
        .to_string()
        .as_slice()
        .chars()
        .filter_map(|c| char::to_digit(c, 10))
        .sum()
}

fn solve() -> String { compute(2, 1000).to_string() }

fn main() { Solver::new("1366", solve).run(); }

#[cfg(test)]
mod tests {
    #[test]
    fn two_fifteen() {
        assert_eq!(26, super::compute(2, 15));
    }
}
