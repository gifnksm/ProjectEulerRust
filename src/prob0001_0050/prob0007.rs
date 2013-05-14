use common::prime;
use common::problem::{ Problem };

pub static problem: Problem<'static> = Problem {
    id: 7,
    answer: "104743",
    solver: solve
};

fn solve() -> ~str {
    return prime::nth(10000).to_str();
}
