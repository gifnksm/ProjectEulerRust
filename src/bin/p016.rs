//! [Problem 16](https://projecteuler.net/problem=16) solver.

#![warn(bad_style,
        unused, unused_extern_crates, unused_import_braces,
        unused_qualifications, unused_results, unused_typecasts)]

#![feature(collections, core, unicode)]

extern crate num;
#[macro_use(problem)] extern crate common;

use std::iter::AdditiveIterator;
use std::num::FromPrimitive;
use num::bigint::BigInt;

fn compute(base: u32, exp: u32) -> usize {
    let base: BigInt = FromPrimitive::from_u32(base).unwrap();
    num::pow(base, exp as usize)
        .to_string()
        .chars()
        .filter_map(|c| c.to_digit(10))
        .sum()
}

fn solve() -> String { compute(2, 1000).to_string() }

problem!("1366", solve);

#[cfg(test)]
mod tests {
    #[test]
    fn two_fifteen() {
        assert_eq!(26, super::compute(2, 15));
    }
}
