use std::bigint::{ BigUint };

use common::calc::{ each_fib };
use common::problem::{ Problem };

pub static problem: Problem<'static> = Problem {
    id: 25,
    answer: "4782",
    solver: solve
};

fn solve() -> ~str {
    let mut i = 0u;
    let limit = BigUint::from_str_radix(
        str::from_bytes(vec::from_elem(999, '9' as u8)), 10).get();
    for each_fib |f: &BigUint| {
        i += 1;
        if *f > limit { break; }
    }

    return i.to_str();
}