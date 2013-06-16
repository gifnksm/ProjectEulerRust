#[link(name = "prob0012", vers = "0.0")];
#[crate_type = "lib"];

extern mod common;

use common::prime;
use common::extiter::Triangle;
use common::problem::Problem;

pub static problem: Problem<'static> = Problem {
    id: 12,
    answer: "76576500",
    solver: solve
};

pub fn solve() -> ~str {
    return Triangle::new()
        .skip_while(|&t| prime::num_of_divisors(t) <= 500)
        .next().get().to_str();
}
