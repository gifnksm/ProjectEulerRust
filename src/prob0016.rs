#[crate_id = "prob0016"];
#[crate_type = "rlib"];

extern mod extra;

use std::{char, num};
use std::iter::AdditiveIterator;
use extra::bigint::BigInt;

pub static EXPECTED_ANSWER: &'static str = "1366";

pub fn solve() -> ~str {
    let two: BigInt = FromPrimitive::from_uint(2).unwrap();
    num::pow(two, 1000)
        .to_str()
        .chars()
        .filter_map(|c| char::to_digit(c, 10))
        .sum()
        .to_str()
}

