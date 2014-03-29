#![crate_id = "prob0078"]
#![crate_id = "prob0078"]
#![crate_type = "rlib"]
#![crate_type = "rlib"]

use std::iter;

pub static EXPECTED_ANSWER: &'static str = "55374";

static MILLION: int = 1000000;

#[inline]
fn penta(n: int) -> int { n * (3 * n - 1) / 2 }

pub fn solve() -> ~str {
    let mut v = [0, ..65536];
    v[0] = 1;

    for n in iter::count(1, 1) {
        let mut way = 0;
        for i in iter::count(0, 1) {
            let k = i % 4;
            let p = if k == 0 || k == 2 { penta(i / 2 + 1) } else { penta(-i / 2 - 1) };
            if p > n { break; }

            way = match k {
                0 => way + v[n - p],
                1 => way + v[n - p],
                2 => way - v[n - p],
                _ => way - v[n - p]
            } % MILLION
        }
        v[n] = way;

        if way == 0 { return n.to_str() }
    }

    unreachable!()
}
