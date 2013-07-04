#[link(name = "prob0015", vers = "0.0")];
#[crate_type = "lib"];

extern mod common;

use common::prime;

pub static EXPECTED_ANSWER: &'static str = "137846528820";

pub fn solve() -> ~str {
    return prime::comb(40, 20).to_str();
}
