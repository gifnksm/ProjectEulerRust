use core::iterator::{ IteratorUtil };

use common::extiter::{ AdditiveIterator };
use common::prime;
use common::problem::{ Problem };

pub static problem: Problem<'static> = Problem {
    id: 21,
    answer: "31626",
    solver: solve
};

fn solve() -> ~str {
    let limit = 10000;

    let sum_of_divs = vec::from_fn(limit, |n| prime::sum_of_proper_divisors(n));
    let is_deficient = |&(n, div): &(uint, uint)| div < n;
    let is_amicable  = |&(n, div): &(uint, uint)| sum_of_divs[div] == n;
    return sum_of_divs.iter()
        .transform(|&n| n)
        .enumerate()
        .filter(is_deficient)
        .filter(is_amicable)
        .transform(|(a, b)| a + b)
        .sum()
        .to_str();
}
