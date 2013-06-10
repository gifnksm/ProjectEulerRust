#[link(name = "prob0016", vers = "0.0")];
#[crate_type = "lib"];

extern mod extra;
extern mod common;

use std::{char};
use std::num::{One};
use std::iterator::{AdditiveIterator, IteratorUtil};
use extra::bigint::{BigInt};
use common::problem::{Problem};

pub static problem: Problem<'static> = Problem {
    id: 16,
    answer: "1366",
    solver: solve
};

pub fn solve() -> ~str {
    let mut i = One::one::<BigInt>();
    for 1000.times { i = i * BigInt::from_uint(2); }
    let s = i.to_str();
    return s.iter()
        .filter_map(|c| char::to_digit(c, 10))
        .sum()
        .to_str();
}

