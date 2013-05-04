use core::iterator::{ IteratorUtil };

use common::extiter::{ OrderedIterator };
use common::prime;
use common::problem::{ Problem };

pub static problem: Problem<'static> = Problem {
    id: 3,
    answer: "6857",
    solver: solve
};

fn solve() -> ~str {
    let num = 600851475143;

    return prime::factorize(num)
        .transform(|(base, _exp)| base)
        .max()
        .to_str();
}
