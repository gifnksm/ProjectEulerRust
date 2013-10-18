#[link(name = "prob0020", vers = "0.0")];
#[crate_type = "lib"];

extern mod extra;

use std::char;
use std::iter::{AdditiveIterator, MultiplicativeIterator};
use extra::bigint::BigUint;

pub static EXPECTED_ANSWER: &'static str = "648";

pub fn solve() -> ~str {
    let s = range::<BigUint>(FromPrimitive::from_uint(1).unwrap(),
                             FromPrimitive::from_uint(101).unwrap())
        .product()
        .to_str();
    return s.iter()
        .filter_map(|c| char::to_digit(c, 10))
        .sum()
        .to_str();
}
