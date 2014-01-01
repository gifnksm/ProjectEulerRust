#[crate_type = "rlib"];

extern mod math;

use math::prime;
use math::prime::Prime;

pub static EXPECTED_ANSWER: &'static str = "6857";

pub fn solve() -> ~str {
    let num = 600851475143;
    let prime = Prime::new();

    return prime::factorize(&prime, num)
        .map(|(base, _exp)| base)
        .max()
        .unwrap()
        .to_str();
}
