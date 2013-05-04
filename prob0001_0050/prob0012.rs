use core::iterator::{ IteratorUtil };

use common::prime;
use common::extiter::{ Triangle, ExtIteratorUtil };
use common::problem::{ Problem };

pub static problem: Problem<'static> = Problem {
    id: 12,
    answer: "76576500",
    solver: solve
};

fn solve() -> ~str {
    return Triangle::new()
        .skip_while(|&t| prime::num_of_divisors(t) <= 500)
        .nth(0).to_str();
}
