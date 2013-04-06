use common::prime::{ Prime };
use common::problem::{ Problem };

pub static problem: Problem<'static> = Problem {
    id: 72,
    answer: "303963552391",
    solver: solve
};

fn solve() -> ~str {
    let limit = 1000000;

    let mut v = vec::from_fn(limit + 1, |n| n);
    v[1] = 0;

    let mut ps = Prime::new();
    for ps.each |p| {
        if p > limit { break; }
        for uint::range_step(p, limit + 1, p as int) |n| {
            v[n] = v[n] * (p - 1) / p;
        }
    }

    let mut cnt = 0u;
    for v.each_val |phi| { cnt += phi; }
    return cnt.to_str();
}
