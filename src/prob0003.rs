#![crate_id = "prob0003"]
#![crate_type = "rlib"]

extern crate math;

use math::prime::Prime;

pub static EXPECTED_ANSWER: &'static str = "6857";

pub fn solve() -> String {
    let num = 600851475143;

    return Prime::new()
        .factorize(num)
        .map(|(base, _exp)| base)
        .max()
        .unwrap()
        .to_str();
}
