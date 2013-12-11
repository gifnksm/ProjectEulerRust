#[crate_type = "lib"];

extern mod math;

use math::prime;

pub static EXPECTED_ANSWER: &'static str = "6857";

pub fn solve() -> ~str {
    let num = 600851475143;

    return prime::factorize(num)
        .map(|(base, _exp)| base)
        .max()
        .unwrap()
        .to_str();
}
