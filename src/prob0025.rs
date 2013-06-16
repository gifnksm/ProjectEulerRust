#[link(name = "prob0025", vers = "0.0")];
#[crate_type = "lib"];

extern mod extra;
extern mod common;

use extra::bigint::BigUint;
use common::extiter::Fibonacci;
use common::problem::Problem;

pub static problem: Problem<'static> = Problem {
    id: 25,
    answer: "4782",
    solver: solve
};

pub fn solve() -> ~str {
    let limit = FromStr::from_str("9".repeat(999)).get();
    let mut it = Fibonacci::new::<BigUint>().take_while(|&n| n <= limit);
    return (it.count() + 1).to_str();
}
