#[crate_type = "lib"];

use std::iter::AdditiveIterator;

pub static EXPECTED_ANSWER: &'static str = "233168";

pub fn solve() -> ~str {
    range(0u, 1000)
        .filter(|&n| n % 3 == 0 || n % 5 == 0)
        .sum()
        .to_str()
}
