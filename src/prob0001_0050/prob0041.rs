#[link(name = "prob0041", vers = "0.0")];
#[crate_type = "lib"];

extern mod common;

use std::util;
use common::calc;
use common::prime;
use common::problem::{Problem};

pub static problem: Problem<'static> = Problem {
    id: 41,
    answer: "7652413",
    solver: solve
};

pub fn solve() -> ~str {
    for calc::permutate_num(&[7, 6, 5, 4, 3, 2, 1], 7, 0, 9999999) |num, _rest| {
        if prime::contains(num) { return num.to_str(); }
    }

    util::unreachable();
}
