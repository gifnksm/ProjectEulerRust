#[crate_id = "prob0069"];
#[crate_type = "rlib"];

extern crate math;

use math::prime::Prime;

pub static EXPECTED_ANSWER: &'static str = "510510";

pub fn solve() -> ~str {
    let limit = 1000000;
    let prime = Prime::new();

    let mut n = 1;
    for p in prime.iter() {
        if n * p > limit { break; }
        n *= p;
    }

    return n.to_str();
}
