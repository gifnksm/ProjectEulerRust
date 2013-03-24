use common::prime::{ Prime, Factors };
use common::problem::{ Problem };

pub static problem: Problem<'static> = Problem {
    id: 3,
    answer: "6857",
    solver: solve
};

fn solve() -> ~str {
    let mut num = 600851475143;

    let mut ps = Prime::new();
    return iter::max(&Factors::new(num, &mut ps)).first().to_str();
}
