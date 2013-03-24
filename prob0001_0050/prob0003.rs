use core::util::{ unreachable };

use common::prime::{ Prime };
use common::problem::{ Problem };

pub static problem: Problem<'static> = Problem {
    id: 3,
    answer: "6857",
    solver: solve
};

fn solve() -> ~str {
    let mut num = 600851475143;
    let mut ps = Prime();
    for ps.each |p| {
        while num % p == 0 {
            num /= p;
        }
        if num == 1 {
            return p.to_str();
        }
    }

    unreachable();
}
