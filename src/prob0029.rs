#![crate_id = "prob0029"]
#![crate_id = "prob0029"]
#![crate_type = "rlib"]
#![crate_type = "rlib"]

extern crate collections;
extern crate math;

use collections::HashSet;
use math::prime::Prime;

pub static EXPECTED_ANSWER: &'static str = "9183";

pub fn solve() -> ~str {
    let mut set = HashSet::new();
    let prime = Prime::new();

    for a in range(2u, 101) {
        let a_factor = prime.factorize(a).collect::<~[(uint, int)]>();
        for b in range(2u, 101) {
            let ab_factor = a_factor
                .iter()
                .map(|&(base, exp)| (base, (exp) as uint * b))
                .collect::<~[(uint, uint)]>();
            set.insert(ab_factor);
        }
    }

    set.len().to_str()
}
