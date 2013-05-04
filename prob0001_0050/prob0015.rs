use common::prime;
use common::problem::{ Problem };

pub static problem: Problem<'static> = Problem {
    id: 15,
    answer: "137846528820",
    solver: solve
};

fn solve() -> ~str {
    return prime::comb(40, 20).to_str();
}
