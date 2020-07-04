//! [Problem 80](https://projecteuler.net/problem=80) solver.

#![warn(
    bad_style,
    unused,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

use iter::Difference;
use num_bigint::BigInt;
use num_traits::{FromPrimitive, Zero};

#[allow(clippy::just_underscores_and_digits)]
fn sqrt_newton_raphson(n: u32, precision: usize) -> String {
    assert!(precision >= 1);

    let _1: BigInt = FromPrimitive::from_u32(1).unwrap();
    let _10: BigInt = FromPrimitive::from_u32(10).unwrap();
    let n: BigInt = FromPrimitive::from_u32(n).unwrap();

    let ds = num_traits::pow(_10.clone(), precision - 1);

    let shift = 4 * precision; // log_2 10 = 3.3... < 4
    let _1_2 = &_1 << (2 * shift);
    let mut x_1 = (&_1 << shift) / _10;
    let mut delta_2 = &_1_2 - (&x_1 * &x_1 * &n);

    loop {
        x_1 = ((&x_1 << (2 * shift)) + ((&x_1 * delta_2) >> 1)) >> (2 * shift);
        delta_2 = &_1_2 - (&x_1 * &x_1 * &n);
        if ((&ds * &delta_2) >> (2 * shift)).is_zero() {
            break;
        }
    }

    ((n * x_1 * ds) >> shift).to_string()
}

fn sqrt_digit_sum(n: u32, precision: usize) -> u32 {
    sqrt_newton_raphson(n, precision)
        .chars()
        .filter_map(|c| c.to_digit(10))
        .sum()
}

fn solve() -> String {
    let ns = 2u32..101;
    let sq = (2u32..101).map(|x| x * x);

    Difference::new(ns, sq)
        .map(|n| sqrt_digit_sum(n, 100))
        .sum::<u32>()
        .to_string()
}

common::problem!("40886", solve);

#[cfg(test)]
mod test {
    #[test]
    fn sqrt2() {
        assert_eq!("141421356237309504880", super::sqrt_newton_raphson(2, 21));
        assert_eq!(475, super::sqrt_digit_sum(2, 100));
    }
}
