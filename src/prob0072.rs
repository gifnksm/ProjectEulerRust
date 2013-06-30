#[link(name = "prob0072", vers = "0.0")];
#[crate_type = "lib"];

extern mod common;

use std::{uint, vec};
use std::iterator::AdditiveIterator;
use common::prime;

pub static expected_answer: &'static str = "303963552391";

pub fn solve() -> ~str {
    let limit = 1000000;

    let mut v = vec::from_fn(limit + 1, |n| n);
    v[1] = 0;

    for prime::each |p| {
        if p > limit { break; }
        for uint::range_step(p, limit + 1, p as int) |n| {
            v[n] = v[n] * (p - 1) / p;
        }
    }

    return v.iter().transform(|&x| x).sum().to_str();
}
