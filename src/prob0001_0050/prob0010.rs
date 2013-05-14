#[link(name = "prob0010", vers = "0.0")];
#[crate_type = "lib"];

extern mod common;

use core::iterator::{ IteratorUtil };
use common::prime;
use common::extiter::{ AdditiveIterator };
use common::problem::{ Problem };

pub static problem: Problem<'static> = Problem {
    id: 10,
    answer: "142913828922",
    solver: solve
};

pub fn solve() -> ~str {
    let limit = 2000000;
    return prime::iter()
        .take_while(|&p| p < limit)
        .sum()
        .to_str();
}
