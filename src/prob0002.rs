#[link(name = "prob0002", vers = "0.0", package_id = "prob0002")];
#[crate_type = "lib"];

extern mod math;

use std::iter::AdditiveIterator;
use math::sequence;

pub static EXPECTED_ANSWER: &'static str = "4613732";

pub fn solve() -> ~str {
    let limit = 4000000;
    return sequence::fibonacci::<uint>()
        .take_while(|&f| f < limit)
        .filter(|&f| f % 2 == 0)
        .sum()
        .to_str();
}
