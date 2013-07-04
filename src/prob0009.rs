#[link(name = "prob0009", vers = "0.0")];
#[crate_type = "lib"];

extern mod common;

use std::{uint, util};
use common::arith;

pub static EXPECTED_ANSWER: &'static str = "31875000";

fn each_pyrhagorean(sum: uint, f: &fn(uint, uint, uint) -> bool) -> bool {
    for uint::range(2, sum - 2) |c| {
        for uint::range(1, uint::min((sum - c) / 2, arith::isqrt(c*c / 2))) |a| {
            let b = sum - c - a;
            if a * a + b * b == c * c {
                if !f(a, b, c) { return false; }
            }
        }
    }

    return true;
}

pub fn solve() -> ~str {
    for each_pyrhagorean(1000) |a, b, c| {
        return (a * b * c).to_str();
    }

    util::unreachable();
}
