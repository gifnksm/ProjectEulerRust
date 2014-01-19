#[crate_id = "prob0056"];
#[crate_type = "rlib"];

extern mod extra;

use std::char;
use std::iter::{AdditiveIterator, Unfold};
use std::num::One;
use extra::bigint::BigUint;

pub static EXPECTED_ANSWER: &'static str = "972";

pub fn solve() -> ~str {
    range(One::one(), FromPrimitive::from_uint(100).unwrap())
        .map(|a: BigUint| {
            Unfold::new(One::one(), |n| { (*n) = a * (*n); Some(n.to_str())})
                .map(|s| s.chars().filter_map(|c| char::to_digit(c, 10)).sum())
                .take(100).max().unwrap()
        }).max().unwrap().to_str()
}
