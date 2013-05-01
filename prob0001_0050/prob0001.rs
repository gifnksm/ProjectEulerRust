use core::iterator::{ IteratorUtil };

use common::extiter;
use common::extiter::{ uint_range };
use common::problem::{ Problem };

pub static problem: Problem<'static> = Problem {
    id: 1,
    answer: "233168",
    solver: solve
};

fn solve() -> ~str {
    let it = uint_range(0, 1000).filter(|&n| n % 3 == 0 || n % 5 == 0);
    return extiter::sum(it).to_str();
}
