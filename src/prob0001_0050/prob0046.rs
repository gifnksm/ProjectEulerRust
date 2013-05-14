use core::iterator::{ Counter, IteratorUtil };

use common::extiter::{ ExtIteratorUtil };
use common::prime;
use common::arith::{ isqrt };
use common::problem::{ Problem };

pub static problem: Problem<'static> = Problem {
    id: 46,
    answer: "5777",
    solver: solve
};

fn is_goldbach(n: uint) -> bool {
    for uint::range(1, isqrt(n / 2) + 1) |s| {
        let sq = s * s * 2;
        if sq > n { return false; }
        if prime::contains(n - sq) { return true; }
    }
    return false;
}

fn solve() -> ~str {
    return Counter::new::<uint>(3, 2)
        .filter(|&n| !prime::contains(n))
        .skip_while(|&n| is_goldbach(n))
        .first()
        .to_str();
}
