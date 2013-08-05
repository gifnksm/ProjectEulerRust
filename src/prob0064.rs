#[link(name = "prob0064", vers = "0.0")];
#[crate_type = "lib"];

extern mod common;

use common::calc;

pub static EXPECTED_ANSWER: &'static str = "1322";

pub fn solve() -> ~str {
    let mut cnt = 0u;
    for n in range(1u, 10001) {
        let (_a0, an) = calc::cont_frac_sqrt(n);
        let period = an.len();
        if period % 2 == 1 { cnt += 1; }
    }
    return cnt.to_str();
}

