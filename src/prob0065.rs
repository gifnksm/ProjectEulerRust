#[link(name = "prob0065", vers = "0.0")];
#[crate_type = "lib"];

extern mod extra;
extern mod common;

use std::{char, vec};
use std::iterator::AdditiveIterator;
use extra::bigint::BigUint;
use common::calc;

pub static expected_answer: &'static str = "272";

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
    let ns = n.to_str();
    return ns.iter()
        .filter_map(|c| char::to_digit(c, 10))
        .sum()
        .to_str();
}

