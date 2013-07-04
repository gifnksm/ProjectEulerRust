#[link(name = "prob0001", vers = "0.0")];
#[crate_type = "lib"];

extern mod common;

use std::iterator::AdditiveIterator;
use common::extiter::Range;

pub static EXPECTED_ANSWER: &'static str = "233168";

pub fn solve() -> ~str {
    Range::new(0, 1000)
        .filter(|&n| n % 3 == 0 || n % 5 == 0)
        .sum()
        .to_str()
}
