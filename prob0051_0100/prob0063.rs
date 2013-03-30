use std::bigint::{ BigUint };

use common::problem::{ Problem };

pub static problem: Problem<'static> = Problem {
    id: 63,
    answer: "49",
    solver: solve
};

fn solve() -> ~str {
    let mut cnt = 1u; // a == 1
    for uint::range(2, 10) |a_uint| {
        let mut a = BigUint::from_uint(a_uint);
        let mut n = 0;
        let mut an = BigUint::from_uint(1);
        loop {
            n += 1;
            an = an * a;
            let an_str = an.to_str();
            if an_str.len() != n { break; }

            cnt += 1;
        }
    }

    return cnt.to_str();
}

