#[link(name = "prob0035", vers = "0.0")];
#[crate_type = "lib"];

extern mod common;

use common::{prime, calc};

pub static EXPECTED_ANSWER: &'static str = "55";

#[inline(always)]
fn is_circular_prime(n: uint) -> bool {
    let buf = calc::num_to_digits(n, 10);

    for i in range(1, buf.len()) {
        let mut num = 0;
        for j in range(0, buf.len()) {
            num = num * 10 + (buf[(i + j) % buf.len()] as uint);
        }
        if !prime::contains(num) { return false; }
    }

    return true;
}

pub fn solve() -> ~str {
    return prime::iter()
        .take_while(|&p| p < 1000000)
        .count(is_circular_prime)
        .to_str();
}
