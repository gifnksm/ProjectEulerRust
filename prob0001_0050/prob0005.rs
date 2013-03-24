use common::prime::{ Prime, Factors, factors_to_uint };
use common::monoid::{ mergei_as, Max };
use common::problem::{ Problem };

pub static problem: Problem<'static> = Problem {
    id: 5,
    answer: "232792560",
    solver: solve
};

fn solve() -> ~str {
    let mut primes = Prime::new();
    let mut fs = ~[];

    for uint::range(1, 20 + 1) |n| {
        fs.push(iter::to_vec(&Factors::new(n, &mut primes)));
    };

    return factors_to_uint(&mergei_as(fs, Max)).to_str();
}
