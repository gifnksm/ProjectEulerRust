#[link(name = "prob0046", vers = "0.0")];
#[crate_type = "lib"];

extern mod common;

use std::iterator::Counter;
use common::{arith, prime};

pub static EXPECTED_ANSWER: &'static str = "5777";

fn is_goldbach(n: uint) -> bool {
    foreach s in range(1, arith::isqrt(n / 2) + 1) {
        let sq = s * s * 2;
        if sq > n { return false; }
        if prime::contains(n - sq) { return true; }
    }
    return false;
}

pub fn solve() -> ~str {
    return Counter::new::<uint>(3, 2)
        .filter(|&n| !prime::contains(n))
        .skip_while(|&n| is_goldbach(n))
        .next()
        .get()
        .to_str();
}
