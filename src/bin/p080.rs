#![warn(bad_style,
        unused, unused_extern_crates, unused_import_braces,
        unused_qualifications, unused_results, unused_typecasts)]

#![feature(phase)]

extern crate num;
#[phase(plugin, link)] extern crate common;
extern crate iter;

use std::iter::AdditiveIterator;
use num::{BigInt, Zero};
use iter::Difference;

fn sqrt_newton_raphson(n: uint, precision: uint) -> String {
    assert!(precision >= 1);

    let _1:  BigInt = FromPrimitive::from_uint(1).unwrap();
    let _10: BigInt = FromPrimitive::from_uint(10).unwrap();
    let n:   BigInt = FromPrimitive::from_uint(n).unwrap();

    let ds = num::pow(_10.clone(), precision - 1);

    let shift   = 4 * precision; // log_2 10 = 3.3... < 4
    let _1_2    = &_1 << (2 * shift);
    let mut x_1 = (&_1 << shift) / _10;
    let mut delta_2 = &_1_2 - (&x_1 * &x_1 * &n);

    loop {
        x_1 = ((&x_1 << (2 * shift)) + ((&x_1 * delta_2) >> 1)) >> (2 * shift);
        delta_2 = &_1_2 - (&x_1 * &x_1 * &n);
        if ((&ds * &delta_2) >> (2 * shift)).is_zero() { break; }
    }

    ((n * x_1 * ds) >> shift).to_string()
}

fn sqrt_digit_sum(n: uint, precision: uint) -> uint {
    sqrt_newton_raphson(n, precision)
        .chars()
        .filter_map(|c| c.to_digit(10))
        .sum()
}

fn solve() -> String {
    let ns = range(2u, 101);
    let sq = range(2u, 101).map(|x| x*x);

    Difference::new(ns, sq)
        .map(|n| sqrt_digit_sum(n, 100))
        .sum()
        .to_string()
}

problem!("40886", solve);

#[cfg(test)]
mod test {
    #[test]
    fn sqrt2() {
        assert_eq!("141421356237309504880", super::sqrt_newton_raphson(2, 21)[]);
        assert_eq!(475, super::sqrt_digit_sum(2, 100));
    }
}
