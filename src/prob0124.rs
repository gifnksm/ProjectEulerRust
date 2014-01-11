#[crate_type = "rlib"];

extern mod extra;
extern mod math;

use extra::priority_queue::PriorityQueue;
use math::prime::Prime;

pub static EXPECTED_ANSWER: &'static str = "21417";

struct Multiple(uint, uint);

impl Ord for Multiple {
    #[inline]
    fn lt(&self, other: &Multiple) -> bool {
        let Multiple(ref sn, _) = *self;
        let Multiple(ref on, _) = *other;
        on.lt(sn)
    }
}

struct Multiples {
    facts: ~[uint],
    queue: PriorityQueue<Multiple>
}

impl Multiples {
    #[inline]
    fn new(base: uint, facts: ~[uint]) -> Multiples {
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
            self.queue.push(Multiple(n * self.facts[i], i));
        }

        for j in range(i + 1, self.facts.len()) {
            // n = ... * f[i]^k => ... * f[i]^k * f[j]
            self.queue.push(Multiple(n * self.facts[j], j));
        }

        Some(n)
    }
}

struct RadValue(uint, ~[uint], uint);

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
        queue.push(RadValue(1, ~[], 0));
        RadValues { prime: Prime::new(), queue: queue }
    }
}

impl Iterator<(uint, ~[uint])> for RadValues {
    #[inline]
    fn next(&mut self) -> Option<(uint, ~[uint])> {
        let RadValue(n, facts, i) = self.queue.pop();
        let p = self.prime.nth(i);

        // n = ... * p[i-1] => ... * p[i-1] * p[i] (append p[i])
        self.queue.push(RadValue(n * p, facts + &[p], i + 1));

        if !facts.is_empty() {
            // n = ... * p[i-1] => ... * p[i] (replace p[i-1] with p[i])
            let last = *facts.last();
            let mut next_facts = facts.clone();
            next_facts[next_facts.len() - 1] = p;
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
mod test {
    use super::{Multiples, RadValues};

    #[test]
    fn rad_nums() {
        let mut it = RadValues::new();
        assert_eq!(Some((1, ~[])), it.next());
        assert_eq!(Some((2, ~[2])), it.next());
        assert_eq!(Some((3, ~[3])), it.next());
        assert_eq!(Some((5, ~[5])), it.next());
        assert_eq!(Some((6, ~[2, 3])), it.next());
        assert_eq!(Some((7, ~[7])), it.next());
        assert_eq!(Some((10, ~[2, 5])), it.next());
        assert_eq!(Some((11, ~[11])), it.next());
        assert_eq!(Some((13, ~[13])), it.next());
        assert_eq!(Some((14, ~[2, 7])), it.next());
        assert_eq!(Some((15, ~[3, 5])), it.next());
        assert_eq!(Some((17, ~[17])), it.next());
        assert_eq!(Some((19, ~[19])), it.next());
        assert_eq!(Some((21, ~[3, 7])), it.next());
        assert_eq!(Some((22, ~[2, 11])), it.next());
        assert_eq!(Some((23, ~[23])), it.next());
        assert_eq!(Some((26, ~[2, 13])), it.next());
        assert_eq!(Some((29, ~[29])), it.next());
        assert_eq!(Some((30, ~[2, 3, 5])), it.next());
    }

    #[test]
    fn prod_nums() {
        let mut it = Multiples::new(1, ~[]);
        assert_eq!(Some(1), it.next());
        assert_eq!(None, it.next());

        let mut it = Multiples::new(2, ~[2]);
        assert_eq!(Some(2), it.next());
        assert_eq!(Some(4), it.next());
        assert_eq!(Some(8), it.next());
        assert_eq!(Some(16), it.next());
        assert_eq!(Some(32), it.next());

        let mut it = Multiples::new(6, ~[2, 3]);
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

        let mut it = Multiples::new(30, ~[2, 3, 5]);
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
    use extra::test::BenchHarness;
    use super::RadValues;

    #[bench]
    fn rad_value_below_10000(bh: &mut BenchHarness) {
        bh.iter(|| for _n in RadValues::new().take_while(|&(n, _)| n < 10000) {})
    }
}
