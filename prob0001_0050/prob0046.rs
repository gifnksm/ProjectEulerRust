use common::prime::{ Prime };
use common::arith::{ isqrt };
use common::problem::{ Problem };

pub static problem: Problem<'static> = Problem {
    id: 46,
    answer: "5777",
    solver: solve
};

fn is_goldbach(n: uint, ps: &mut Prime) -> bool {
    for uint::range(1, isqrt(n / 2) + 1) |s| {
        let sq = s * s * 2;
        if sq > n { return false; }
        if ps.is_prime(n - sq) { return true; }
    }
    return false;
}

fn solve() -> ~str {
    let mut ps = Prime();
    let mut n = 1;
    loop {
        n += 2;
        if ps.is_prime(n) { loop; }
        if !is_goldbach(n, &mut ps) {
            break
        }
    }

    return n.to_str();
}
