#[crate_id = "prob0016"];
#[crate_type = "rlib"];

extern crate num;

use std::char;
use std::iter::AdditiveIterator;
use num::bigint::BigInt;

pub static EXPECTED_ANSWER: &'static str = "1366";

pub fn solve() -> ~str {
    let two: BigInt = FromPrimitive::from_uint(2).unwrap();
    std::num::pow(two, 1000)
        .to_str()
        .chars()
        .filter_map(|c| char::to_digit(c, 10))
        .sum()
        .to_str()
}

