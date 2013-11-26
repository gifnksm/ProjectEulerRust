#[link(name = "prob0001", vers = "0.0", package_id = "prob0001")];
#[crate_type = "lib"];

use std::iter::AdditiveIterator;

pub static EXPECTED_ANSWER: &'static str = "233168";

pub fn solve() -> ~str {
    range(0u, 1000)
        .filter(|&n| n % 3 == 0 || n % 5 == 0)
        .sum()
        .to_str()
}
