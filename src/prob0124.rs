#![crate_id = "prob0124"]
#![crate_type = "rlib"]

extern crate collections;
extern crate math;
#[cfg(test)]
extern crate test;

use collections::priority_queue::PriorityQueue;
use math::prime::Prime;

pub static EXPECTED_ANSWER: &'static str = "21417";

struct Multiple(uint, uint);

impl Eq for Multiple {
    fn eq(&self, other: &Multiple) -> bool {
        let Multiple(ref sn, _) = *self;
        let Multiple(ref on, _) = *other;
        on.eq(sn)
    }
}

impl Ord for Multiple {
    #[inline]
    fn lt(&self, other: &Multiple) -> bool {
        let Multiple(ref sn, _) = *self;
        let Multiple(ref on, _) = *other;
        on.lt(sn)
    }
}

struct Multiples {
    facts: Vec<uint>,
    queue: PriorityQueue<Multiple>
}

impl Multiples {
    #[inline]
    fn new(base: uint, facts: Vec<uint>) -> Multiples {
        let mut queue = PriorityQueue::new();
        queue.push(Multiple(base, 0));
        Multiples { facts: facts, queue: queue }
    }
}

impl Iterator<uint> for Multiples {
    #[inline]
    fn next(&mut self) -> Option<uint> {
        if self.queue.is_empty() { return None }

        let Multiple(n, i) = self.queue.pop();

        if i < self.facts.len() {
            // n = ... * f[i]^k => ... * f[i]^(k+1)
            self.queue.push(Multiple(n * *self.facts.get(i), i));
        }

        for j in range(i + 1, self.facts.len()) {
            // n = ... * f[i]^k => ... * f[i]^k * f[j]
            self.queue.push(Multiple(n * *self.facts.get(j), j));
        }

        Some(n)
    }
}

struct RadValue(uint, Vec<uint>, uint);

impl Eq for RadValue {
    #[inline]
    fn eq(&self, other: &RadValue) -> bool {
        let RadValue(ref sn, _, _) = *self;
        let RadValue(ref on, _, _) = *other;
        on.eq(sn)
    }
}

impl Ord for RadValue {
    #[inline]
    fn lt(&self, other: &RadValue) -> bool {
        let RadValue(ref sn, _, _) = *self;
        let RadValue(ref on, _, _) = *other;
        on.lt(sn)
    }
}

struct RadValues {
    prime: Prime,
    queue: PriorityQueue<RadValue>
}

impl RadValues {
    #[inline]
    fn new() -> RadValues {
        let mut queue = PriorityQueue::new();
        queue.push(RadValue(1, vec![], 0));
        RadValues { prime: Prime::new(), queue: queue }
    }
}

impl Iterator<(uint, Vec<uint>)> for RadValues {
    #[inline]
    fn next(&mut self) -> Option<(uint, Vec<uint>)> {
        let RadValue(n, facts, i) = self.queue.pop();
        let p = self.prime.nth(i);

        // n = ... * p[i-1] => ... * p[i-1] * p[i] (append p[i])
        self.queue.push(RadValue(n * p, facts.clone().append_one(p), i + 1));

        if !facts.is_empty() {
            // n = ... * p[i-1] => ... * p[i] (replace p[i-1] with p[i])
            let last = *facts.last().unwrap();
            let mut next_facts = facts.clone();
            let len = next_facts.len();
            *next_facts.get_mut(len - 1) = p;
            self.queue.push(RadValue(p * n / last, next_facts, i + 1));
        }

        Some((n, facts))
    }
}

pub fn solve() -> ~str {
    static LIMIT: uint = 100000;
    let index = 10000;

    RadValues::new()
        .take_while(|&(n, _)| n <= LIMIT)
        .flat_map(|(base, facts)| Multiples::new(base, facts).take_while(|&n| n <= LIMIT))
        .nth(index - 1)
        .unwrap()
        .to_str()
}

#[cfg(test)]
mod tests {
    use super::{Multiples, RadValues};

