//! Prime number generator and related functions.

#![warn(unused, bad_style,
        missing_doc, unnecessary_qualification, unnecessary_typecast,
        unused_result)]

#![feature(macro_rules)]

extern crate num;
#[cfg(test)] extern crate test;

use std::{iter, mem, uint};
use std::cell::RefCell;
use std::num::One;
use std::rc::Rc;
use num::Integer;

static SMALL_PRIMES: &'static [u64] = &[
      2,   3,   5,   7,  11,  13,  17,  19,  23,  29,  31,  37,  41,  43,  47,
     53,  59,  61,  67,  71,  73,  79,  83,  89,  97, 101, 103, 107, 109, 113,
    127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181, 191, 193, 197,
    199, 211, 223, 227, 229, 233, 239, 241, 251, 257, 263, 269, 271, 277, 281,
    283, 293, 307, 311, 313, 317, 331, 337, 347, 349, 353, 359, 367, 373, 379,
    383, 389, 397, 401, 409, 419, 421, 431, 433, 439, 443, 449, 457, 461, 463,
    467, 479, 487, 491, 499, 503, 509, 521, 523, 541, 547, 557, 563, 569, 571,
    577, 587, 593, 599, 601, 607, 613, 617, 619, 631, 641, 643, 647, 653, 659,
    661, 673, 677, 683, 691, 701, 709, 719, 727, 733, 739, 743, 751, 757, 761,
    769, 773, 787, 797, 809, 811, 821, 823, 827, 829, 839, 853, 857, 859, 863,
    877, 881, 883, 887, 907, 911, 919, 929, 937, 941, 947, 953, 967, 971, 977,
    983, 991, 997
];

static INITIAL_CAPACITY: uint = 10000;

struct PrimeInner {
    data: Vec<u64>
}

impl PrimeInner {
    #[inline]
    fn new() -> PrimeInner { PrimeInner::with_capacity(INITIAL_CAPACITY) }

    #[inline]
    fn new_empty() -> PrimeInner {
        let mut data = Vec::with_capacity(INITIAL_CAPACITY);
        data.push(2);
        data.push(3);
        PrimeInner { data: data }
    }

    #[inline]
    fn with_capacity(capacity: uint) -> PrimeInner {
        let mut data = Vec::with_capacity(capacity + SMALL_PRIMES.len());
        data.push_all(SMALL_PRIMES);
        PrimeInner { data: data }
    }

    #[inline]
    fn max_prime(&self) -> u64 { *self.data.last().unwrap() }

    #[inline]
    fn nth(&mut self, n: uint) -> u64 { self.grow(n + 1); self.data[n] }

    #[inline]
    fn contains(&mut self, n: &u64) -> bool {
        if *n < self.max_prime() {
            return self.data.as_slice().binary_search_elem(n).found().is_some()
        }

        if !self.is_coprime(*n) { return false }

        iter::count(self.data.len(), 1)
            .map(|i| self.nth(i))
            .take_while(|&p| p * p <= *n)
            .all(|p| !n.is_multiple_of(&p))
    }

    #[inline]
    fn is_coprime(&self, n: u64) -> bool {
        self.data.iter()
            .take_while(|& &p| p * p <= n)
            .all(|&p| !n.is_multiple_of(&p))
    }

    #[inline]
    fn grow(&mut self, len: uint) {
        if self.data.len() >= len { return }

        for n in iter::count(self.max_prime() + 2, 2) {
            if self.is_coprime(n) { self.data.push(n); }
            if self.data.len() >= len { return; }
        }
    }
}

/// Prime number set
#[deriving(Clone)]
pub struct PrimeSet {
    data: Rc<RefCell<PrimeInner>>
}

impl PrimeSet {
    /// Create a new prime number generator.
    #[inline]
    pub fn new() -> PrimeSet { PrimeSet::from_inner(PrimeInner::new()) }

    /// Create a new prime number generator with empty buffers.
    #[inline]
    pub fn new_empty() -> PrimeSet { PrimeSet::from_inner(PrimeInner::new_empty()) }

    /// Create a new prime number generator with specifying buffer capacity.
    #[inline]
    pub fn with_capacity(capacity: uint) -> PrimeSet {
        PrimeSet::from_inner(PrimeInner::with_capacity(capacity))
    }

    /// Get nth prime.
    ///
    /// # Example
    ///
    /// ```
    /// use prime::PrimeSet;
    /// let ps = PrimeSet::new();
    /// assert_eq!(2, ps.nth(0));
    /// assert_eq!(3, ps.nth(1));
    /// assert_eq!(5, ps.nth(2));
    /// assert_eq!(743, ps.nth(131));
    /// ```
    #[inline]
    pub fn nth(&self, n: uint) -> u64 { self.data.borrow_mut().nth(n) }

    /// An iterator visiting all prime numbers in ascending order.
    ///
    /// # Example
    ///
    /// ```
    /// use prime::PrimeSet;
    /// let mut it = PrimeSet::new().iter();
    /// assert_eq!(Some(2), it.next());
    /// assert_eq!(Some(3), it.next());
    /// assert_eq!(Some(5), it.next());
    /// assert_eq!(Some(7), it.next());
    /// ```
    #[inline]
    pub fn iter<'a>(&'a self) -> Nums {
        Nums { idx: 0, data: self.data.clone() }
    }

    fn from_inner(inner: PrimeInner) -> PrimeSet {
        PrimeSet { data: Rc::new(RefCell::new(inner)) }
    }
}

