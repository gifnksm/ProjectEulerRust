use common::prime::{ Prime, comb };
use common::problem::{ Problem };

pub static problem: Problem<'static> = Problem {
    id: 15,
    answer: "137846528820",
    solver: solve
};

fn solve() -> ~str {
    let mut primes = Prime::new();
    return comb(40, 20, &mut primes).to_str();
}
