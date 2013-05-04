use core::util;
use core::iterator::{ Counter, IteratorUtil };

use common::prime;
use common::extiter::{ ExtIteratorUtil };
use common::problem::{ Problem };

pub static problem: Problem<'static> = Problem {
    id: 47,
    answer: "134043",
    solver: solve
};

fn solve() -> ~str {
    let len = 4;
    let num_factor = 4;

    let mut cnt = 0;
    let mut it = Counter::new::<uint>(1, 1);
    for it.advance |n| {
        if prime::factorize(n).count_elem() != num_factor {
            cnt = 0;
            loop;
        }

        cnt += 1;
        if cnt == len {
            return (n + 1 - len).to_str();
        }
    }

    util::unreachable();
}
