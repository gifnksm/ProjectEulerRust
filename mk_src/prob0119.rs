#![crate_name = "prob0119"]
#![crate_type = "rlib"]

extern crate math;

use std::num::Int;
use std::iter::AdditiveIterator;
use std::collections::BinaryHeap;
use math::numconv;

pub const EXPECTED_ANSWER: &'static str = "248155780267521";

struct Power(uint, uint, uint);

impl PartialEq for Power {
    #[inline]
    fn eq(&self, other: &Power) -> bool {
        let Power(sn, sb, _) = *self;
        let Power(on, ob, _) = *other;
        sn == on && sb == ob
    }
}

impl Eq for Power {}

impl PartialOrd for Power {
    fn partial_cmp(&self, other: &Power) -> Option<Ordering> { Some(self.cmp(other)) }
}

impl Ord for Power {
    fn cmp(&self, other: &Power) -> Ordering {
        let Power(sn, sb, _) = *self;
        let Power(on, ob, _) = *other;
        (sn, sb).cmp(&(on, ob)).reverse()
    }
}

struct Powers {
    heap: BinaryHeap<Power>
}

impl Powers {
    #[inline]
    fn new() -> Powers {
        let mut heap = BinaryHeap::new();
        heap.push(Power(4, 2, 2));
        Powers { heap: heap }
    }
}

impl Iterator<(uint, uint, uint)> for Powers {
    #[inline]
    fn next(&mut self) -> Option<(uint, uint, uint)> {
        let Power(n, b, e) = self.heap.pop().unwrap();
        if b == 2 { self.heap.push(Power(n * b, b, e + 1)); }
        self.heap.push(Power((b + 1).pow(e), b + 1, e));
        Some((n, b, e))
    }
}

pub fn solve() -> String {
    let (n, _b, _e) = Powers::new()
        .skip_while(|&mut: &(n, _b, _e)| n < 10)
        .filter(|&(n, b, _e)| numconv::to_digits(n, 10).sum() == b)
        .nth(29).unwrap();
    n.to_string()
}

#[cfg(test)]
mod tests {
    use super::Powers;
    use std::iter::AdditiveIterator;
    use math::numconv;

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
        let mut it = Powers::new()
            .skip_while(|&mut: &(n, _b, _e)| n < 10)
            .filter(|&(n, b, _e)| numconv::to_digits(n, 10).sum() == b);
        assert_eq!(Some((512, 8, 3)), it.nth(1));
        assert_eq!(Some((614656, 28, 4)), it.nth(8 - 1));
    }
}
