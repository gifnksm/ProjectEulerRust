//! [Problem 16](https://projecteuler.net/problem=16) solver.

#![warn(
    bad_style,
    unused,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

use num_bigint::BigInt;
use num_traits::FromPrimitive;

fn compute(base: u32, exp: u32) -> u32 {
    let base: BigInt = FromPrimitive::from_u32(base).unwrap();
    num_traits::pow(base, exp as usize)
        .to_string()
        .chars()
        .filter_map(|c| c.to_digit(10))
        .sum()
}

fn solve() -> String {
    compute(2, 1000).to_string()
}

common::problem!("1366", solve);

#[cfg(test)]
mod tests {
    #[test]
    fn two_fifteen() {
        assert_eq!(26, super::compute(2, 15));
    }
}
