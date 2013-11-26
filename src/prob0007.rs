#[link(name = "prob0007", vers = "0.0", package_id = "prob0007")];
#[crate_type = "lib"];

extern mod math;

use math::prime;

pub static EXPECTED_ANSWER: &'static str = "104743";

pub fn solve() -> ~str {
    return prime::nth(10000).to_str();
}
