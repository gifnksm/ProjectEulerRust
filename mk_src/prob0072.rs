#![crate_name = "prob0072"]
#![crate_type = "rlib"]

extern crate math;

use std::iter;
use std::iter::AdditiveIterator;
use math::prime::Prime;

pub const EXPECTED_ANSWER: &'static str = "303963552391";

pub fn solve() -> String {
    let limit = 1000000;
    let prime = Prime::new();

    let mut v = Vec::from_fn(limit + 1, |n| n);
    v[1] = 0;

    for p in prime.iter() {
        if p > limit { break; }
        for n in iter::range_step(p, limit + 1, p) {
            v[n] = (v[n] * (p - 1)) / p;
        }
    }

    v.into_iter().sum().to_string()
}
