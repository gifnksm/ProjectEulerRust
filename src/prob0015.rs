#![crate_id = "prob0015"]
#![crate_type = "rlib"]

extern crate math;

use math::prime::Prime;

pub static EXPECTED_ANSWER: &'static str = "137846528820";

pub fn solve() -> String { Prime::new().comb(40, 20).to_str() }
