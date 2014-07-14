#![crate_name = "prob0012"]
#![crate_type = "rlib"]

extern crate math;

use math::sequence;
use math::prime::Prime;

pub static EXPECTED_ANSWER: &'static str = "76576500";

pub fn solve() -> String {
    let prime = Prime::new();
    return sequence::triangle::<uint>()
        .skip_while(|&t| prime.num_of_divisor(t) <= 500)
        .next().unwrap().to_string();
}
