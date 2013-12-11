#[crate_type = "lib"];

extern mod math;

use math::cont_frac;

pub static EXPECTED_ANSWER: &'static str = "1322";

pub fn solve() -> ~str {
    let mut cnt = 0u;
    for n in range(1u, 10001) {
        let (_a0, an) = cont_frac::sqrt(n);
        let period = an.len();
        if period % 2 == 1 { cnt += 1; }
    }
    return cnt.to_str();
}

