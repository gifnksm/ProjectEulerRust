#![crate_id = "prob0002"]
#![crate_type = "rlib"]

extern crate math;

use std::iter::AdditiveIterator;
use math::sequence;

pub static EXPECTED_ANSWER: &'static str = "4613732";

pub fn solve() -> ~str {
    let limit = 4000000;
    return sequence::fibonacci::<uint>()
        .take_while(|&f| f < limit)
        .filter(|&f| f % 2 == 0)
        .sum()
        .to_str();
}
