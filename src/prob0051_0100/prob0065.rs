#[link(name = "prob0065", vers = "0.0")];
#[crate_type = "lib"];

extern mod extra;
extern mod common;

use std::{char, str, vec};
use extra::bigint::{BigUint};
use common::calc;
use common::problem::{Problem};

pub static problem: Problem<'static> = Problem {
    id: 65,
    answer: "272",
    solver: solve
};

fn napier_seq(i: uint) -> uint {
    match i {
        0 => 2,
        i if i % 3 == 2 => 2 * (i + 1) / 3,
        _ => 1
    }
}

pub fn solve() -> ~str {
    let len = 100;

    let napier = vec::from_fn(len, napier_seq);

    let (n, _d) = calc::fold_cont_frac::<BigUint>(napier);
    let mut sum = 0;
    for str::each_char(n.to_str()) |c| {
        sum += char::to_digit(c, 10).get();
    }
    return sum.to_str();
}

