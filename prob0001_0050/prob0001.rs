use core::iterator::{ IteratorUtil };

use common::extiter::{ uint_range, sum_uint };
use common::problem::{ Problem };

pub static problem: Problem<'static> = Problem {
    id: 1,
    answer: "233168",
    solver: solve
};

fn solve() -> ~str {
    let it = uint_range(0, 1000).filter(|&n| n % 3 == 0 || n % 5 == 0);
    return sum_uint(it).to_str();
}
