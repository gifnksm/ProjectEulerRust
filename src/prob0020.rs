#![crate_name = "prob0020"]
#![crate_type = "rlib"]

extern crate num;

use std::char;
use std::iter::{AdditiveIterator, MultiplicativeIterator};
use num::bigint::BigUint;

pub static EXPECTED_ANSWER: &'static str = "648";

pub fn solve() -> String {
    range::<BigUint>(FromPrimitive::from_uint(1).unwrap(),
                     FromPrimitive::from_uint(101).unwrap())
        .product()
        .to_str()
        .as_slice()
        .chars()
        .filter_map(|c| char::to_digit(c, 10))
        .sum()
        .to_str()
}
