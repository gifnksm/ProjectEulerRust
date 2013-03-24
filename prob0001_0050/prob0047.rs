use common::prime::{ Prime, Factors };
use common::problem::{ Problem };

pub static problem: Problem<'static> = Problem {
    id: 47,
    answer: "134043",
    solver: solve
};

fn num_factors(n: uint, ps: &mut Prime) -> uint {
    iter::foldl(&Factors::new(n, ps), 0, |&cnt, _elm| cnt + 1)
}

fn solve() -> ~str {
    let mut ps = Prime::new();
    let mut cnt = 0;
    let len = 4;
    let num_factor = 4;
    let mut n = 1;
    loop {
        if num_factors(n, &mut ps) == num_factor {
            cnt += 1;
            if cnt == len {
                return (n + 1 - len).to_str();
            }
        } else {
            cnt = 0;
        }
        n += 1;
    }
}
