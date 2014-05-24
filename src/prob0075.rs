#![crate_id = "prob0075"]
#![crate_type = "rlib"]

extern crate math;

use std::iter;
use math::{arith, sequence};

pub static EXPECTED_ANSWER: &'static str = "161667";

pub fn solve() -> StrBuf {
    let limit = 1500000;
    let mut v = Vec::from_elem(limit + 1, 0);
    for m in range(2, arith::isqrt(limit / 2)) {
        for (a, b, c) in sequence::prim_pythagorean(m) {
            let sum = a + b + c;
            for s in iter::range_step(sum, limit + 1, sum) {
                *v.get_mut(s) += 1;
            }
        }
    }

    return v.iter().count(|&x| x == 1).to_str();
}
