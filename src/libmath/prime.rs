use std::{iter, num, uint};
use std::iter::MultiplicativeIterator;
use std::cell::RefCell;
use std::rc::Rc;
use std::local_data::Key;
use std::collections::HashMap;

use num::Integer;

static PRIMES_BELOW100: &'static [uint] = &[
    2,  3,  5,  7, 11, 13, 17, 19, 23, 29, 31, 37, 41,
    43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97
];

struct PrimeInner {
    data: Vec<uint>
}

impl PrimeInner {
    #[inline]
    fn new() -> PrimeInner { PrimeInner { data: Vec::from_slice(PRIMES_BELOW100) } }

    #[inline]
    fn max_prime(&self) -> uint { *self.data.last().unwrap() }

    #[inline]
    fn nth(&mut self, n: uint) -> uint { self.grow(n + 1); *self.data.get(n) }

    #[inline]
    fn contains(&mut self, n: uint) -> bool {
        if n < self.max_prime() { return self.data.as_slice().bsearch_elem(&n).is_some() }

        if !self.is_coprime(n) { return false }

        iter::count(self.data.len(), 1)
            .map(|i| self.nth(i))
            .take_while(|&p| p * p <= n)
            .all(|p| !n.divides(&p))
    }

    #[inline]
    fn is_coprime(&self, n: uint) -> bool {
        self.data.iter()
            .take_while(|& &p| p * p <= n)
            .all(|&p| !n.divides(&p))
    }

    #[inline]
    fn grow(&mut self, len: uint) {
        if self.data.len() >= len { return }

        for n in iter::count(self.max_prime() + 2, 2) {
            if self.is_coprime(n) { self.data.push(n); }
            if self.data.len() >= len { return }
        }
    }
}

static TASK_PRIME_KEY: Key<Prime> = &Key;

#[deriving(Clone)]
pub struct Prime {
    data: Rc<RefCell<PrimeInner>>
}

impl Prime {
    #[inline]
    pub fn new() -> Prime {
        match TASK_PRIME_KEY.get() {
            Some(prime) => prime.clone(),
            None => {
                let prime = Prime::new_empty();
                TASK_PRIME_KEY.replace(Some(prime.clone()));
                prime
            }
        }
    }
    #[inline]
    fn new_empty() -> Prime { Prime { data: Rc::new(RefCell::new(PrimeInner::new())) } }
    #[inline]
    pub fn nth(&self, n: uint) -> uint { self.data.borrow_mut().nth(n) }
    #[inline]
    pub fn contains(&self, n: uint) -> bool { self.data.borrow_mut().contains(n) }

    #[inline]
    pub fn iter<'a>(&'a self) -> PrimeIterator { PrimeIterator { idx: 0, data: self.data.clone() } }
    #[inline]
    pub fn factorize(&self, n: uint) -> FactorizeIterator {
        FactorizeIterator { num: n, iter: self.iter() }
    }
    #[inline]
    pub fn num_of_divisor(&self, n: uint) -> uint {
        if n == 0 { return 0 }
        self.factorize(n)
            .map(|(_base, exp)| (exp as uint) + 1)
            .product()
    }
    #[inline]
    pub fn sum_of_divisor(&self, n: uint) -> uint {
        if n == 0 { return 0 }
        self.factorize(n)
            .map(|(base, exp)| (num::pow(base, (exp as uint) + 1) - 1) / (base - 1) )
            .product()
    }
    #[inline]
    pub fn num_of_proper_divisor(&self, n: uint) -> uint { self.num_of_divisor(n) - 1 }
    #[inline]
    pub fn sum_of_proper_divisor(&self, n: uint) -> uint { self.sum_of_divisor(n) - n }

    #[inline]
    pub fn comb(&self, n: uint, r: uint) -> uint {
        let mut map = HashMap::new();
        for n in range(r + 1, n + 1) {
            for (b, e) in self.factorize(n) {
                *map.find_or_insert(b, 0) += e;
            }
        }
        for n in range(1, n - r + 1) {
            for (b, e) in self.factorize(n) {
                *map.get_mut(&b) -= e;
            }
        }
        map.move_iter().to_uint()
    }
}

pub struct PrimeIterator {
    idx: uint,
    data: Rc<RefCell<PrimeInner>>
}

impl Iterator<uint> for PrimeIterator {
    #[inline]
    fn next(&mut self) -> Option<uint> {
        let p = self.data.borrow_mut().nth(self.idx);
        self.idx += 1;
        Some(p)
    }
}

impl RandomAccessIterator<uint> for PrimeIterator {
    #[inline]
    fn indexable(&self) -> uint { uint::MAX }

    #[inline]
    fn idx(&mut self, index: uint) -> Option<uint> {
        Some(self.data.borrow_mut().nth(index))
    }
}

pub type Factor = (uint, int);

pub struct FactorizeIterator {
    num: uint,
    iter: PrimeIterator
}

impl Iterator<Factor> for FactorizeIterator {
    #[inline]
    fn next(&mut self) -> Option<Factor> {
        if self.num <= 1 { return None }

        for p in self.iter {
            if p * p > self.num {
                let n = self.num;
                self.num = 1;
                return Some((n, 1))
            }

            if self.num.divides(&p) {
                let mut exp = 1;
                self.num /= p;

                while self.num.divides(&p) {
                    exp += 1;
                    self.num /= p;
                }
                return Some((p, exp))
            }
        }

        unreachable!()
    }
}

pub trait FactorIterator {
    fn to_uint(&mut self) -> uint;
}

