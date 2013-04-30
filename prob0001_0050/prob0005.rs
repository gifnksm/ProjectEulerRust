use core::iterator::{ IteratorUtil };

use common::extvec;
use common::prime::{ Prime, factors_to_uint };
use common::monoid::{ mergei_as, Max };
use common::problem::{ Problem };

pub static problem: Problem<'static> = Problem {
    id: 5,
    answer: "232792560",
    solver: solve
};

fn solve() -> ~str {
    let mut ps = Prime::new();
    let mut fs = ~[];

    for uint::range(1, 20 + 1) |n| {
        fs.push(extvec::from_iter(ps.factorize(n)));
    };

    let mut v = mergei_as(fs, Max);
    return factors_to_uint(v.iter().transform(|&x| x)).to_str();
}
