#[link(name = "prob0016", vers = "0.0")];
#[crate_type = "lib"];

extern mod extra;

use std::char;
use std::num::One;
use std::iterator::AdditiveIterator;
use extra::bigint::BigInt;

pub static EXPECTED_ANSWER: &'static str = "1366";

pub fn solve() -> ~str {
    let mut i = One::one::<BigInt>();
    do 1000.times { i = i * BigInt::from_uint(2); }
    let s = i.to_str();
    return s.iter()
        .filter_map(|c| char::to_digit(c, 10))
        .sum()
        .to_str();
}

