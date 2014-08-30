#![crate_name = "prob0001"]
#![crate_type = "rlib"]

use std::iter::AdditiveIterator;

pub static EXPECTED_ANSWER: &'static str = "233168";

pub fn solve() -> String {
    range(0u, 1000)
        .filter(|&n| n % 3 == 0 || n % 5 == 0)
        .sum()
        .to_string()
}