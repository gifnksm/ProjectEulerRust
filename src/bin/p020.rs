//! [Problem 20](https://projecteuler.net/problem=20) solver.

#![warn(bad_style,
        unused, unused_extern_crates, unused_import_braces,
        unused_qualifications, unused_results, unused_typecasts)]

#![feature(phase, slicing_syntax)]

extern crate num;
#[phase(plugin, link)] extern crate common;

use std::iter::{AdditiveIterator, MultiplicativeIterator};
use num::bigint::BigUint;

fn compute(max: uint) -> uint {
    num::range::<BigUint>(FromPrimitive::from_uint(1).unwrap(),
                          FromPrimitive::from_uint(max + 1).unwrap())
        .product()
        .to_string()
        .chars()
        .filter_map(|c| c.to_digit(10))
        .sum()
}

fn solve() -> String { compute(100).to_string() }

problem!("648", solve);

#[cfg(test)]
mod tests {
    #[test]
    fn ten() {
        assert_eq!(27, super::compute(10));
    }
}
