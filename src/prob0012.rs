#[crate_type = "rlib"];

extern mod math;

use math::{prime, sequence};

pub static EXPECTED_ANSWER: &'static str = "76576500";

pub fn solve() -> ~str {
    return sequence::triangle::<uint>()
        .skip_while(|&t| prime::num_of_divisors(t) <= 500)
        .next().unwrap().to_str();
}
