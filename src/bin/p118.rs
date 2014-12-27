//! [Problem 118](https://projecteuler.net/problem=118) solver.

#![warn(bad_style,
        unused, unused_extern_crates, unused_import_braces,
        unused_qualifications, unused_results, unused_typecasts)]

#![feature(phase)]

#[phase(plugin, link)] extern crate common;
extern crate integer;
extern crate "iter" as itercrate;
extern crate num;
extern crate prime;

use std::iter;
use integer::Integer;
use itercrate::BitCombination;
use num::Integer as NumInteger;
use prime::PrimeSet;


trait ImmutableCloneableVector<T> {
    fn groups(&self, n: uint) -> Groups<T>;
}

impl<'a, T: Clone> ImmutableCloneableVector<T> for &'a [T] {
    #[inline]
    fn groups(&self, n: uint) -> Groups<T> { Groups::new(n, self.to_vec()) }
}

struct Groups<T> {
    idx: BitCombination,
    vec: Vec<T>
}

impl<T: Clone> Groups<T> {
    fn new(num_select_elem: uint, v: Vec<T>) -> Groups<T> {
        Groups { idx: BitCombination::new(num_select_elem, v.len()), vec: v }
    }
}

impl<T: Clone> Iterator<(Vec<T>, Vec<T>)> for Groups<T> {
    #[inline]
    fn next(&mut self) -> Option<(Vec<T>, Vec<T>)> {
        if let Some(idx) = self.idx.next() {
            let mut left  = Vec::with_capacity(idx.len());
            let mut right = Vec::with_capacity(self.vec.len() - idx.len());
            for (i, e) in self.vec.iter().enumerate() {
                if idx.contains(&i) {
                    left.push(e.clone())
                } else {
                    right.push(e.clone())
                }
            }
            return Some((left, right))
        }
        None
    }
}

fn count_primes(ps: &PrimeSet, digits: &[u64]) -> uint {
    if digits.len() == 0 { return 1 }

    let mut cnt = 0;
    for n in iter::range_inclusive(1, digits.len()) {
        for (ds, rest) in digits.groups(n) {
            if ds[0] != digits[0] { break }
            if rest.len() == 1 && !ps.contains(rest[0]) { continue }

            let num_prime = if ds.len() == 1 {
                if ps.contains(ds[0]) { 1 } else { 0 }
            } else {
                if ds.iter().fold(0, |x, &y| x + y) % 3 != 0 {
                    ds.as_slice().permutations()
                        .filter(|perm| perm[0].is_odd() && perm[0] != 5)
                        .filter(|perm| ps.contains(Integer::from_digits(perm.iter().map(|&x| x), 10)))
                        .count()
                } else {
                    0
                }
            };

            if num_prime != 0 {
                let rest_primes = count_primes(ps, rest.as_slice());
                cnt += num_prime * rest_primes;
            }
        }
    }
    cnt
}

fn solve() -> String {
    let digits = iter::range_inclusive(1, 9).collect::<Vec<_>>();
    let ps = PrimeSet::new();
    count_primes(&ps, digits.as_slice()).to_string()
}

problem!("44680", solve);

#[cfg(test)]
mod tests {
    use super::ImmutableCloneableVector;

    #[test]
    fn groups() {
        let v = [1u, 2, 3];
        let v = v.as_slice();
        let mut it = v.groups(0);
        assert_eq!(Some((vec![], vec![1, 2, 3])), it.next());
        assert_eq!(None, it.next());

        let mut it = v.groups(1);
        assert_eq!(Some((vec![1], vec![2, 3])), it.next());
        assert_eq!(Some((vec![2], vec![1, 3])), it.next());
        assert_eq!(Some((vec![3], vec![1, 2])), it.next());
        assert_eq!(None, it.next());

        let mut it = v.groups(2);
        assert_eq!(Some((vec![1, 2], vec![3])), it.next());
        assert_eq!(Some((vec![1, 3], vec![2])), it.next());
        assert_eq!(Some((vec![2, 3], vec![1])), it.next());
        assert_eq!(None, it.next());

        let mut it = v.groups(3);
        assert_eq!(Some((vec![1, 2, 3], vec![])), it.next());
        assert_eq!(None, it.next());
    }
}
