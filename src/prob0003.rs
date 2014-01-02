#[crate_type = "rlib"];

extern mod math;

use math::prime::Prime;

pub static EXPECTED_ANSWER: &'static str = "6857";

pub fn solve() -> ~str {
    let num = 600851475143;

    return Prime::new()
        .factorize(num)
        .map(|(base, _exp)| base)
        .max()
        .unwrap()
        .to_str();
}
