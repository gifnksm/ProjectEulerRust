#[crate_id = "prob0118"];
#[crate_type = "rlib"];

extern mod math;

use std::{iter, util, vec};
use math::numconv;
use math::prime::Prime;

pub static EXPECTED_ANSWER: &'static str = "44680";

pub trait ImmutableCloneableVector<T> {
    fn groups(&self, n: uint) -> Groups<T>;
}

impl<'a, T: Clone> ImmutableCloneableVector<T> for &'a [T] {
    #[inline]
    fn groups(&self, n: uint) -> Groups<T> { Groups::new(n, self.to_owned()) }
}

struct ElementIndex {
    num_elem: uint,
    idx: Option<~[uint]>
}

impl ElementIndex {
    #[inline]
    fn new(num_select_elem: uint, num_all_elem: uint) -> ElementIndex {
        ElementIndex {
            num_elem: num_all_elem,
            idx: if num_select_elem > num_all_elem {
                None
            } else {
                Some(vec::from_fn(num_select_elem, |i| i))
            }
        }
    }
}

impl Iterator<~[uint]> for ElementIndex {
    #[inline]
    fn next(&mut self) -> Option<~[uint]> {
        let next = self.idx
            .as_ref()
            .and_then(|idx| {
                let max_num = self.num_elem - 1;
                let max_idx = idx.len() - 1;
                range(0, idx.len())
                    .invert()
                    .find(|&i| idx[i] < max_num - (max_idx - i))
                    .map(|incr_idx| (idx, incr_idx))
            }).map(|(idx, incr_idx)| {
                let mut next = idx.clone();
                next[incr_idx] += 1;
                for j in range(incr_idx + 1, idx.len()) {
                    next[j] = next[incr_idx] + (j - incr_idx);
                }
                next
            });
        util::replace(&mut self.idx, next)
    }
}

pub struct Groups<T> {
    idx: ElementIndex,
    vec: ~[T]
}

impl<T: Clone> Groups<T> {
    fn new(num_select_elem: uint, v: ~[T]) -> Groups<T> {
        Groups { idx: ElementIndex::new(num_select_elem, v.len()), vec: v }
    }
}

impl<T: Clone> Iterator<(~[T], ~[T])> for Groups<T> {
    #[inline]
    fn next(&mut self) -> Option<(~[T], ~[T])> {
        self.idx
            .next()
            .map(|idx| {
                let left = vec::from_fn(idx.len(), |i| self.vec[idx[i]].clone());
                let mut offset = 0;
                let right = vec::from_fn(self.vec.len() - idx.len(), |i| {
                        while offset < idx.len() && offset + i == idx[offset] { offset += 1; }
                        self.vec[offset + i].clone()
                    });
                (left, right)
            })
    }
}

fn count_primes(prime: &Prime, digits: &[uint]) -> uint {
    if digits.len() == 0 { return 1 }

    let mut cnt = 0;
    for n in iter::range_inclusive(1, digits.len()) {
        for (ds, rest) in digits.groups(n) {
            if ds[0] != digits[0] { break }
            if rest.len() == 1 && !prime.contains(rest[0]) { continue }

            let num_prime = if ds.len() == 1 {
                if prime.contains(ds[0]) { 1 } else { 0 }
            } else {
                if ds.iter().fold(0, |x, &y| x + y) % 3 != 0 {
                    ds.permutations()
                        .filter(|perm| perm[0].is_odd() && perm[0] != 5)
                        .filter(|perm| prime.contains(numconv::from_digits(*perm, 10)))
                        .len()
                } else {
                    0
                }
            };

            if num_prime != 0 {
                let rest_primes = count_primes(prime, rest);
                cnt += num_prime * rest_primes;
            }
        }
    }
    cnt
}

pub fn solve() -> ~str {
    let digits = iter::range_inclusive(1u, 9).to_owned_vec();
    let prime = Prime::new();
    count_primes(&prime, digits).to_str()
}

#[cfg(test)]
mod test {
    use super::{ImmutableCloneableVector, ElementIndex};

    #[test]
    fn element_index() {
        let mut it = ElementIndex::new(0, 0);
        assert_eq!(Some(~[]), it.next());
        assert_eq!(None, it.next());

        let mut it = ElementIndex::new(3, 3);
        assert_eq!(Some(~[0, 1, 2]), it.next());
        assert_eq!(None, it.next());

        let mut it = ElementIndex::new(2, 3);
        assert_eq!(Some(~[0, 1]), it.next());
        assert_eq!(Some(~[0, 2]), it.next());
        assert_eq!(Some(~[1, 2]), it.next());
        assert_eq!(None, it.next());

        let mut it = ElementIndex::new(1, 3);
        assert_eq!(Some(~[0]), it.next());
        assert_eq!(Some(~[1]), it.next());
        assert_eq!(Some(~[2]), it.next());
        assert_eq!(None, it.next());

        let mut it = ElementIndex::new(4, 3);
        assert_eq!(None, it.next());

        let mut it = ElementIndex::new(3, 6);
        assert_eq!(Some(~[0, 1, 2]), it.next());
        assert_eq!(Some(~[0, 1, 3]), it.next());
        assert_eq!(Some(~[0, 1, 4]), it.next());
        assert_eq!(Some(~[0, 1, 5]), it.next());
        assert_eq!(Some(~[0, 2, 3]), it.next());
        assert_eq!(Some(~[0, 2, 4]), it.next());
        assert_eq!(Some(~[0, 2, 5]), it.next());
        assert_eq!(Some(~[0, 3, 4]), it.next());
        assert_eq!(Some(~[0, 3, 5]), it.next());
        assert_eq!(Some(~[0, 4, 5]), it.next());
        assert_eq!(Some(~[1, 2, 3]), it.next());
        assert_eq!(Some(~[1, 2, 4]), it.next());
        assert_eq!(Some(~[1, 2, 5]), it.next());
        assert_eq!(Some(~[1, 3, 4]), it.next());
        assert_eq!(Some(~[1, 3, 5]), it.next());
        assert_eq!(Some(~[1, 4, 5]), it.next());
        assert_eq!(Some(~[2, 3, 4]), it.next());
        assert_eq!(Some(~[2, 3, 5]), it.next());
        assert_eq!(Some(~[2, 4, 5]), it.next());
        assert_eq!(Some(~[3, 4, 5]), it.next());
        assert_eq!(None, it.next());
    }

    #[test]
    fn groups() {
        let v = ~[1, 2, 3];
        let mut it = v.groups(0);
        assert_eq!(Some((~[], ~[1, 2, 3])), it.next());
        assert_eq!(None, it.next());

        let mut it = v.groups(1);
        assert_eq!(Some((~[1], ~[2, 3])), it.next());
        assert_eq!(Some((~[2], ~[1, 3])), it.next());
        assert_eq!(Some((~[3], ~[1, 2])), it.next());
        assert_eq!(None, it.next());

        let mut it = v.groups(2);
        assert_eq!(Some((~[1, 2], ~[3])), it.next());
        assert_eq!(Some((~[1, 3], ~[2])), it.next());
        assert_eq!(Some((~[2, 3], ~[1])), it.next());
        assert_eq!(None, it.next());

        let mut it = v.groups(3);
        assert_eq!(Some((~[1, 2, 3], ~[])), it.next());
        assert_eq!(None, it.next());

        let mut it = v.groups(4);
        assert_eq!(None, it.next());
    }
}
