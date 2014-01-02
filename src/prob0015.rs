#[crate_type = "rlib"];

extern mod math;

use math::prime::Prime;

pub static EXPECTED_ANSWER: &'static str = "137846528820";

pub fn solve() -> ~str { Prime::new().comb(40, 20).to_str() }
