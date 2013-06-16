#[link(name = "prob0020", vers = "0.0")];
#[crate_type = "lib"];

extern mod extra;
extern mod common;

use std::char;
use std::iterator::{AdditiveIterator, MultiplicativeIterator};
use extra::bigint::BigUint;
use common::extiter::Range;
use common::problem::Problem;

pub static problem: Problem<'static> = Problem {
    id: 20,
    answer: "648",
    solver: solve
};

pub fn solve() -> ~str {
    let s = Range::new::<uint>(1, 101)
        .transform(|n| BigUint::from_uint(n))
        .product().to_str();
    return s.iter()
        .filter_map(|c| char::to_digit(c, 10))
        .sum()
        .to_str();
}
