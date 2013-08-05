#[link(name = "prob0069", vers = "0.0")];
#[crate_type = "lib"];

extern mod common;

use common::prime;

pub static EXPECTED_ANSWER: &'static str = "510510";

pub fn solve() -> ~str {
    let limit = 1000000;

    let mut n = 1;
    for p in prime::iter() {
        if n * p > limit { break; }
        n *= p;
    }

    return n.to_str();
}