impl<IA: Iterator<Factor>> FactorIterator for IA {
    #[inline]
    fn to_uint(&mut self) -> uint {
        self.fold(1, |s, (base, exp)| {
                if exp > 0 {
                    s * num::pow(base, exp as uint)
                } else {
                    s / num::pow(base, (-exp) as uint)
                }
            })
    }
}

#[cfg(test)]
mod tests {
    use super::{Prime, Factor, FactorIterator};

    static PRIMES_BELOW200: &'static [uint] = &[
        2,  3,  5,  7, 11, 13, 17, 19, 23, 29, 31, 37, 41,
        43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97,
        101, 103, 107, 109, 113, 127, 131, 137, 139, 149,
        151, 157, 163, 167, 173, 179, 181, 191, 193, 197, 199
            ];

    #[test]
    fn iter() {
        let prime = Prime::new();
        assert_eq!(PRIMES_BELOW200,
                   prime.iter().take(PRIMES_BELOW200.len()).collect::<Vec<_>>().as_slice());
    }

    #[test]
    fn nth() {
        let prime = Prime::new_empty();
        // Generated primes
        for (i, &p) in PRIMES_BELOW200.iter().enumerate() {
            assert_eq!(p, prime.nth(i));
        }
        // Memoized primes
        for (i, &p) in PRIMES_BELOW200.iter().enumerate() {
            assert_eq!(p, prime.nth(i));
        }
    }

    #[test]
    fn contains() {
        let prime = Prime::new();
        assert!(!prime.contains(0));
        assert!(!prime.contains(1));
        assert!(prime.contains(2));
        assert!(prime.contains(3));
        assert!(!prime.contains(4));
        assert!(prime.contains(5));
        assert!(!prime.contains(6));
        assert!(prime.contains(7));
        assert!(!prime.contains(100));
    }

    #[test]
    fn multi_iter() {
        let prime = Prime::new();
        for _p1 in prime.iter() {
            for _p2 in prime.iter() {
                break;
            }
            break;
        }
    }

    #[test]
    fn clone_clones_data() {
        let p1 = Prime::new();
        let p2 = p1.clone();
        p1.nth(500);
        let l1 = p1.data.borrow().data.len();
        let l2 = p2.data.borrow().data.len();
        assert_eq!(l1, l2);
    }

    #[test]
    fn new_clones_data() {
        let p1 = Prime::new();
        let p2 = Prime::new();
        p1.nth(500);
        let l1 = p1.data.borrow().data.len();
        let l2 = p2.data.borrow().data.len();
        assert_eq!(l1, l2);
    }

    #[test]
    fn factorize() {
        fn check(n: uint, fs: &[Factor]) {
            let ps = Prime::new();
            assert_eq!(fs, ps.factorize(n).collect::<Vec<_>>().as_slice());
            if n != 0 {
                assert_eq!(n, ps.factorize(n).to_uint());
            }
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

    #[test]
    fn num_of_divisor() {
        let pairs = [
            (0, 0),
            (1, 1), (2, 2), (3, 2), (4, 3), (5, 2), (6, 4),
            (7, 2), (8, 4), (9, 3), (10, 4), (11, 2), (12, 6),
            (24, 8), (36, 9), (48, 10), (60, 12),
            (50, 6)
            ];
        let p = Prime::new();
        for &(n, num_div) in pairs.iter() {
            assert_eq!(num_div, p.num_of_divisor(n));
        }
    }

    #[test]
    fn sum_of_divisor() {
        let pairs = [
            (0, 0),
            (1, 1), (2, 3), (3, 4), (4, 7), (5, 6), (6, 12),
            (7, 8), (8, 15), (9, 13), (10, 18), (11, 12), (12, 28),
            (24, 60), (36, 91), (48, 124), (60, 168),
            (50, 93)
            ];
        let p = Prime::new();
        for &(n, sum_div) in pairs.iter() {
            assert_eq!(sum_div, p.sum_of_divisor(n));
        }
    }

    #[test]
    fn comb() {
        let prime = Prime::new();
        assert_eq!(1, prime.comb(2, 2));
        assert_eq!(3, prime.comb(3, 2));
        assert_eq!(6, prime.comb(4, 2));
        assert_eq!(10, prime.comb(5, 2));

        assert_eq!(137846528820, prime.comb(40, 20));
    }
}

#[cfg(test)]
mod bench {
    use super::Prime;
    use test::Bencher;

    #[bench]
    fn get_5000th(bh: &mut Bencher) {
        bh.iter(|| { Prime::new().nth(5000); });
    }
    #[bench]
    fn get_5000th_nocache(bh: &mut Bencher) {
        bh.iter(|| { Prime::new_empty().nth(5000); });
    }

    #[bench]
    fn get_below_5000th(bh: &mut Bencher) {
        bh.iter(|| {
                let prime = Prime::new();
                for _p in prime.iter().take(5000) {}
            });
    }
    #[bench]
    fn get_below_5000th_nocache(bh: &mut Bencher) {
        bh.iter(|| {
                let prime = Prime::new_empty();
                for _p in prime.iter().take(5000) {}
            });
    }

    #[bench]
    fn factorial_600851475143(bh: &mut Bencher) {
        bh.iter(|| {
                let ps = Prime::new();
                ps.factorize(600851475143).fold(0, |a, (b, _)| a + b);
            });
    }

    #[bench]
    fn factorial_600851475143_nocache(bh: &mut Bencher) {
        bh.iter(|| {
                let ps = Prime::new_empty();
                ps.factorize(600851475143).fold(0, |a, (b, _)| a + b);
            });
    }
}
