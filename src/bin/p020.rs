//! [Problem 20](https://projecteuler.net/problem=20) solver.

#![warn(
    bad_style,
    unused,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

use num_bigint::BigUint;
use num_traits::FromPrimitive;

fn compute(max: u32) -> u32 {
    num_iter::range::<BigUint>(
        FromPrimitive::from_u32(1).unwrap(),
        FromPrimitive::from_u32(max + 1).unwrap(),
    )
    .fold(num_traits::one::<BigUint>(), |acc, elt| acc * elt)
    .to_string()
    .chars()
    .filter_map(|c| c.to_digit(10))
    .sum()
}

fn solve() -> String {
    compute(100).to_string()
}

common::problem!("648", solve);

#[cfg(test)]
mod tests {
    #[test]
    fn ten() {
        assert_eq!(27, super::compute(10));
    }
}
