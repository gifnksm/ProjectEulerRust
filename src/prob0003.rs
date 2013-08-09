#[link(name = "prob0003", vers = "0.0")];
#[crate_type = "lib"];

extern mod common;

use std::iterator::OrdIterator;
use common::prime;

pub static EXPECTED_ANSWER: &'static str = "6857";

pub fn solve() -> ~str {
    let num = 600851475143;

    return prime::factorize(num)
        .transform(|(base, _exp)| base)
        .max()
        .unwrap()
        .to_str();
}
