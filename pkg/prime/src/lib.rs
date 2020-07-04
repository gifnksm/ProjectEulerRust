//! Prime number generator and related functions.

#![warn(
    bad_style,
    missing_docs,
    unused,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]
#![cfg_attr(all(test, feature = "unstable"), feature(test))]

#[cfg(all(test, feature = "unstable"))]
extern crate test;

use num_integer::Integer;
use num_traits::{FromPrimitive, One, Zero};
use std::{
    cell::RefCell,
    cmp,
    collections::{
        hash_map::Entry::{Occupied, Vacant},
        HashMap,
    },
    hash::Hash,
    iter::IntoIterator,
    mem,
    rc::Rc,
};

const SMALL_PRIMES: &[u64] = &[
    2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97,
    101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181, 191, 193,
    197, 199, 211, 223, 227, 229, 233, 239, 241, 251, 257, 263, 269, 271, 277, 281, 283, 293, 307,
    311, 313, 317, 331, 337, 347, 349, 353, 359, 367, 373, 379, 383, 389, 397, 401, 409, 419, 421,
    431, 433, 439, 443, 449, 457, 461, 463, 467, 479, 487, 491, 499, 503, 509, 521, 523, 541, 547,
    557, 563, 569, 571, 577, 587, 593, 599, 601, 607, 613, 617, 619, 631, 641, 643, 647, 653, 659,
    661, 673, 677, 683, 691, 701, 709, 719, 727, 733, 739, 743, 751, 757, 761, 769, 773, 787, 797,
    809, 811, 821, 823, 827, 829, 839, 853, 857, 859, 863, 877, 881, 883, 887, 907, 911, 919, 929,
    937, 941, 947, 953, 967, 971, 977, 983, 991, 997,
];

const INITIAL_CAPACITY: usize = 10000;

struct PrimeInner {
    data: Vec<u64>,
}

impl PrimeInner {
    #[inline]
    fn new() -> PrimeInner {
        PrimeInner::with_capacity(INITIAL_CAPACITY)
    }

    #[inline]
    fn new_empty() -> PrimeInner {
        let mut data = Vec::with_capacity(INITIAL_CAPACITY);
        data.push(2);
        data.push(3);
        PrimeInner { data }
    }

    #[inline]
    fn with_capacity(capacity: usize) -> PrimeInner {
        let mut data = Vec::with_capacity(capacity + SMALL_PRIMES.len());
        data.extend(SMALL_PRIMES.iter().cloned());
        PrimeInner { data }
    }

    #[inline]
    fn max_prime(&self) -> u64 {
        *self.data.last().unwrap()
    }

    #[inline]
    fn nth(&mut self, n: usize) -> u64 {
        self.grow(n + 1);
        self.data[n]
    }

    #[inline]
    fn contains(&mut self, n: u64) -> bool {
        if n < self.max_prime() {
            return self.data.binary_search(&n).is_ok();
        }

        if !self.is_coprime(n) {
            return false;
        }

        (self.data.len()..)
            .map(|i| self.nth(i))
            .take_while(|&p| p * p <= n)
            .all(|p| !n.is_multiple_of(&p))
    }

    #[inline]
    fn is_coprime(&self, n: u64) -> bool {
        self.data
            .iter()
            .take_while(|&&p| p * p <= n)
            .all(|&p| !n.is_multiple_of(&p))
    }

    #[inline]
    fn grow(&mut self, len: usize) {
        if self.data.len() >= len {
            return;
        }

        for n in (self.max_prime() + 2..).step_by(2) {
            if self.is_coprime(n) {
                self.data.push(n);
            }
            if self.data.len() >= len {
                return;
            }
        }
    }
}

/// Prime number set
#[derive(Clone)]
pub struct PrimeSet {
    data: Rc<RefCell<PrimeInner>>,
}

impl Default for PrimeSet {
    fn default() -> Self {
        Self::new()
    }
}

