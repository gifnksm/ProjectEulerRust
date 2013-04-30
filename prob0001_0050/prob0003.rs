use core::iterator::{ IteratorUtil };

use common::prime::{ Prime };
use common::problem::{ Problem };

pub static problem: Problem<'static> = Problem {
    id: 3,
    answer: "6857",
    solver: solve
};

fn solve() -> ~str {
    let num = 600851475143;

    let mut ps = Prime::new();
    let mut it = ps.factorize(num);

    let mut max = 0;
    for it.advance |(base, _exp)| { if base > max { max = base; } }
    return max.to_str();
}
