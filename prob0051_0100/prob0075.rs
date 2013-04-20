use common::arith::{ isqrt };
use common::calc::{ each_prim_pythagorean };
use common::problem::{ Problem };

pub static problem: Problem<'static> = Problem {
    id: 75,
    answer: "161667",
    solver: solve
};

fn solve() -> ~str {
    let limit = 1500000;
    let mut v = vec::from_elem(limit + 1, 0);
    for uint::range(2, isqrt(limit / 2)) |m| {
        for each_prim_pythagorean(m) |a, b, c| {
            let sum = a + b + c;
            for uint::range_step(sum, limit + 1, sum as int) |s| {
                v[s] += 1;
            }
        }
    }

    return v.count(&1).to_str();
}
