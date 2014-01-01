#[crate_type = "rlib"];

extern mod math;

use math::{numconv, oldprime};

pub static EXPECTED_ANSWER: &'static str = "55";

#[inline(always)]
fn is_circular_prime(n: uint) -> bool {
    let ds = numconv::to_digits(n, 10).to_owned_vec();
    let mut buf = ds.clone();

    for i in range(1, ds.len()) {
        for j in range(0, buf.len()) {
            buf[j] = ds[(i + j) % ds.len()];
        }
        if !oldprime::contains(numconv::from_digits(buf, 10)) { return false; }
    }

    true
}

pub fn solve() -> ~str {
    return oldprime::iter()
        .take_while(|&p| p < 1000000)
        .count(is_circular_prime)
        .to_str();
}
