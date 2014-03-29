#![crate_id = "prob0119"]
#![crate_id = "prob0119"]
#![crate_type = "rlib"]
#![crate_type = "rlib"]

extern crate collections;
extern crate math;

use std::num;
use std::iter::{AdditiveIterator, Filter, SkipWhile};
use collections::priority_queue::PriorityQueue;
use math::numconv;

pub static EXPECTED_ANSWER: &'static str = "248155780267521";

struct Power(uint, uint, uint);

impl Eq for Power {
    #[inline]
    fn eq(&self, other: &Power) -> bool {
        let Power(sn, sb, _) = *self;
        let Power(on, ob, _) = *other;
        sn == on && sb == ob
    }
}

impl Ord for Power {
    #[inline]
    fn lt(&self, other: &Power) -> bool {
        let Power(sn, sb, _) = *self;
        let Power(on, ob, _) = *other;
        sn > on || (sn == on && sb > ob)
    }
}

struct Powers {
    queue: PriorityQueue<Power>
}

impl Powers {
    #[inline]
    fn new() -> Powers {
        let mut queue = PriorityQueue::new();
        queue.push(Power(4, 2, 2));
        Powers { queue: queue }
    }
}

impl Iterator<(uint, uint, uint)> for Powers {
    #[inline]
    fn next(&mut self) -> Option<(uint, uint, uint)> {
        let Power(n, b, e) = self.queue.pop();
        if b == 2 { self.queue.push(Power(n * b, b, e + 1)); }
        self.queue.push(Power(num::pow(b + 1, e), b + 1, e));
        Some((n, b, e))
    }
}

#[inline]
fn a<'a>() -> Filter<'a, (uint, uint, uint), SkipWhile<'a, (uint, uint, uint), Powers>> {
    Powers::new()
        .skip_while(|&(n, _b, _e)| n < 10)
        .filter(|&(n, b, _e)| numconv::to_digits(n, 10).sum() == b)
}

pub fn solve() -> ~str {
    let (n, _b, _e) = a().nth(29).unwrap();
    n.to_str()
}

#[cfg(test)]
mod tests {
    use super::Powers;

    #[test]
    fn powers() {
        let mut it = Powers::new();
        assert_eq!(Some((4, 2, 2)), it.next());
        assert_eq!(Some((8, 2, 3)), it.next());
        assert_eq!(Some((9, 3, 2)), it.next());
        assert_eq!(Some((16, 2, 4)), it.next());
        assert_eq!(Some((16, 4, 2)), it.next());
        assert_eq!(Some((25, 5, 2)), it.next());
        assert_eq!(Some((27, 3, 3)), it.next());
        assert_eq!(Some((32, 2, 5)), it.next());
        assert_eq!(Some((36, 6, 2)), it.next());
        assert_eq!(Some((49, 7, 2)), it.next());
        assert_eq!(Some((64, 2, 6)), it.next());
        assert_eq!(Some((64, 4, 3)), it.next());
        assert_eq!(Some((64, 8, 2)), it.next());
        assert_eq!(Some((81, 3, 4)), it.next());
        assert_eq!(Some((81, 9, 2)), it.next());
        assert_eq!(Some((100, 10, 2)), it.next());
    }

    #[test]
    fn a() {
        let mut it = super::a();
        assert_eq!(Some((512, 8, 3)), it.nth(1));
        assert_eq!(Some((614656, 28, 4)), it.nth(8 - 1));
    }
}
