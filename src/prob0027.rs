#[crate_type = "rlib"];

extern mod math;

use std::iter;
use math::prime;

pub static EXPECTED_ANSWER: &'static str = "-59231";

// p(n) = n^2 + an + b is prime for n = 0 .. N
// p(0) = b         => b must be prime
// p(1) = 1 + a + b => a > -(1+b)
// p(2) = 4 + 2a + b
fn get_len(a: int, b: int) -> uint {
    return iter::count(0, 1)
        .take_while(|&n| {
            let val = n * n + a * n + b;
            (val >= 0 && prime::contains(val as uint))
        }).last().unwrap() as uint;
}

pub fn solve() -> ~str {
    let (a, b, _len) = prime::iter()
        .take_while(|&p| p < 1000)
        .filter_map(|p| {
            let b = p as int;
            range(-(b as int), 1000)
                .map(|a| (a, b, get_len(a, b)))
                .max_by(|&(_a, _b, len)| len)
        }).max_by(|&(_a, _b, len)| len)
        .unwrap();
    return (a * b).to_str();
}
