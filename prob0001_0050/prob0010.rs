use core::iterator::{ IteratorUtil };

use common::prime::{ Prime };
use common::extiter::{ AdditiveIterator };
use common::problem::{ Problem };

pub static problem: Problem<'static> = Problem {
    id: 10,
    answer: "142913828922",
    solver: solve
};

fn solve() -> ~str {
    let mut ps = Prime::new();
    return ps.iter()
        .take_while(|&p| p < 2000000)
        .sum()
        .to_str();
}
