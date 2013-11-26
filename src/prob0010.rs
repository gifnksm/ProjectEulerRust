#[link(name = "prob0010", vers = "0.0", package_id = "prob0010")];
#[crate_type = "lib"];

extern mod math;

use std::iter::AdditiveIterator;
use math::prime;

pub static EXPECTED_ANSWER: &'static str = "142913828922";

pub fn solve() -> ~str {
    let limit = 2000000;
    return prime::iter()
        .take_while(|&p| p < limit)
        .sum()
        .to_str();
}
