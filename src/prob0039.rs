#[link(name = "prob0039", vers = "0.0")];
#[crate_type = "lib"];

extern mod common;

use std::uint;
use std::hashmap::HashMap;
use common::arith;

pub static EXPECTED_ANSWER: &'static str = "840";

pub fn solve() -> ~str {
    // a + b + c = 2m(m + n) <= L
    // 1 <= n <= L / 2m - m
    // if n == 1, a + b + c = 2m^2 + 2m <= L
    // m <= (sqrt(1 + L) - 1)/2
    let limit = 1000;
    let mut map = HashMap::new::<uint, uint>();

    for uint::range(1, (arith::isqrt(1 + limit) - 1) / 2) |m| {
        for uint::range(1, uint::min(1 + limit / (2 * m) - m, m)) |n| {
            if (m - n) % 2 == 0 { loop; }
            if m.gcd(&n) != 1 { loop; }
            let (a, b, c) = (m * m - n * n, 2 * m * n, m * m + n * n);
            let s = a + b + c;
            for uint::range(1, limit / s + 1) |k| {
                let new_val = map.find(&(k * s)).map_default(1, |&v| v + 1);
                map.insert(k * s, new_val);
            }
        }
    }

    let (max_key, _max_val) = map.iter().max_by(|&(&_k, &v)| v).unwrap();
    return max_key.to_str();
}
