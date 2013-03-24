use common::prime::{ Prime };
use common::problem::{ Problem };

pub static problem: Problem<'static> = Problem {
    id: 7,
    answer: "104743",
    solver: solve
};

fn solve() -> ~str {
    let mut ps = Prime::new();
    return ps.get_at(10000).to_str();
}
