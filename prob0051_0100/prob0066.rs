use std::bigint::{ BigUint };

use common::calc::{ cont_frac_sqrt, fold_cont_frac };
use common::problem::{ Problem };

pub static problem: Problem<'static> = Problem {
    id: 66,
    answer: "661",
    solver: solve
};

fn each_d(f: &fn(uint) -> bool) {
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
        if !f(d) { break; }
    }
}

fn solve_diophantine(d: uint) -> (BigUint, BigUint) {
    let (a0, an) = cont_frac_sqrt(d);
    if an.len() % 2 == 0 {
        return fold_cont_frac::<BigUint>(~[a0] + an.init());
    } else {
        return fold_cont_frac::<BigUint>(~[a0] + an + an.init());
    }
}

fn solve() -> ~str {
    let mut max_x   = BigUint::from_uint(0);
    let mut max_x_d = 0;
    for each_d |d| {
        if d > 1000 { break; }
        let (x, _y) = solve_diophantine(d);
        if x > max_x { max_x = x; max_x_d = d; }
    }
    return max_x_d.to_str();
}

