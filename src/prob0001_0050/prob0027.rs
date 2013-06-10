#[link(name = "prob0027", vers = "0.0")];
#[crate_type = "lib"];

extern mod common;

use std::iterator::{Counter, IteratorUtil};
use common::extiter::{ExtIteratorUtil, Range};
use common::prime;
use common::problem::{Problem};

pub static problem: Problem<'static> = Problem {
    id: 27,
    answer: "-59231",
    solver: solve
};

// p(n) = n^2 + an + b is prime for n = 0 .. N
// p(0) = b         => b must be prime
// p(1) = 1 + a + b => a > -(1+b)
// p(2) = 4 + 2a + b


fn get_len(a: int, b: int) -> uint {
    return Counter::new(0, 1)
        .take_while(|&n| {
            let val = n * n + a * n + b;
            (val >= 0 && prime::contains(val as uint))
        }).last_().get() as uint;
}

pub fn solve() -> ~str {
    let mut max_a = 0;
    let mut max_b = 0;
    let mut max_len = 0;

    for prime::each() |b| {
        let b = b as int;
        if b >= 1000 { break; }

        let (a, len) = Range::new(-b, 1000)
            .transform(|a| (a, get_len(a, b)))
            .max_as(|&(_a, len)| len);

        if len > max_len {
            max_a = a;
            max_b = b;
            max_len = len;
        }
    }

    return (max_a * max_b).to_str();
}
