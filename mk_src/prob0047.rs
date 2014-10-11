#![crate_name = "prob0047"]
#![crate_type = "rlib"]

extern crate math;

use std::iter;
use math::prime::Prime;

pub const EXPECTED_ANSWER: &'static str = "134043";

pub fn solve() -> String {
    let len = 4;
    let num_factor = 4;
    let prime = Prime::new();

    let mut cnt = 0;
    for n in iter::count(1u, 1) {
        if prime.factorize(n).count() != num_factor {
            cnt = 0;
            continue
        }

        cnt += 1;
        if cnt == len {
            return (n + 1 - len).to_string()
        }
    }

    unreachable!();
}
