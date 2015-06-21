//! [Problem 20](https://projecteuler.net/problem=20) solver.

#![warn(bad_style,
        unused, unused_extern_crates, unused_import_braces,
        unused_qualifications, unused_results)]

#![feature(iter_arith)]

extern crate num;
#[macro_use(problem)] extern crate common;

use num::{BigUint, FromPrimitive};

fn compute(max: u32) -> u32 {
    num::range::<BigUint>(FromPrimitive::from_u32(1).unwrap(),
                          FromPrimitive::from_u32(max + 1).unwrap())
        .fold(num::one::<BigUint>(), |acc, elt| acc * elt)
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
