#[link(name = "prob0029", vers = "0.0")];
#[crate_type = "lib"];

extern mod common;

use std::hashmap::{ HashSet };
use std::iterator::{ IteratorUtil };
use common::prime;
use common::problem::{ Problem };

pub static problem: Problem<'static> = Problem {
    id: 29,
    answer: "9183",
    solver: solve
};

pub fn solve() -> ~str {
    let mut set = HashSet::new();

    for uint::range(2, 101) |a| {
        let a_factor = prime::factorize(a).to_vec();
        for uint::range(2, 101) |b| {
            let ab_factor = a_factor
                .map(|&(base, exp)| (base, (exp) as uint * b));
            set.insert(ab_factor);
        }
    }

    return set.len().to_str();
}
