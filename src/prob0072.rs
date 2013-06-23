#[link(name = "prob0072", vers = "0.0")];
#[crate_type = "lib"];

extern mod common;

use std::{uint, vec};
use std::iterator::AdditiveIterator;
use common::prime;
use common::problem::Problem;

pub static problem: Problem<'static> = Problem {
    id: 72,
    answer: "303963552391",
    solver: solve
};

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
