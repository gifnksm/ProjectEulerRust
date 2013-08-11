#[link(name = "prob0080", vers = "0.0")];
#[crate_type = "lib"];

extern mod extra;
extern mod common;

use std::char;
use std::iterator::AdditiveIterator;
use std::num::Zero;
use extra::bigint::BigInt;
use common::arith;
use common::extiter::Range;

pub static EXPECTED_ANSWER: &'static str = "40886";

fn sqrt_newton_raphson(n: uint, precision: uint) -> ~str {
    assert!(precision >= 1);

    let n = BigInt::from_uint(n);
    let _10 = BigInt::from_uint(10);
    let mut ds = BigInt::from_uint(1);
    do (precision - 1).times { ds = ds * _10; }

    let shift = 4 * precision; // log_2 10 = 3.3... < 4
    let _1_2 = BigInt::from_uint(1) << (2 * shift);
    let mut x_1 = (BigInt::from_uint(1) << shift) / _10;
    let mut delta_2 = (_1_2 - (x_1 * x_1 * n));

    loop {
        x_1 = ((x_1 << (2 * shift)) + ((x_1 * delta_2) >> 1)) >> (2 * shift);
        delta_2 = (_1_2 - (x_1 * x_1 * n));
        if ((ds * delta_2) >> (2 * shift)).is_zero() { break; }
    }

    return ((n * x_1 * ds) >> shift).to_str();
}

fn is_square(n: uint) -> bool {
    let isq = arith::isqrt(n);
    return isq * isq == n;
}

pub fn solve() -> ~str {
    return Range::new(2u, 101)
        .filter(|&n| !is_square(n))
        .map(|n| {
            let sqn = sqrt_newton_raphson(n, 100);
            sqn.iter()
                .filter_map(|c| char::to_digit(c, 10))
                .sum()
        }).sum()
        .to_str();
}