impl Collection for PrimeSet {
    fn len(&self) -> uint { uint::MAX }
}

impl Set<u64> for PrimeSet {
    #[inline]
    fn contains(&self, n: &u64) -> bool { self.data.borrow_mut().contains(n) }

    #[inline]
    fn is_disjoint(&self, _: &PrimeSet) -> bool { false }

    #[inline]
    fn is_subset(&self, _: &PrimeSet) -> bool { true }
}

/// Prime number iterator
pub struct Nums {
    idx: uint,
    data: Rc<RefCell<PrimeInner>>
}

impl Iterator<u64> for Nums {
    #[inline]
    fn next(&mut self) -> Option<u64> {
        let p = self.data.borrow_mut().nth(self.idx);
        self.idx += 1;
        Some(p)
    }
}

impl RandomAccessIterator<u64> for Nums {
    #[inline]
    fn indexable(&self) -> uint { uint::MAX }

    #[inline]
    fn idx(&mut self, index: uint) -> Option<u64> {
        Some(self.data.borrow_mut().nth(index))
    }
}

pub type Factor<T> = (T, int);

/// Numbers which can be factorized.
pub trait Factorize {
    /// An iterator visiting all factors in ascending order.
    fn factorize(&self, ps: &PrimeSet) -> Factors<Self>;
}

macro_rules! trait_impl_unsigned(
    ($($t:ty)*) => ($(
        impl Factorize for $t {
            #[inline]
            fn factorize(&self, ps: &PrimeSet) -> Factors<$t> {
                Factors { num: *self, iter: ps.iter() }
            }
        }
    )*)
)
macro_rules! trait_impl_signed(
    ($($t:ty)*) => ($(
        impl Factorize for $t {
            #[inline]
            fn factorize(&self, ps: &PrimeSet) -> Factors<$t> {
                if *self < 0 {
                    Factors { num: -*self, iter: ps.iter() }
                } else {
                    Factors { num: *self, iter: ps.iter() }
                }
            }
        }
    )*)
)
trait_impl_unsigned!(uint u8 u16 u32 u64)
trait_impl_signed!(int i8 i16 i32 i64)

/// Factors iterator.
pub struct Factors<T> {
    num: T,
    iter: Nums
}

impl<T: Integer + FromPrimitive> Iterator<Factor<T>> for Factors<T> {
    #[inline]
    fn next(&mut self) -> Option<Factor<T>> {
        if self.num <= One::one() { return None }

        for p in self.iter {
            let p: T = FromPrimitive::from_u64(p).unwrap();
            if p * p > self.num {
                let n = mem::replace(&mut self.num, One::one());
                return Some((n, 1))
            }

            if self.num.is_multiple_of(&p) {
                let mut exp = 1;
                self.num = self.num / p;
                while self.num.is_multiple_of(&p) {
                    exp += 1;
                    self.num = self.num / p;
                }
                return Some((p, exp))
            }
        }

        unreachable!()
    }
}


#[cfg(test)]
mod tests {
    use super::{PrimeSet, Factor, Factorize};

    #[test]
    fn iter() {
        let p1 = PrimeSet::new_empty();
        assert_eq!(super::SMALL_PRIMES,
                   p1.iter().take(super::SMALL_PRIMES.len()).collect::<Vec<_>>().as_slice())
    }

    #[test]
    fn contains() {
        let ps = PrimeSet::new();
        assert!(!ps.contains(&0));
        assert!(!ps.contains(&1));
        assert!(ps.contains(&2));
        assert!(ps.contains(&3));
        assert!(!ps.contains(&4));
        assert!(ps.contains(&5));
        assert!(!ps.contains(&6));
        assert!(ps.contains(&7));
        assert!(!ps.contains(&100));
    }

    #[test]
    fn multi_iter() {
        let ps = PrimeSet::new();
        for _p1 in ps.iter() {
            for _p2 in ps.iter() {
                break;
            }
            break;
        }
    }

    #[test]
    fn clone_clones_data() {
        let p1 = PrimeSet::new_empty();
        let p2 = p1.clone();
        let _ = p1.nth(5000);
        let l1 = p1.data.borrow().data.len();
        let l2 = p2.data.borrow().data.len();
        assert_eq!(l1, l2);
    }

    #[test]
    fn factorize() {
        fn check(n: uint, fs: &[Factor<uint>]) {
            let ps = PrimeSet::new();
            assert_eq!(fs, n.factorize(&ps).collect::<Vec<_>>().as_slice());
        }

        check(0, []);
        check(1, []);
        check(2, [(2, 1)]);
        check(3, [(3, 1)]);
        check(4, [(2, 2)]);
        check(5, [(5, 1)]);
        check(6, [(2, 1), (3, 1)]);
        check(7, [(7, 1)]);
        check(8, [(2, 3)]);
        check(9, [(3, 2)]);
        check(10, [(2, 1), (5, 1)]);

        check(8 * 27, [(2, 3), (3, 3)]);
        check(97, [(97, 1)]);
        check(97 * 41, [(41, 1), (97, 1)]);
    }
}

#[cfg(test)]
mod bench {
    use super::PrimeSet;
    use test::Bencher;

    #[bench]
    fn get_5000th(bh: &mut Bencher) {
        bh.iter(|| { PrimeSet::new().nth(5000) });
    }

    #[bench]
    fn get_below_5000th(bh: &mut Bencher) {
        bh.iter(|| {
                let ps = PrimeSet::new();
                for _p in ps.iter().take(5000) {}
            });
    }

}
