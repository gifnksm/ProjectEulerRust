#[link(name = "prob0010", vers = "0.0")];
#[crate_type = "lib"];

extern mod common;

use std::iterator::AdditiveIterator;
use common::prime;

pub static expected_answer: &'static str = "142913828922";

pub fn solve() -> ~str {
    let limit = 2000000;
    return prime::iter()
        .take_while(|&p| p < limit)
        .sum()
        .to_str();
}
