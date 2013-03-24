use core::util::{ unreachable };

use common::calc::{ permutate_num };
use common::prime::{ Prime };
use common::problem::{ Problem };

pub static problem: Problem<'static> = Problem {
    id: 41,
    answer: "7652413",
    solver: solve
};

fn solve() -> ~str {
    let mut ps = Prime::new();
    for permutate_num(&[7, 6, 5, 4, 3, 2, 1], 7, 0, 9999999) |num, _rest| {
        if ps.is_prime(num) {
            return num.to_str();
        }
    }

    unreachable();
}
