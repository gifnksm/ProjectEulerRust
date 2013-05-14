use core::iterator::{ IteratorUtil };

use std::bigint::{ BigUint };

use common::extiter::{ ExtIteratorUtil, Range, AdditiveIterator, MultiplicativeIterator };
use common::problem::{ Problem };

pub static problem: Problem<'static> = Problem {
    id: 20,
    answer: "648",
    solver: solve
};

fn solve() -> ~str {
    let s = Range::new::<uint>(1, 101)
        .transform(|n| BigUint::from_uint(n))
        .prod().to_str();
    return s.char_iter()
        .filter_map(|c| char::to_digit(c, 10))
        .sum()
        .to_str();
}
