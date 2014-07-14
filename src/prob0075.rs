#![crate_name = "prob0075"]
#![crate_type = "rlib"]

extern crate math;

use std::iter;
use math::{arith, sequence};

pub static EXPECTED_ANSWER: &'static str = "161667";

pub fn solve() -> String {
    let limit = 1500000u;
    let mut v = Vec::from_elem(limit + 1, 0u);
    for m in range(2, arith::isqrt(limit / 2)) {
        for (a, b, c) in sequence::prim_pythagorean(m) {
            let sum = a + b + c;
            for s in iter::range_step(sum, limit + 1, sum) {
                *v.get_mut(s) += 1;
            }
        }
    }

    v.iter()
        .filter(|&x| x == &1)
        .count()
        .to_string()
}
