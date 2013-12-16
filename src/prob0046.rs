#[crate_type = "rlib"];

extern mod math;

use std::iter;
use math::{arith, prime};

pub static EXPECTED_ANSWER: &'static str = "5777";

fn is_goldbach(n: uint) -> bool {
    for s in range(1, arith::isqrt(n / 2) + 1) {
        let sq = s * s * 2;
        if sq > n { return false; }
        if prime::contains(n - sq) { return true; }
    }
    return false;
}

pub fn solve() -> ~str {
    return iter::count(3u, 2)
        .filter(|&n| !prime::contains(n))
        .skip_while(|&n| is_goldbach(n))
        .next()
        .unwrap()
        .to_str();
}
