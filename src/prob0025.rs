#[crate_id = "prob0025"];
#[crate_type = "rlib"];

extern crate num;
extern crate math;

use num::bigint::BigUint;
use math::sequence;

pub static EXPECTED_ANSWER: &'static str = "4782";

pub fn solve() -> ~str {
    let limit = FromStr::from_str("9".repeat(999)).unwrap();
    let mut it = sequence::fibonacci::<BigUint>().take_while(|n| *n <= limit);
    return (it.len() + 1).to_str();
}
