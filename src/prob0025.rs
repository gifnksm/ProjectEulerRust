#![crate_id = "prob0025"]
#![crate_type = "rlib"]

extern crate num;
extern crate math;

use num::bigint::BigUint;
use math::sequence;

pub static EXPECTED_ANSWER: &'static str = "4782";

pub fn solve() -> String {
    let limit = from_str("9".repeat(999).as_slice()).unwrap();
    let mut it = sequence::fibonacci::<BigUint>().take_while(|n| *n <= limit);
    (it.len() + 1).to_str()
}
