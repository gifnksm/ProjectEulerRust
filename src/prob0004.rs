#[crate_id = "prob0004"];
#[crate_type = "rlib"];

extern mod math;

use std::{cmp, uint};
use math::numconv;

pub static EXPECTED_ANSWER: &'static str = "906609";

struct DividablePairsIterator { num: uint, max: uint, div: uint }

impl DividablePairsIterator {
    fn new(num: uint, min: uint, max: uint) -> DividablePairsIterator {
        let div = cmp::max(uint::div_ceil(num, max), min);
        DividablePairsIterator { num: num, div: div, max: max }
    }
}

impl Iterator<(uint, uint)> for DividablePairsIterator {
    fn next(&mut self) -> Option<(uint, uint)> {
        while self.div * self.div <= self.num && self.div <= self. max {
            if self.num % self.div == 0 {
                let tp = (self.div, self.num / self.div);
                self.div += 1;
                return Some(tp);
            }
            self.div += 1;
        }
        return None;
    }
}

pub fn solve() -> ~str {
    let it1 = range(99u, 999).invert().map(|seed| numconv::to_palindromic(seed, 10, false));
    let it2 = range(99u, 999).invert().map(|seed| numconv::to_palindromic(seed, 10, true));

    it1.chain(it2)
        .flat_map(|n| DividablePairsIterator::new(n, 100, 999))
        .next()
        .map(|(d1, d2)| d1 * d2)
        .unwrap()
        .to_str()
}
