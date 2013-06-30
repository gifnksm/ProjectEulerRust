#[link(name = "prob0007", vers = "0.0")];
#[crate_type = "lib"];

extern mod common;

use common::prime;

pub static expected_answer: &'static str = "104743";

pub fn solve() -> ~str {
    return prime::nth(10000).to_str();
}
