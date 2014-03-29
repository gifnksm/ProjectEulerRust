#![crate_id = "prob0047"]
#![crate_id = "prob0047"]
#![crate_type = "rlib"]
#![crate_type = "rlib"]

extern crate math;

use std::iter;
use math::prime::Prime;

pub static EXPECTED_ANSWER: &'static str = "134043";

pub fn solve() -> ~str {
    let len = 4;
    let num_factor = 4;
    let prime = Prime::new();

    let mut cnt = 0;
    for n in iter::count(1u, 1) {
        if prime.factorize(n).len() != num_factor {
            cnt = 0;
            continue
        }

        cnt += 1;
        if cnt == len {
            return (n + 1 - len).to_str()
        }
    }

    unreachable!();
}
