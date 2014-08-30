#![crate_name = "prob0027"]
#![crate_type = "rlib"]

extern crate math;

use std::iter;
use math::prime::Prime;

pub static EXPECTED_ANSWER: &'static str = "-59231";

// p(n) = n^2 + an + b is prime for n = 0 .. N
// p(0) = b         => b must be prime
// p(1) = 1 + a + b => a > -(1+b)
// p(2) = 4 + 2a + b
fn get_len(prime: &Prime, a: int, b: int) -> uint {
    return iter::count(0i, 1)
        .take_while(|&n| {
            let val = n * n + a * n + b;
            (val >= 0 && prime.contains(val as uint))
        }).last().unwrap() as uint;
}

pub fn solve() -> String {
    let prime = Prime::new();
    let (a, b, _len) = prime.iter()
        .take_while(|&p| p < 1000)
        .filter_map(|p| {
            let b = p as int;
            range(-b, 1000)
                .map(|a| (a, b, get_len(&prime, a, b)))
                .max_by(|&(_a, _b, len)| len)
        }).max_by(|&(_a, _b, len)| len)
        .unwrap();
    return (a * b).to_string();
}
