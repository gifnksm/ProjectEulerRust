#[link(name = "prob0004", vers = "0.0")];
#[crate_type = "lib"];

extern mod common;

use std::uint;
use common::calc;
use common::extiter::Range;

pub static EXPECTED_ANSWER: &'static str = "906609";

struct DividablePairsIterator { num: uint, max: uint, div: uint }

impl DividablePairsIterator {
    fn new(num: uint, min: uint, max: uint) -> DividablePairsIterator {
        let div = uint::max(uint::div_ceil(num, max), min);
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
    let it1 = Range::new_rev(999u, 99).transform(|seed| calc::to_palindromic(seed, 10, false));
    let it2 = Range::new_rev(999u, 99).transform(|seed| calc::to_palindromic(seed, 10, true));

    it1.chain_(it2)
        .flat_map_(|n| DividablePairsIterator::new(n, 100, 999))
        .next()
        .map_consume(|(d1, d2)| d1 * d2)
        .get()
        .to_str()
}

