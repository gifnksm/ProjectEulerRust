#[link(name = "prob0015", vers = "0.0")];
#[crate_type = "lib"];

extern mod common;

use common::prime;
use common::problem::{ Problem };

pub static problem: Problem<'static> = Problem {
    id: 15,
    answer: "137846528820",
    solver: solve
};

pub fn solve() -> ~str {
    return prime::comb(40, 20).to_str();
}
