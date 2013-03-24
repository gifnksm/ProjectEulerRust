use common::prime::{ Prime };
use common::calc::{ num_to_digits };
use common::problem::{ Problem };

pub static problem: Problem<'static> = Problem {
    id: 35,
    answer: "55",
    solver: solve
};

fn is_circular_prime(n: uint, ps: &mut Prime) -> bool {
    let buf = num_to_digits(n, 10);

    for uint::range(1, buf.len()) |i| {
        let mut num = 0;
        for uint::range(0, buf.len()) |j| {
            num = num * 10 + (buf[(i + j) % buf.len()] as uint);
        }
        if !ps.is_prime(num) { return false; }
    }
    return true;
}

fn solve() -> ~str {
    let mut ps = Prime::new();
    let mut cnt = 0u;

    for ps.each_borrow |p, ps| {
        if p >= 1000000 { break; }
        if is_circular_prime(p, ps) {
            cnt += 1;
        }
    }

    return cnt.to_str();
}
