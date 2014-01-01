#[crate_type = "rlib"];

extern mod math;

use prime = math::oldprime;

pub static EXPECTED_ANSWER: &'static str = "137846528820";

pub fn solve() -> ~str {
    return prime::comb(40, 20).to_str();
}
