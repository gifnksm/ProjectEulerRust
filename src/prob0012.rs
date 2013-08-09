#[link(name = "prob0012", vers = "0.0")];
#[crate_type = "lib"];

extern mod common;

use common::prime;
use common::extiter::Triangle;

pub static EXPECTED_ANSWER: &'static str = "76576500";

pub fn solve() -> ~str {
    return Triangle::new()
        .skip_while(|&t| prime::num_of_divisors(t) <= 500)
        .next().unwrap().to_str();
}
