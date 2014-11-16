#![warn(bad_style,
        unused, unused_extern_crates, unused_import_braces,
        unused_qualifications, unused_results, unused_typecasts)]

#![feature(slicing_syntax)]

extern crate num;
extern crate common;

use std::char;
use std::iter::{AdditiveIterator, MultiplicativeIterator};
use num::bigint::BigUint;
use common::Solver;

fn compute(max: uint) -> uint {
    num::range::<BigUint>(FromPrimitive::from_uint(1).unwrap(),
                          FromPrimitive::from_uint(max + 1).unwrap())
        .product()
        .to_string()
        []
        .chars()
        .filter_map(|c| char::to_digit(c, 10))
        .sum()
}

fn solve() -> String { compute(100).to_string() }

fn main() { Solver::new("648", solve).run(); }

#[cfg(test)]
mod tests {
    #[test]
    fn ten() {
        assert_eq!(27, super::compute(10));
    }
}
