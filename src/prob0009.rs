#[link(name = "prob0009", vers = "0.0")];
#[crate_type = "lib"];

extern mod common;

use std::uint;
use common::arith;

pub static EXPECTED_ANSWER: &'static str = "31875000";

fn each_pyrhagorean(sum: uint, f: &fn(uint, uint, uint) -> bool) -> bool {
    for c in range(2, sum - 2) {
        for a in range(1, uint::min((sum - c) / 2, arith::isqrt(c*c / 2))) {
            let b = sum - c - a;
            if a * a + b * b == c * c {
                if !f(a, b, c) { return false; }
            }
        }
    }

    return true;
}

pub fn solve() -> ~str {
    let mut ans = 0;
    do each_pyrhagorean(1000) |a, b, c| {
        ans = a * b * c;
        false
    };
    ans.to_str()
}
