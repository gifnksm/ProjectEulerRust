//! [Problem 110](https://projecteuler.net/problem=110) solver.

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

struct Elem(u64, Vec<u64>);

impl PartialEq for Elem {
    fn eq(&self, other: &Elem) -> bool {
        self.0 == other.0
    }
}
impl Eq for Elem {}
impl PartialOrd for Elem {
    fn partial_cmp(&self, other: &Elem) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Elem {
    fn cmp(&self, other: &Elem) -> Ordering {
        self.0.cmp(&other.0).reverse()
    }
}

fn compute(limit: u64) -> u64 {
    let ps = PrimeSet::new();
    let mut heap = BinaryHeap::new();
    heap.push(Elem(2, vec![1]));

    loop {
        let Elem(n, mut pairs) = heap.pop().unwrap();
        let num_sol = (pairs.iter().fold(1, |n, &i| n * (2 * i + 1)) + 1) / 2;
        if num_sol > limit {
            return n;
        }

        if pairs.len() == 1 || pairs[pairs.len() - 1] < pairs[pairs.len() - 2] {
            let mut new_pairs = pairs.clone();
            new_pairs[pairs.len() - 1] += 1;
            heap.push(Elem(n * ps.nth(pairs.len() - 1), new_pairs));
        }
        pairs.push(1);
        heap.push(Elem(n * ps.nth(pairs.len() - 1), pairs));
    }
}

fn solve() -> String {
    compute(4000000).to_string()
}

common::problem!("9350130049860600", solve);

#[cfg(test)]
mod tests {
    #[test]
    fn hundred() {
        assert_eq!(1260, super::compute(100));
    }
}
