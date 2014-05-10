#![crate_id = "prob0009"]
#![crate_type = "rlib"]

extern crate math;

use std::iter::Repeat;
use std::cmp;
use math::arith;

pub static EXPECTED_ANSWER: &'static str = "31875000";

pub fn solve() -> ~str {
    let sum = 1000u;

    range(2, sum - 1)
        .flat_map(|c| {
            let a_max = cmp::min((sum - c) / 2, arith::isqrt(c * c / 2));
            range(1, a_max).zip(Repeat::new(c))
        }).map(|(a, c)| (a, sum - c - a, c))
        .find(|&(a, b, c)| a * a + b * b == c * c)
        .map(|(a, b, c)| a * b * c)
        .unwrap()
        .to_str()
}
