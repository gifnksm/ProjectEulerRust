#[link(name = "prob0015", vers = "0.0", package_id = "prob0015")];
#[crate_type = "lib"];

extern mod math;

use math::prime;

pub static EXPECTED_ANSWER: &'static str = "137846528820";

pub fn solve() -> ~str {
    return prime::comb(40, 20).to_str();
}
