#![crate_id = "prob0046"]
#![crate_type = "rlib"]

extern crate math;

use std::iter;
use math::arith;
use math::prime::Prime;

pub static EXPECTED_ANSWER: &'static str = "5777";

fn is_goldbach(prime: &Prime, n: uint) -> bool {
    for s in range(1, arith::isqrt(n / 2) + 1) {
        let sq = s * s * 2;
        if sq > n { return false; }
        if prime.contains(n - sq) { return true; }
    }
    return false;
}

pub fn solve() -> ~str {
    let prime = Prime::new();
    return iter::count(3u, 2)
        .filter(|&n| !prime.contains(n))
        .skip_while(|&n| is_goldbach(&prime, n))
        .next()
        .unwrap()
        .to_str();
}
