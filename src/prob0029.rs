#[crate_type = "rlib"];

extern mod math;

use std::hashmap::HashSet;
use math::prime;
use math::prime::Prime;

pub static EXPECTED_ANSWER: &'static str = "9183";

pub fn solve() -> ~str {
    let mut set = HashSet::new();
    let prime = Prime::new();

    for a in range(2u, 101) {
        let a_factor = prime::factorize(&prime, a).to_owned_vec();
        for b in range(2u, 101) {
            let ab_factor = a_factor
                .map(|&(base, exp)| (base, (exp) as uint * b));
            set.insert(ab_factor);
        }
    }

    return set.len().to_str();
}
