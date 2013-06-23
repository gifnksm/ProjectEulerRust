#[link(name = "prob0090", vers = "0.0")];
#[crate_type = "lib"];

extern mod common;

use std::vec;
use common::calc;
use common::problem::Problem;

pub static problem: Problem<'static> = Problem {
    id: 90,
    answer: "1217",
    solver: solve
};

pub fn solve() -> ~str {
    let mut all_combs = ~[];
    for calc::combinate(vec::from_fn(10, |i| i), 6) |mut cs, _| {
        match (cs.iter().any_(|&x| x == 6), cs.iter().any_(|&x| x == 9)) {
            (false, true)  => cs.push(6),
            (true,  false) => cs.push(9),
            _ => {}
        }
        all_combs.push(cs);
    }

    let nums = do vec::from_fn(9) |i| {
        let n = (i + 1) * (i + 1);
        (n / 10, n % 10)
    };

    let mut cnt = 0u;
    for all_combs.iter().enumerate().advance |(i, cs1)| {
        for all_combs.tailn(i + 1).iter().advance |cs2| {
            let cond = |&(a, b): &(uint, uint)| {
                (cs1.iter().any_(|&x| x == a) && cs2.iter().any_(|&x| x == b)) ||
                    (cs1.iter().any_(|&x| x == b) && cs2.iter().any_(|&x| x == a))
            };
            if nums.iter().all(cond) { cnt += 1; }
        }
    }
    return cnt.to_str();
}
