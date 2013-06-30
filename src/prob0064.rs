#[link(name = "prob0064", vers = "0.0")];
#[crate_type = "lib"];

extern mod common;

use std::uint;
use common::calc;

pub static expected_answer: &'static str = "1322";

pub fn solve() -> ~str {
    let mut cnt = 0u;
    for uint::range(1, 10001) |n| {
        let (_a0, an) = calc::cont_frac_sqrt(n);
        let period = an.len();
        if period % 2 == 1 { cnt += 1; }
    }
    return cnt.to_str();
}

