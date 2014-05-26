#![crate_id = "prob0005"]
#![crate_type = "rlib"]

extern crate collections;
extern crate data;
extern crate math;

use std::cmp;
use collections::HashMap;
use math::prime::{Prime, FactorIterator};

pub static EXPECTED_ANSWER: &'static str = "232792560";

pub fn solve() -> String {
    let mut map = HashMap::new();
    let prime = Prime::new();
    for i in range(1u, 20) {
        for (b, e) in prime.factorize(i) {
            map.insert_or_update_with(b, e, |_, v| {
                *v = cmp::max(*v, e);
            });
        }
    }
    map.move_iter().to_uint().to_str()
}
