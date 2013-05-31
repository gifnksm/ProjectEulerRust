#[link(name = "prob0071", vers = "0.0")];
#[crate_type = "lib"];

extern mod common;

use std::uint;
use common::problem::{Problem};

pub static problem: Problem<'static> = Problem {
    id: 71,
    answer: "428570",
    solver: solve
};

pub fn solve() -> ~str {
    let limit = 1000000;
    let mut max_n = 0;
    let mut max_d = 1;
    for uint::range_rev(limit, limit - 7) |d| {
        let n = if 3 * d % 7 == 0 { 3 * d / 7 - 1 } else { 3 * d / 7 };
        if n *max_d > max_n * d {
            max_n = n;
            max_d = d;
        }
    }
    return max_n.to_str();
}
