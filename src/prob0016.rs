#[crate_type = "rlib"];

extern mod extra;

use std::char;
use std::num::One;
use std::iter::AdditiveIterator;
use extra::bigint::BigInt;

pub static EXPECTED_ANSWER: &'static str = "1366";

pub fn solve() -> ~str {
    let mut i: BigInt = One::one();
    let two = FromPrimitive::from_uint(2).unwrap();
    1000.times(|| i = i * two);
    let s = i.to_str();
    return s.chars()
        .filter_map(|c| char::to_digit(c, 10))
        .sum()
        .to_str();
}

