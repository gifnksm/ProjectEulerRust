#[link(name = "prob0007", vers = "0.0")];
#[crate_type = "lib"];

extern mod common;

use common::prime;
use common::problem::{Problem};

pub static problem: Problem<'static> = Problem {
    id: 7,
    answer: "104743",
    solver: solve
};

pub fn solve() -> ~str {
    return prime::nth(10000).to_str();
}
