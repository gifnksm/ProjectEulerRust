//! [Problem 124](https://projecteuler.net/problem=124) solver.

#![warn(
    bad_style,
    unused,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

use prime::PrimeSet;
use std::{cmp::Ordering, collections::BinaryHeap};

struct Multiple(u64, u64);

impl PartialEq for Multiple {
    fn eq(&self, other: &Multiple) -> bool {
        self.0.eq(&other.0)
    }
}

impl Eq for Multiple {}

impl PartialOrd for Multiple {
    fn partial_cmp(&self, other: &Multiple) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Multiple {
    #[inline]
    fn cmp(&self, other: &Multiple) -> Ordering {
        other.0.cmp(&self.0)
    }
}

struct Multiples {
    facts: Vec<u64>,
    heap: BinaryHeap<Multiple>,
}

impl Multiples {
    #[inline]
    fn new(base: u64, facts: Vec<u64>) -> Multiples {
        let mut heap = BinaryHeap::new();
        heap.push(Multiple(base, 0));
        Multiples { facts, heap }
    }
}

impl Iterator for Multiples {
    type Item = u64;

    #[inline]
    fn next(&mut self) -> Option<u64> {
        self.heap.pop().map(|Multiple(n, i)| {
            if i < self.facts.len() as u64 {
                // n = ... * f[i]^k => ... * f[i]^(k+1)
                self.heap.push(Multiple(n * self.facts[i as usize], i));
            }

            for j in (i + 1)..(self.facts.len() as u64) {
                // n = ... * f[i]^k => ... * f[i]^k * f[j]
                self.heap.push(Multiple(n * self.facts[j as usize], j));
            }
            n
        })
    }
}

struct RadValue(u64, Vec<u64>, u64);

impl PartialEq for RadValue {
    #[inline]
    fn eq(&self, other: &RadValue) -> bool {
        self.0.eq(&other.0)
    }
}

impl Eq for RadValue {}

impl PartialOrd for RadValue {
    fn partial_cmp(&self, other: &RadValue) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for RadValue {
    #[inline]
    fn cmp(&self, other: &RadValue) -> Ordering {
        other.0.cmp(&self.0)
    }
}

struct RadValues {
    ps: PrimeSet,
    heap: BinaryHeap<RadValue>,
}

impl RadValues {
    #[inline]
    fn new() -> RadValues {
        let mut heap = BinaryHeap::new();
        heap.push(RadValue(1, vec![], 0));
        RadValues {
            ps: PrimeSet::new(),
            heap,
        }
    }
}

impl Iterator for RadValues {
    type Item = (u64, Vec<u64>);

    #[inline]
    fn next(&mut self) -> Option<(u64, Vec<u64>)> {
        self.heap.pop().map(|RadValue(n, facts, i)| {
            let p = self.ps.nth(i as usize);

            // n = ... * p[i-1] => ... * p[i-1] * p[i] (append p[i])
            {
                let mut v = facts.clone();
                v.push(p);
                self.heap.push(RadValue(n * p, v, i + 1));
            }

            if !facts.is_empty() {
                // n = ... * p[i-1] => ... * p[i] (replace p[i-1] with p[i])
                let last = *facts.last().unwrap();
                let mut next_facts = facts.clone();
                let len = next_facts.len();
                next_facts[len - 1] = p;
                self.heap.push(RadValue(p * n / last, next_facts, i + 1));
            }

            (n, facts)
        })
    }
}

fn solve() -> String {
    const LIMIT: u64 = 100000;
    let index = 10000;

    RadValues::new()
        .take_while(|&(n, _)| n <= LIMIT)
        .flat_map(|(base, facts)| Multiples::new(base, facts).take_while(|&n| n <= LIMIT))
        .nth(index - 1)
        .unwrap()
        .to_string()
}

common::problem!("21417", solve);

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
        assert_eq!(Some(6), it.next());
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
        assert_eq!(Some(30), it.next());
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