impl PrimeSet {
    /// Create a new prime number generator.
    #[inline]
    pub fn new() -> Self {
        Self::from_inner(PrimeInner::new())
    }

    /// Create a new prime number generator with empty buffers.
    #[inline]
    pub fn new_empty() -> Self {
        Self::from_inner(PrimeInner::new_empty())
    }

    /// Create a new prime number generator with specifying buffer capacity.
    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Self::from_inner(PrimeInner::with_capacity(capacity))
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
    pub fn nth(&self, n: usize) -> u64 {
        self.data.borrow_mut().nth(n)
    }

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
    pub fn iter(&self) -> Nums {
        Nums {
            idx: 0,
            data: self.data.clone(),
        }
    }

    /// Return `true` if the given number is prime.
    #[inline]
    pub fn contains(&self, n: u64) -> bool {
        self.data.borrow_mut().contains(n)
    }

    /// Calculates the combination of the number
    #[inline]
    pub fn combination(&self, n: u64, r: u64) -> u64 {
        let mut fac = Factorized::<u64>::new(self);
        for n in (r + 1)..(n + 1) {
            fac.mul_assign(n);
        }
        for n in 1..(n - r + 1) {
            fac.div_assign(n);
        }
        fac.into_integer()
    }

    fn from_inner(inner: PrimeInner) -> PrimeSet {
        PrimeSet {
            data: Rc::new(RefCell::new(inner)),
        }
    }
}

impl<'a> IntoIterator for &'a PrimeSet {
    type Item = u64;
    type IntoIter = Nums;

    fn into_iter(self) -> Nums {
        self.iter()
    }
}

/// Prime number iterator
pub struct Nums {
    idx: usize,
    data: Rc<RefCell<PrimeInner>>,
}

impl Iterator for Nums {
    type Item = u64;

    #[inline]
    fn next(&mut self) -> Option<u64> {
        let p = self.data.borrow_mut().nth(self.idx);
        self.idx += 1;
        Some(p)
    }
}

/// The base and exponent that represents factor.
pub type Factor<T> = (T, i32);

/// Numbers which can be factorized.
pub trait Factorize: Integer + FromPrimitive + Clone {
    /// An iterator visiting all factors in ascending order.
    fn factorize(&self, ps: &PrimeSet) -> Factors<Self>;

    /// Calculates the number of all positive divisors.
    fn num_of_divisor(&self, ps: &PrimeSet) -> u64 {
        if self.is_zero() {
            return Zero::zero();
        }
        self.factorize(ps)
            .map(|(_base, exp)| (exp as u64) + 1)
            .product()
    }

    /// Calculates the sum of all positive divisors.
    fn sum_of_divisor(&self, ps: &PrimeSet) -> Self {
        if self.is_zero() {
            return Zero::zero();
        }
        let one: Self = One::one();
        self.factorize(ps)
            .map(|(base, exp)| {
                let denom = base.clone() - one.clone();
                (num_traits::pow(base, (exp as usize) + 1) - one.clone()) / denom
            })
            .fold(num_traits::one::<Self>(), |acc, n| acc * n)
    }

    /// Calculates the number of proper positive divisors.
    #[inline]
    fn num_of_proper_divisor(&self, ps: &PrimeSet) -> u64 {
        self.num_of_divisor(ps) - 1
    }

    /// Caluculates the sum of all positive divisors.
    #[inline]
    fn sum_of_proper_divisor(&self, ps: &PrimeSet) -> Self {
        self.sum_of_divisor(ps) - self.clone()
    }
}

macro_rules! trait_impl_unsigned {
    ($($t:ty)*) => ($(
        impl Factorize for $t {
            #[inline]
            fn factorize(&self, ps: &PrimeSet) -> Factors<$t> {
                Factors { num: *self, iter: ps.iter() }
            }
        }
    )*)
}
macro_rules! trait_impl_signed {
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
}
trait_impl_unsigned!(usize u8 u16 u32 u64);
trait_impl_signed!(isize i8 i16 i32 i64);

