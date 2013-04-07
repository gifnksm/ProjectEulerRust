use common::arith::{ isqrt };
use common::calc::{ get_gcd };
use common::problem::{ Problem };

pub static problem: Problem<'static> = Problem {
    id: 75,
    answer: "161667",
    solver: solve
};

fn each_prim_pythagorean(m: uint, f: &fn(uint, uint, uint) -> bool) {
    let n0 = if m % 2 == 0 { 1 } else { 2 };
    for uint::range_step(n0, m, 2) |n| {
        if get_gcd(m, n) == 1 {
            let a = m * m - n * n;
            let b = 2 * m * n;
            let c = m * m + n * n;
            if a < b {
                if !f(a, b, c) { return; }
            } else {
                if !f(b, a, c) { return; }
            }
        }
    }
}


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
