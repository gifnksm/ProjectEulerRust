#[link(name = "prob0046", vers = "0.0")];
#[crate_type = "lib"];

extern mod common;

use core::iterator::{ Counter, Iterator, IteratorUtil };
use common::prime;
use common::arith;
use common::problem::{ Problem };

pub static problem: Problem<'static> = Problem {
    id: 46,
    answer: "5777",
    solver: solve
};

fn is_goldbach(n: uint) -> bool {
    for uint::range(1, arith::isqrt(n / 2) + 1) |s| {
        let sq = s * s * 2;
        if sq > n { return false; }
        if prime::contains(n - sq) { return true; }
    }
    return false;
}

pub fn solve() -> ~str {
    return Counter::new::<uint>(3, 2)
        .filter(|&n| !prime::contains(n))
        .skip_while(|&n| is_goldbach(n))
        .next()
        .get()
        .to_str();
}
