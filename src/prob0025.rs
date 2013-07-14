#[link(name = "prob0025", vers = "0.0")];
#[crate_type = "lib"];

extern mod extra;
extern mod common;

use extra::bigint::BigUint;
use common::extiter::Fibonacci;

pub static EXPECTED_ANSWER: &'static str = "4782";

pub fn solve() -> ~str {
    let limit = FromStr::from_str("9".repeat(999)).get();
    let mut it = Fibonacci::new::<BigUint>().take_while(|n| *n <= limit);
    return (it.len_() + 1).to_str();
}
