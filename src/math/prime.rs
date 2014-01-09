use std::{iter, local_data};
use std::iter::MultiplicativeIterator;
use std::cell::RefCell;
use std::rc::Rc;
use std::local_data::Key;

use arith;
use data::monoid::{Sum, MergeMonoidIterator, MergeMultiMonoidIterator, Wrap};

static PRIMES_BELOW100: &'static [uint] = &[
    2,  3,  5,  7, 11, 13, 17, 19, 23, 29, 31, 37, 41,
    43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97
];

struct PrimeInner {
    priv data: ~[uint]
}

impl PrimeInner {
    fn new() -> PrimeInner { PrimeInner { data: PRIMES_BELOW100.to_owned() } }
    fn nth(&mut self, n: uint) -> uint {
        self.grow(n + 1);
        return self.data[n];
    }
    fn contains(&mut self, n: uint) -> bool {
        if n < *self.data.last() {
            self.data.bsearch_elem(&n).is_some()
        } else {
            iter::count(0u, 1)
                .map(|i| self.nth(i))
                .take_while(|&p| p * p <= n)
                .all(|p| n % p != 0)
        }
    }

    #[inline]
    fn is_coprime(&self, n: uint) -> bool {
        self.data.iter()
            .take_while(|& &p| p * p <= n)
            .all(|&p| n % p != 0)

    }

    #[inline]
    fn grow(&mut self, len: uint) {
        if self.data.len() >= len { return; }
        let mut it = iter::count(self.data.last() + 2, 2)
            .filter(|&n| self.is_coprime(n));
        for n in it {
            self.data.push(n);
            if self.data.len() > len { return; }
        }
    }
}

static TASK_PRIME_KEY: Key<Prime> = &Key;

#[deriving(Clone)]
pub struct Prime {
    priv data: Rc<RefCell<PrimeInner>>
}

impl Prime {
    #[inline]
    pub fn new() -> Prime {
        let prime = local_data::get_mut(TASK_PRIME_KEY, |val| match val {
                Some(prime) => Some(prime.clone()),
                None        => None
            });

        match prime {
            Some(prime) => prime,
            None => {
                let prime = Prime::new_empty();
                local_data::set(TASK_PRIME_KEY, prime.clone());
                prime
            }
        }
    }
    #[inline]
    fn new_empty() -> Prime { Prime { data: Rc::from_send(RefCell::new(PrimeInner::new())) } }
    #[inline]
    pub fn nth(&self, n: uint) -> uint { self.data.borrow().with_mut(|p| p.nth(n)) }
    #[inline]
    pub fn contains(&self, n: uint) -> bool { self.data.borrow().with_mut(|p| p.contains(n)) }

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
            .map(|(base, exp)| (arith::pow(base, (exp as uint) + 1) - 1) / (base - 1) )
            .product()
    }
    #[inline]
    pub fn num_of_proper_divisor(&self, n: uint) -> uint { self.num_of_divisor(n) - 1 }
    #[inline]
    pub fn sum_of_proper_divisor(&self, n: uint) -> uint { self.sum_of_divisor(n) - n }
    #[inline]
    pub fn comb(&self, n: uint, r: uint) -> uint {
        let ns = range(r + 1, n + 1)
            .map(|n| {
                self.factorize(n)
                    .map(|(base, exp)| (base, Sum(exp)))
            }).to_owned_vec();
        let numer = MergeMultiMonoidIterator::new(ns);

        let ds = range(1, n - r + 1)
            .map(|n| {
                self.factorize(n)
                    .map(|(base, exp)| (base, Sum(-exp)))
            }).to_owned_vec();
        let denom = MergeMultiMonoidIterator::new(ds);

        MergeMonoidIterator::new(numer, denom).map(|(a, m)| (a, m.unwrap())).to_uint()
    }
}

pub struct PrimeIterator {
    priv idx: uint,
    priv data: Rc<RefCell<PrimeInner>>
}

impl Iterator<uint> for PrimeIterator {
    #[inline]
    fn next(&mut self) -> Option<uint> {
        let p = self.data.borrow().with_mut(|p| p.nth(self.idx));
        self.idx += 1;
        Some(p)
    }
}

pub type Factor = (uint, int);

pub struct FactorizeIterator {
    priv num: uint,
    priv iter: PrimeIterator
}

impl Iterator<Factor> for FactorizeIterator {
    #[inline]
    fn next(&mut self) -> Option<Factor> {
        if self.num == 0 || self.num == 1 { return None; }

        while self.num > 1 {
            let p = self.iter.next().unwrap();

            if p * p > self.num {
                let n = self.num;
                self.num = 1;
                return Some((n, 1));
            }

            let mut exp = 0;
            while self.num % p == 0 {
                exp += 1;
                self.num /= p;
            }
            if exp > 0 { return Some((p, exp)); }
        }

        return None;
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
                    s * arith::pow(base, exp as uint)
                } else {
                    s / arith::pow(base, (-exp) as uint)
                }
            })
    }
}

#[cfg(test)]
mod test {
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
        assert_eq!(PRIMES_BELOW200.to_owned(), prime.iter().take(PRIMES_BELOW200.len()).to_owned_vec());
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
        let l1 = p1.data.borrow().with(|p| p.data.len());
        let l2 = p2.data.borrow().with(|p| p.data.len());
        assert_eq!(l1, l2);
    }

    #[test]
    fn factorize() {
        fn check(n: uint, fs: ~[Factor]) {
            let ps = Prime::new();
            assert_eq!(fs, ps.factorize(n).to_owned_vec());
            if n != 0 {
                assert_eq!(n, ps.factorize(n).to_uint());
            }
        }

        check(0, ~[]);
        check(1, ~[]);
        check(2, ~[(2, 1)]);
        check(3, ~[(3, 1)]);
        check(4, ~[(2, 2)]);
        check(5, ~[(5, 1)]);
        check(6, ~[(2, 1), (3, 1)]);
        check(7, ~[(7, 1)]);
        check(8, ~[(2, 3)]);
        check(9, ~[(3, 2)]);
        check(10, ~[(2, 1), (5, 1)]);

        check(8 * 27, ~[(2, 3), (3, 3)]);
        check(97, ~[(97, 1)]);
        check(97 * 41, ~[(41, 1), (97, 1)]);
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
    use extra::test::BenchHarness;

    #[bench]
    fn get_5000th(bh: &mut BenchHarness) {
        bh.iter(|| { Prime::new().nth(5000); });
    }
    #[bench]
    fn get_5000th_nocache(bh: &mut BenchHarness) {
        bh.iter(|| { Prime::new_empty().nth(5000); });
    }

    #[bench]
    fn get_below_5000th(bh: &mut BenchHarness) {
        bh.iter(|| {
                let prime = Prime::new();
                for _p in prime.iter().take(5000) {}
            });
    }
    #[bench]
    fn get_below_5000th_nocache(bh: &mut BenchHarness) {
        bh.iter(|| {
                let prime = Prime::new_empty();
                for _p in prime.iter().take(5000) {}
            });
    }

    #[bench]
    fn factorial_600851475143(bh: &mut BenchHarness) {
        bh.iter(|| {
                let ps = Prime::new();
                ps.factorize(600851475143).fold(0, |a, (b, _)| a + b);
            });
    }

    #[bench]
    fn factorial_600851475143_nocache(bh: &mut BenchHarness) {
        bh.iter(|| {
                let ps = Prime::new_empty();
                ps.factorize(600851475143).fold(0, |a, (b, _)| a + b);
            });
    }
}
