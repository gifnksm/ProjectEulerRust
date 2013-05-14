#[link(name = "prob0068", vers = "0.0")];
#[crate_type = "lib"];

extern mod common;

use common::problem::{ Problem };

pub static problem: Problem<'static> = Problem {
    id: 68,
    answer: "6531031914842725",
    solver: solve
};

pub fn solve() -> ~str {
    // solve by hand...
    //
    // outside: 6, 10, 9, 8, 7
    // (1 + 2 + 3 + 4 + 5) * 2 +
    // 6 + 7 + 8 + 9 + 10
    // = 15 + 55 = 70
    // 70 / 5 = 14
    //
    // 6, 5, X is max => 6, 5, 3
    //
    // 6, 5, 3
    // 10, 3, 1
    // 9, 1, 4
    // 8, 4, 2
    // 7, 2, 5

    ~"6531031914842725"
}

