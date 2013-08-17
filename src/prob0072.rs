#[link(name = "prob0072", vers = "0.0")];
#[crate_type = "lib"];

extern mod math;

use std::{uint, vec};
use std::iterator::AdditiveIterator;
use math::prime;

pub static EXPECTED_ANSWER: &'static str = "303963552391";

pub fn solve() -> ~str {
    let limit = 1000000;

    let mut v = vec::from_fn(limit + 1, |n| n);
    v[1] = 0;

    for p in prime::iter() {
        if p > limit { break; }
        do uint::range_step(p, limit + 1, p as int) |n| {
            v[n] = v[n] * (p - 1) / p;
            true
        };
    }

    return v.move_iter().sum().to_str();
}
