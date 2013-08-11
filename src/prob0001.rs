#[link(name = "prob0001", vers = "0.0")];
#[crate_type = "lib"];

use std::iterator;
use std::iterator::AdditiveIterator;

pub static EXPECTED_ANSWER: &'static str = "233168";

pub fn solve() -> ~str {
    iterator::range(0u, 1000)
        .filter(|&n| n % 3 == 0 || n % 5 == 0)
        .sum()
        .to_str()
}