    #[test]
    fn rad_nums() {
        let mut it = RadValues::new();
        assert_eq!(Some((1, vec![])), it.next());
        assert_eq!(Some((2, vec![2])), it.next());
        assert_eq!(Some((3, vec![3])), it.next());
        assert_eq!(Some((5, vec![5])), it.next());
        assert_eq!(Some((6, vec![2, 3])), it.next());
        assert_eq!(Some((7, vec![7])), it.next());
        assert_eq!(Some((10, vec![2, 5])), it.next());
        assert_eq!(Some((11, vec![11])), it.next());
        assert_eq!(Some((13, vec![13])), it.next());
        assert_eq!(Some((14, vec![2, 7])), it.next());
        assert_eq!(Some((15, vec![3, 5])), it.next());
        assert_eq!(Some((17, vec![17])), it.next());
        assert_eq!(Some((19, vec![19])), it.next());
        assert_eq!(Some((21, vec![3, 7])), it.next());
        assert_eq!(Some((22, vec![2, 11])), it.next());
        assert_eq!(Some((23, vec![23])), it.next());
        assert_eq!(Some((26, vec![2, 13])), it.next());
        assert_eq!(Some((29, vec![29])), it.next());
        assert_eq!(Some((30, vec![2, 3, 5])), it.next());
    }

    #[test]
    fn prod_nums() {
        let mut it = Multiples::new(1, vec![]);
        assert_eq!(Some(1), it.next());
        assert_eq!(None, it.next());

        let mut it = Multiples::new(2, vec![2]);
        assert_eq!(Some(2), it.next());
        assert_eq!(Some(4), it.next());
        assert_eq!(Some(8), it.next());
        assert_eq!(Some(16), it.next());
        assert_eq!(Some(32), it.next());

        let mut it = Multiples::new(6, vec![2, 3]);
        assert_eq!(Some(6 * 1), it.next());
        assert_eq!(Some(6 * 2), it.next());
        assert_eq!(Some(6 * 3), it.next());
        assert_eq!(Some(6 * 4), it.next());
        assert_eq!(Some(6 * 6), it.next());
        assert_eq!(Some(6 * 8), it.next());
        assert_eq!(Some(6 * 9), it.next());
        assert_eq!(Some(6 * 12), it.next());
        assert_eq!(Some(6 * 16), it.next());
        assert_eq!(Some(6 * 18), it.next());
        assert_eq!(Some(6 * 24), it.next());

        let mut it = Multiples::new(30, vec![2, 3, 5]);
        assert_eq!(Some(30 * 1), it.next());
        assert_eq!(Some(30 * 2), it.next());
        assert_eq!(Some(30 * 3), it.next());
        assert_eq!(Some(30 * 4), it.next());
        assert_eq!(Some(30 * 5), it.next());
        assert_eq!(Some(30 * 6), it.next());
        assert_eq!(Some(30 * 8), it.next());
        assert_eq!(Some(30 * 9), it.next());
        assert_eq!(Some(30 * 10), it.next());
        assert_eq!(Some(30 * 12), it.next());
        assert_eq!(Some(30 * 15), it.next());
        assert_eq!(Some(30 * 16), it.next());
        assert_eq!(Some(30 * 18), it.next());
        assert_eq!(Some(30 * 20), it.next());
        assert_eq!(Some(30 * 24), it.next());
        assert_eq!(Some(30 * 25), it.next());
    }

    #[test]
    fn rad() {
        let mut it = RadValues::new()
            .take_while(|&(n, _)| n <= 10)
            .flat_map(|(base, facts)| Multiples::new(base, facts).take_while(|&n| n <= 10));

        assert_eq!(Some(1), it.next());
        assert_eq!(Some(2), it.next());
        assert_eq!(Some(4), it.next());
        assert_eq!(Some(8), it.next());
        assert_eq!(Some(3), it.next());
        assert_eq!(Some(9), it.next());
        assert_eq!(Some(5), it.next());
        assert_eq!(Some(6), it.next());
        assert_eq!(Some(7), it.next());
        assert_eq!(Some(10), it.next());
        assert_eq!(None, it.next());
    }
}

#[cfg(test)]
mod bench {
    use test::Bencher;
    use super::RadValues;

    #[bench]
    fn rad_value_below_10000(bh: &mut Bencher) {
        bh.iter(|| for _n in RadValues::new().take_while(|&(n, _)| n < 10000) {})
    }
}
