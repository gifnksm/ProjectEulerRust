#[link(name = "prob0075", vers = "0.0")];
#[crate_type = "lib"];

extern mod common;

use std::{uint, vec};
use common::arith;
use common::calc;
use common::problem::{Problem};

pub static problem: Problem<'static> = Problem {
    id: 75,
    answer: "161667",
    solver: solve
};

pub fn solve() -> ~str {
    let limit = 1500000;
    let mut v = vec::from_elem(limit + 1, 0);
    for uint::range(2, arith::isqrt(limit / 2)) |m| {
        for calc::each_prim_pythagorean(m) |a, b, c| {
            let sum = a + b + c;
            for uint::range_step(sum, limit + 1, sum as int) |s| {
                v[s] += 1;
            }
        }
    }

    return v.count(&1).to_str();
}
