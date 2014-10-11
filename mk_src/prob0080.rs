#![crate_name = "prob0080"]
#![crate_type = "rlib"]

extern crate num;
extern crate math;

use std::char;
use std::iter::AdditiveIterator;
use std::num::Zero;
use num::bigint::BigInt;
use math::arith;

pub const EXPECTED_ANSWER: &'static str = "40886";

fn sqrt_newton_raphson(n: uint, precision: uint) -> String {
    assert!(precision >= 1);

    let _1:  BigInt = FromPrimitive::from_uint(1).unwrap();
    let _10: BigInt = FromPrimitive::from_uint(10).unwrap();
    let n:   BigInt = FromPrimitive::from_uint(n).unwrap();

    let ds = std::num::pow(_10.clone(), precision - 1);

    let shift   = 4 * precision; // log_2 10 = 3.3... < 4
    let _1_2    = _1 << (2 * shift);
    let mut x_1 = (_1 << shift) / _10;
    let mut delta_2 = _1_2 - (x_1 * x_1 * n);

    loop {
        x_1 = ((x_1 << (2 * shift)) + ((x_1 * delta_2) >> 1)) >> (2 * shift);
        delta_2 = _1_2 - (x_1 * x_1 * n);
        if ((ds * delta_2) >> (2 * shift)).is_zero() { break; }
    }

    return ((n * x_1 * ds) >> shift).to_string();
}

fn is_square(n: uint) -> bool {
    let isq = arith::isqrt(n);
    return isq * isq == n;
}

pub fn solve() -> String {
    return range(2u, 101)
        .filter(|&n| !is_square(n))
        .map(|n| {
            sqrt_newton_raphson(n, 100)
                .as_slice()
                .chars()
                .filter_map(|c| char::to_digit(c, 10))
                .sum()
        }).sum()
        .to_string();
}
