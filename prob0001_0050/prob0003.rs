use core::iterator::{ IteratorUtil };

use common::extiter;
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
    let it = ps.factorize(num).transform(|(base, _exp)| base);
    return extiter::max(it).to_str();
}
