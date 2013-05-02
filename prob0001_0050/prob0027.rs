// p(n) = n^2 + an + b is prime for n = 0 .. N
// p(0) = b         => b must be prime
// p(1) = 1 + a + b => a > -(1+b)
// p(2) = 4 + 2a + b

use core::iterator::{ Counter, IteratorUtil };

use common::extiter::{ ExtIteratorUtil, Range };
use common::prime::{ Prime };
use common::problem::{ Problem };

pub static problem: Problem<'static> = Problem {
    id: 27,
    answer: "-59231",
    solver: solve
};

fn get_len(a: int, b: int, ps: &mut Prime) -> uint {
    return Counter::new(0, 1)
        .take_while(|&n| {
            let val = n * n + a * n + b;
            (val >= 0 && ps.is_prime(val as uint))
        }).last() as uint;
}

fn solve() -> ~str {
    let mut ps = Prime::new();

    let mut max_a = 0;
    let mut max_b = 0;
    let mut max_len = 0;

    for ps.each_borrow() |b, ps| {
        let b = b as int;
        if b >= 1000 { break; }

        let (a, len) = Range::new(-b, 1000)
            .transform(|a| (a, get_len(a, b, ps)))
            .max_as(|&(_a, len)| len);

        if len > max_len {
            max_a = a;
            max_b = b;
            max_len = len;
        }
    }

    return (max_a * max_b).to_str();
}