/// Factors iterator.
pub struct Factors<T> {
    num: T,
    iter: Nums,
}

impl<T: Integer + FromPrimitive + Clone> Iterator for Factors<T> {
    type Item = Factor<T>;

    #[inline]
    fn next(&mut self) -> Option<Factor<T>> {
        if self.num <= One::one() {
            return None;
        }

        while let Some(p) = self.iter.next() {
            let p: T = FromPrimitive::from_u64(p).unwrap();
            if p.clone() * p.clone() > self.num {
                let n = mem::replace(&mut self.num, One::one());
                return Some((n, 1));
            }

            if self.num.is_multiple_of(&p) {
                let mut exp = 1;
                self.num = self.num.clone() / p.clone();
                while self.num.is_multiple_of(&p) {
                    exp += 1;
                    self.num = self.num.clone() / p.clone();
                }
                return Some((p, exp));
            }
        }

        unreachable!()
    }
}

/// Factorized number providing multiple or divide operation without causing
/// overflow.
///
/// # Example
///
/// ```
/// use prime::{Factorized, PrimeSet};
/// use std::iter;
///
/// // Calculates 40C20
/// let ps = PrimeSet::new();
/// let mut fac = Factorized::<u64>::new(&ps);
/// for n in 21..41 {
///     fac.mul_assign(n);
/// }
/// for n in 1..21 {
///     fac.div_assign(n);
/// }
/// assert_eq!(137846528820, fac.into_integer());
/// ```
pub struct Factorized<'a, T> {
    ps: &'a PrimeSet,
    map: HashMap<T, i32>,
}

