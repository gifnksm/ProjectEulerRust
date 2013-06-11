#[link(name = "prob0066", vers = "0.0")];
#[crate_type = "lib"];

extern mod extra;
extern mod common;
use extra::bigint::{BigUint};
use common::calc::{solve_pel};
use common::problem::{Problem};

pub static problem: Problem<'static> = Problem {
    id: 66,
    answer: "661",
    solver: solve
};

fn each_d(f: &fn(uint) -> bool) -> bool {
    let mut d      = 0;
    let mut sqrt   = 1;
    let mut square = 1;
    loop {
        d += 1;
        // skip square
        if d == square {
            sqrt += 1;
            square = sqrt * sqrt;
            loop;
        }
        if !f(d) { return false; }
    }
}

pub fn solve() -> ~str {
    let mut max_x   = BigUint::from_uint(0);
    let mut max_x_d = 0;
    for each_d |d| {
        if d > 1000 { break; }
        let (x, _y) = solve_pel::<BigUint>(d);
        if x > max_x { max_x = x; max_x_d = d; }
    }
    return max_x_d.to_str();
}

