#![crate_id = "prob0118"]
#![crate_type = "rlib"]

extern crate num;
extern crate math;

use std::{iter, mem};
use num::Integer;
use math::numconv;
use math::prime::Prime;

pub static EXPECTED_ANSWER: &'static str = "44680";

trait ImmutableCloneableVector<T> {
    fn groups(&self, n: uint) -> Groups<T>;
}

impl<'a, T: Clone> ImmutableCloneableVector<T> for &'a [T] {
    #[inline]
    fn groups(&self, n: uint) -> Groups<T> { Groups::new(n, Vec::from_slice(*self)) }
}

struct ElementIndex {
    num_elem: uint,
    idx: Option<Vec<uint>>
}

impl ElementIndex {
    #[inline]
    fn new(num_select_elem: uint, num_all_elem: uint) -> ElementIndex {
        ElementIndex {
            num_elem: num_all_elem,
            idx: if num_select_elem > num_all_elem {
                None
            } else {
                Some(Vec::from_fn(num_select_elem, |i| i))
            }
        }
    }
}

impl Iterator<Vec<uint>> for ElementIndex {
    #[inline]
    fn next(&mut self) -> Option<Vec<uint>> {
        let next = self.idx
            .as_ref()
            .and_then(|idx| {
                let max_num = self.num_elem - 1;
                let max_idx = idx.len() - 1;
                range(0, idx.len())
                    .rev()
                    .find(|&i| *idx.get(i) < max_num - (max_idx - i))
                    .map(|incr_idx| (idx, incr_idx))
            }).map(|(idx, incr_idx)| {
                let mut next = idx.clone();
                *next.get_mut(incr_idx) += 1;
                for j in range(incr_idx + 1, idx.len()) {
                    *next.get_mut(j) = *next.get_mut(incr_idx) + (j - incr_idx);
                }
                next
            });
        mem::replace(&mut self.idx, next)
    }
}

struct Groups<T> {
    idx: ElementIndex,
    vec: Vec<T>
}

impl<T: Clone> Groups<T> {
    fn new(num_select_elem: uint, v: Vec<T>) -> Groups<T> {
        Groups { idx: ElementIndex::new(num_select_elem, v.len()), vec: v }
    }
}

impl<T: Clone> Iterator<(Vec<T>, Vec<T>)> for Groups<T> {
    #[inline]
    fn next(&mut self) -> Option<(Vec<T>, Vec<T>)> {
        self.idx
            .next()
            .map(|idx| {
                let left = Vec::from_fn(idx.len(), |i| self.vec.get(*idx.get(i)).clone());
                let mut offset = 0;
                let right = Vec::from_fn(self.vec.len() - idx.len(), |i| {
                        while offset < idx.len() && offset + i == *idx.get(offset) { offset += 1; }
                        self.vec.get(offset + i).clone()
                    });
                (left.move_iter().collect(), right.move_iter().collect())
            })
    }
}

fn count_primes(prime: &Prime, digits: &[uint]) -> uint {
    if digits.len() == 0 { return 1 }

    let mut cnt = 0;
    for n in iter::range_inclusive(1, digits.len()) {
        for (ds, rest) in digits.groups(n) {
            if *ds.get(0) != digits[0] { break }
            if rest.len() == 1 && !prime.contains(*rest.get(0)) { continue }

            let num_prime = if ds.len() == 1 {
                if prime.contains(*ds.get(0)) { 1 } else { 0 }
            } else {
                if ds.iter().fold(0, |x, &y| x + y) % 3 != 0 {
                    ds.as_slice().permutations()
                        .filter(|perm| perm[0].is_odd() && perm[0] != 5)
                        .filter(|perm| prime.contains(numconv::from_digits(*perm, 10)))
                        .len()
                } else {
                    0
                }
            };

            if num_prime != 0 {
                let rest_primes = count_primes(prime, rest.as_slice());
                cnt += num_prime * rest_primes;
            }
        }
    }
    cnt
}

pub fn solve() -> ~str {
    let digits = iter::range_inclusive(1u, 9).collect::<Vec<uint>>();
    let prime = Prime::new();
    count_primes(&prime, digits.as_slice()).to_str()
}

#[cfg(test)]
mod tests {
    use super::{ImmutableCloneableVector, ElementIndex};

    #[test]
    fn element_index() {
        let mut it = ElementIndex::new(0, 0);
        assert_eq!(Some(vec![]), it.next());
        assert_eq!(None, it.next());

        let mut it = ElementIndex::new(3, 3);
        assert_eq!(Some(vec![0, 1, 2]), it.next());
        assert_eq!(None, it.next());

        let mut it = ElementIndex::new(2, 3);
        assert_eq!(Some(vec![0, 1]), it.next());
        assert_eq!(Some(vec![0, 2]), it.next());
        assert_eq!(Some(vec![1, 2]), it.next());
        assert_eq!(None, it.next());

        let mut it = ElementIndex::new(1, 3);
        assert_eq!(Some(vec![0]), it.next());
        assert_eq!(Some(vec![1]), it.next());
        assert_eq!(Some(vec![2]), it.next());
        assert_eq!(None, it.next());

        let mut it = ElementIndex::new(4, 3);
        assert_eq!(None, it.next());

        let mut it = ElementIndex::new(3, 6);
        assert_eq!(Some(vec![0, 1, 2]), it.next());
        assert_eq!(Some(vec![0, 1, 3]), it.next());
        assert_eq!(Some(vec![0, 1, 4]), it.next());
        assert_eq!(Some(vec![0, 1, 5]), it.next());
        assert_eq!(Some(vec![0, 2, 3]), it.next());
        assert_eq!(Some(vec![0, 2, 4]), it.next());
        assert_eq!(Some(vec![0, 2, 5]), it.next());
        assert_eq!(Some(vec![0, 3, 4]), it.next());
        assert_eq!(Some(vec![0, 3, 5]), it.next());
        assert_eq!(Some(vec![0, 4, 5]), it.next());
        assert_eq!(Some(vec![1, 2, 3]), it.next());
        assert_eq!(Some(vec![1, 2, 4]), it.next());
        assert_eq!(Some(vec![1, 2, 5]), it.next());
        assert_eq!(Some(vec![1, 3, 4]), it.next());
        assert_eq!(Some(vec![1, 3, 5]), it.next());
        assert_eq!(Some(vec![1, 4, 5]), it.next());
        assert_eq!(Some(vec![2, 3, 4]), it.next());
        assert_eq!(Some(vec![2, 3, 5]), it.next());
        assert_eq!(Some(vec![2, 4, 5]), it.next());
        assert_eq!(Some(vec![3, 4, 5]), it.next());
        assert_eq!(None, it.next());
    }

    #[test]
    fn groups() {
        let v = [1, 2, 3];
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

        let mut it = v.groups(4);
        assert_eq!(None, it.next());
    }
}
