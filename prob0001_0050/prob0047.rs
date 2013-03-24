use common::prime::{ Prime, factors };
use common::problem::{ Problem };

pub static problem: Problem<'static> = Problem {
    id: 47,
    answer: "134043",
    solver: solve
};

fn num_factors(n: uint, ps: &mut Prime) -> uint {
    let mut cnt = 0;
    for factors(n, ps) |_f| { cnt += 1; }
    return cnt;
}

fn solve() -> ~str {
    let mut ps = Prime();
    let mut cnt = 0;
    let len = 4;
    let num_factor = 4;
    let mut n = 1;
    loop {
        if num_factors(n, &mut ps) == num_factor {
            cnt += 1;
        } else {
            cnt = 0;
        }
        if cnt == len {
            return (n + 1 - len).to_str();
        }
        n += 1;
    }
}
