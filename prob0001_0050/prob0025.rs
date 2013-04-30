use core::iterator::{ IteratorUtil };
use core::num::{ FromStrRadix };
use std::bigint::{ BigUint };

use common::extiter::{ Fibonacci, count_elem };
use common::problem::{ Problem };

pub static problem: Problem<'static> = Problem {
    id: 25,
    answer: "4782",
    solver: solve
};

fn solve() -> ~str {
    let limit = FromStrRadix::from_str_radix(
        str::from_bytes(vec::from_elem(999, '9' as u8)), 10).get();

    let it = Fibonacci::new::<BigUint>().take_while(|&n| n <= limit);
    return (count_elem(it) + 1).to_str();
}