impl<'a, T: Factorize + Eq + Hash> Factorized<'a, T> {
    /// Creates new empty factorized number.
    ///
    /// The empty factorized number represents `1`.
    pub fn new(ps: &PrimeSet) -> Factorized<'_, T> {
        Factorized {
            ps,
            map: HashMap::new(),
        }
    }

    /// Creates a factorized number from an integer type.
    pub fn from_integer(ps: &PrimeSet, n: T) -> Factorized<'_, T> {
        Factorized {
            ps,
            map: n.factorize(ps).collect(),
        }
    }

    /// Converts the factorized number into an integer type.
    pub fn into_integer(self) -> T {
        self.map
            .into_iter()
            .fold::<T, _>(One::one(), |prod, (base, exp)| {
                if exp > 0 {
                    prod * num_traits::pow(base, exp as usize)
                } else {
                    prod / num_traits::pow(base, (-exp) as usize)
                }
            })
    }

    /// Takes LCM (lowest common multiple) with given number and the factorized
    /// number.
    pub fn lcm_with(&mut self, n: T) {
        for (b, e) in n.factorize(self.ps) {
            match self.map.entry(b) {
                Vacant(entry) => {
                    let _ = entry.insert(e);
                }
                Occupied(entry) => {
                    let p = entry.into_mut();
                    *p = cmp::max(e, *p);
                }
            }
        }
    }

    /// Multiples the factorized number and given number.
    pub fn mul_assign(&mut self, n: T) {
        for (b, e) in n.factorize(self.ps) {
            match self.map.entry(b) {
                Vacant(entry) => {
                    let _ = entry.insert(e);
                }
                Occupied(entry) => {
                    *entry.into_mut() += e;
                }
            }
        }
    }

    /// Divides the factorized number by given number.
    pub fn div_assign(&mut self, n: T) {
        for (b, e) in n.factorize(self.ps) {
            match self.map.entry(b) {
                Vacant(entry) => {
                    let _ = entry.insert(-e);
                }
                Occupied(entry) => {
                    *entry.into_mut() -= e;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Factor, Factorize, PrimeSet};

    #[test]
    fn iter() {
        let p1 = PrimeSet::new_empty();
        assert_eq!(
            super::SMALL_PRIMES,
            &p1.iter()
                .take(super::SMALL_PRIMES.len())
                .collect::<Vec<_>>()[..]
        )
    }

    #[test]
    fn contains() {
        let ps = PrimeSet::new();
        assert!(!ps.contains(0));
        assert!(!ps.contains(1));
        assert!(ps.contains(2));
        assert!(ps.contains(3));
        assert!(!ps.contains(4));
        assert!(ps.contains(5));
        assert!(!ps.contains(6));
        assert!(ps.contains(7));
        assert!(!ps.contains(100));
    }

    #[test]
    fn multi_iter() {
        let ps = PrimeSet::new();
        for (p1, p2) in ps.iter().zip(ps.iter()).take(500) {
            assert_eq!(p1, p2);
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
        fn check(n: u32, fs: &[Factor<u32>]) {
            let ps = PrimeSet::new();
            assert_eq!(fs, &n.factorize(&ps).collect::<Vec<_>>()[..]);
        }

        check(0, &[]);
        check(1, &[]);
        check(2, &[(2, 1)]);
        check(3, &[(3, 1)]);
        check(4, &[(2, 2)]);
        check(5, &[(5, 1)]);
        check(6, &[(2, 1), (3, 1)]);
        check(7, &[(7, 1)]);
        check(8, &[(2, 3)]);
        check(9, &[(3, 2)]);
        check(10, &[(2, 1), (5, 1)]);

        check(8 * 27, &[(2, 3), (3, 3)]);
        check(97, &[(97, 1)]);
        check(97 * 41, &[(41, 1), (97, 1)]);
    }

    #[test]
    fn num_of_divisor() {
        let pairs = &[
            (0, 0),
            (1, 1),
            (2, 2),
            (3, 2),
            (4, 3),
            (5, 2),
            (6, 4),
            (7, 2),
            (8, 4),
            (9, 3),
            (10, 4),
            (11, 2),
            (12, 6),
            (24, 8),
            (36, 9),
            (48, 10),
            (60, 12),
            (50, 6),
        ];

        let ps = PrimeSet::new();
        for &(n, num_div) in pairs {
            assert_eq!(num_div, n.num_of_divisor(&ps));
            assert_eq!(num_div, (-n).num_of_divisor(&ps));
        }
    }

    #[test]
    fn sum_of_divisor() {
        let pairs = &[
            (0, 0),
            (1, 1),
            (2, 3),
            (3, 4),
            (4, 7),
            (5, 6),
            (6, 12),
            (7, 8),
            (8, 15),
            (9, 13),
            (10, 18),
            (11, 12),
            (12, 28),
            (24, 60),
            (36, 91),
            (48, 124),
            (60, 168),
            (50, 93),
        ];

        let ps = PrimeSet::new();
        for &(n, sum_div) in pairs {
            assert_eq!(sum_div, n.sum_of_divisor(&ps));
            assert_eq!(sum_div, (-n).sum_of_divisor(&ps));
        }
    }

    #[test]
    fn combination() {
        let ps = PrimeSet::new();
        assert_eq!(1, ps.combination(2, 2));
        assert_eq!(3, ps.combination(3, 2));
        assert_eq!(6, ps.combination(4, 2));
        assert_eq!(10, ps.combination(5, 2));

        assert_eq!(137846528820, ps.combination(40, 20));
    }
}

#[cfg(all(test, feature = "unstable"))]
mod bench {
    use super::PrimeSet;
    use test::Bencher;

    #[bench]
    fn get_5000th(bh: &mut Bencher) {
        bh.iter(|| PrimeSet::new().nth(5000));
    }

    #[bench]
    fn get_below_5000th(bh: &mut Bencher) {
        bh.iter(|| {
            let ps = PrimeSet::new();
            for _p in ps.iter().take(5000) {}
        });
    }
}
