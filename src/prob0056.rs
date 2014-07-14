#![crate_name = "prob0056"]
#![crate_type = "rlib"]

extern crate num;

use std::char;
use std::iter::{AdditiveIterator, Unfold};
use std::num::One;
use num::bigint::BigUint;

pub static EXPECTED_ANSWER: &'static str = "972";

pub fn solve() -> String {
    range(One::one(), FromPrimitive::from_uint(100).unwrap())
        .map(|a: BigUint| {
            Unfold::new(One::one(), |n| { (*n) = a * (*n); Some((*n).to_string())})
                .map(|s| s.as_slice().chars().filter_map(|c| char::to_digit(c, 10)).sum())
                .take(100).max().unwrap()
        }).max().unwrap().to_string()
}
