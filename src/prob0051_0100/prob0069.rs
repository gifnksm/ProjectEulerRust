#[link(name = "prob0069", vers = "0.0")];
#[crate_type = "lib"];

extern mod common;

use common::prime;
use common::problem::{ Problem };

pub static problem: Problem<'static> = Problem {
    id: 69,
    answer: "510510",
    solver: solve
};

pub fn solve() -> ~str {
    let limit = 1000000;

    let mut n   = 1;
    let mut val = 1f;
    for prime::each |p| {
        if n * p > limit { break; }
        n   *= p;
        val *= (p as float) * (p as float - 1f);
    }

    return n.to_str();
}
