#[link(name = "prob0002", vers = "0.0")];
#[crate_type = "lib"];

extern mod common;

use std::iterator::AdditiveIterator;
use common::extiter::Fibonacci;
use common::problem::Problem;

pub static problem: Problem<'static> = Problem {
    id: 2,
    answer: "4613732",
    solver: solve
};

pub fn solve() -> ~str {
    let limit = 4000000;
    return Fibonacci::new::<uint>()
        .take_while(|&f| f < limit)
        .filter(|&f| f % 2 == 0)
        .sum()
        .to_str();
}
