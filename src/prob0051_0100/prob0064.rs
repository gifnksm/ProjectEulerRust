#[link(name = "prob0064", vers = "0.0")];
#[crate_type = "lib"];

extern mod common;

use common::calc::{ cont_frac_sqrt };
use common::problem::{ Problem };

pub static problem: Problem<'static> = Problem {
    id: 64,
    answer: "1322",
    solver: solve
};

pub fn solve() -> ~str {
    let mut cnt = 0u;
    for uint::range(1, 10001) |n| {
        let (_a0, an) = cont_frac_sqrt(n);
        let period = an.len();
        if period % 2 == 1 { cnt += 1; }
    }
    return cnt.to_str();
}

