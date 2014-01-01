use std::{iter, local_data};
use std::cell::RefCell;
use std::rc::Rc;
use std::local_data::Key;

use arith;

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
    pub fn iter<'a>(&'a self) -> PrimeIterator { PrimeIterator { idx: 0, data: self.data.clone() } }
    #[inline]
    pub fn nth(&self, n: uint) -> uint { self.data.borrow().with_mut(|p| p.nth(n)) }
    #[inline]
    pub fn contains(&self, n: uint) -> bool { self.data.borrow().with_mut(|p| p.contains(n)) }
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

#[inline(always)]
pub fn factorize(ps: &Prime, n: uint) -> FactorizeIterator {
    FactorizeIterator { num: n, iter: ps.iter() }
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
    mod prime {
        use super::super::Prime;

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
            let prime = Prime::new();
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
    }

    mod factorize {
        use super::super::{Prime, Factor};

        #[test]
        fn factorize() {
            fn check(n: uint, fs: ~[Factor]) {
                let ps = Prime::new();
                assert_eq!(fs, super::super::factorize(&ps, n).to_owned_vec());
                if n != 0 {
                    assert_eq!(n, super::super::factorize(&ps, n).to_uint());
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
                super::factorize(&ps, 600851475143).fold(0, |a, (b, _)| a + b);
            });
    }

    #[bench]
    fn factorial_600851475143_nocache(bh: &mut BenchHarness) {
        bh.iter(|| {
                let ps = Prime::new_empty();
                super::factorize(&ps, 600851475143).fold(0, |a, (b, _)| a + b);
            });
    }
}
