use core::iterator::{ Iterator, IteratorUtil };
use core::util::unreachable;

use common::extiter::{ Range };
use common::extvec;
use common::calc::{ pow };
use common::monoid::{ Sum, MergeMonoidIterator, MergeMultiMonoidIterator, Wrap };

pub struct Prime<'self> {
    priv vec: ~[uint],
}

impl<'self> Prime {
    #[inline(always)]
    pub fn new() -> Prime { Prime { vec: ~[] } }

    #[inline(always)]
    priv fn is_coprime(&mut self, num: uint) -> bool {
        for self.vec.each |&p| {
            if p * p > num  { return true; }
            if num % p == 0 { return false; }
        }
        return true;
    }

    #[inline(always)]
    priv fn grow_len(&mut self, len: uint) {
        if self.vec.len() >= len { return; }

        let mut num;
        if self.vec.is_empty() {
            self.vec.push(2);
            if self.vec.len() >= len { return; }
            num = 2;
        } else {
            num = self.vec.last().clone();
        }

        if num == 2 {
            self.vec.push(3);
            if self.vec.len() >= len { return; }
            num = 3;
        }

        while self.vec.len() < len {
            num += 2;
            if self.is_coprime(num) {
                self.vec.push(num);
            }
        }
    }

    #[inline(always)]
    pub fn get_at(&mut self, idx: uint) -> uint {
        self.grow_len(idx + 1);
        return self.vec[idx];
    }

    #[inline(always)]
    pub fn is_prime(&mut self, num: uint) -> bool {
        if num < 2 { return false; }

        for self.each |p| {
            if p * p > num  { return true;  }
            if num % p == 0 { return false; }
        }
        unreachable();
    }

    #[inline(always)]
    pub fn iter(&'self mut self) -> PrimeIterator<'self> {
        PrimeIterator::new(self)
    }

    #[inline(always)]
    pub fn factorize(&'self mut self, n: uint) -> FactorIterator<'self> {
        FactorIterator::new(n, self)
    }

    #[inline(always)]
    pub fn each(&mut self, f: &fn(uint) -> bool) {
        for self.each_borrow |p, _ps| {
            if !f(p) {
                return;
            }
        }
        unreachable();
    }

    #[inline(always)]
    pub fn each_borrow(&mut self, f: &fn(uint, &mut Prime) -> bool) {
        let init_len = self.vec.len();
        for uint::range(0, init_len) |i| {
            let p = self.vec[i];
            if !f(p, self) { return; }
        }

        let mut idx = init_len;
        loop {
            let p = self.get_at(idx);
            if !f(p, self) { return; }
            idx += 1;
        }
    }
}

priv struct PrimeIterator<'self> {
    prime: &'self mut Prime,
    idx: uint
}

impl<'self> PrimeIterator<'self> {
    #[inline(always)]
    pub fn new<'a>(ps: &'a mut Prime) -> PrimeIterator<'a> {
        PrimeIterator { prime: ps, idx: 0 }
    }
}

impl<'self> Iterator<uint> for PrimeIterator<'self> {
    #[inline(always)]
    fn next(&mut self) -> Option<uint> {
        let p = self.prime.get_at(self.idx);
        self.idx += 1;
        return Some(p);
    }
}

pub type Factor = (uint, int);

priv struct FactorIterator<'self> {
    priv num: uint,
    priv primeIter: PrimeIterator<'self>
}

impl<'self> FactorIterator<'self> {
    #[inline(always)]
    pub fn new<'a>(num: uint, primes: &'a mut Prime) -> FactorIterator<'a> {
        FactorIterator { num: num, primeIter: primes.iter() }
    }
}

impl<'self> Iterator<Factor> for FactorIterator<'self> {
    fn next(&mut self) -> Option<Factor> {
        if self.num == 0 || self.num == 1 { return None; }

        while self.num > 1 {
            let p = self.primeIter.next().get();

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

#[inline(always)]
pub fn factors_to_uint<IA: Iterator<Factor>>(mut fs: IA) -> uint {
    let mut result = 1;
    for fs.advance |(base, exp)| {
        if exp > 0 {
            result *= pow(base, exp as uint);
        } else {
            result /= pow(base, (-exp) as uint);
        }
    }
    return result;
}

#[inline(always)]
pub fn comb(n: uint, r: uint, ps: &mut Prime) -> uint {
    let factorize = |n| ps.factorize(n);

    let numer = MergeMultiMonoidIterator::new(extvec::from_iter(
        Range::new(r + 1, n + 1).transform(factorize)
        .transform(|fs| fs.transform(|(base, exp)| (base, Sum(exp))))
    ));

    let denom = MergeMultiMonoidIterator::new(extvec::from_iter(
        Range::new(1, n - r + 1).transform(factorize)
        .transform(|fs| fs.transform(|(base, exp)| (base, Sum(-exp))))
    ));

    return factors_to_uint(
        MergeMonoidIterator::new(numer, denom).transform(|(a, m)| (a, m.unwrap()))
    );
}

#[inline(always)]
pub fn num_of_divisors(num: uint, ps: &mut Prime) -> uint {
    if num == 0 { return 0; }
    let mut prod = 1;
    let mut it = ps.factorize(num);
    for it.advance |(_base, exp)| {
        prod *= (exp as uint) + 1;
    }
    return prod;
}

#[inline(always)]
pub fn num_of_proper_divisors(num: uint, ps: &mut Prime) -> uint {
    num_of_proper_divisors(num, ps) - 1
}

#[inline(always)]
pub fn sum_of_divisors(num: uint, ps: &mut Prime) -> uint {
    if num == 0 { return 0; }
    let mut sum = 1;
    let mut it = ps.factorize(num);
    for it.advance |(base, exp)| {
        sum *= (pow(base, (exp as uint) + 1) - 1) / (base - 1);
    }
    return sum;
}

#[inline(always)]
pub fn sum_of_proper_divisors(num: uint, ps: &mut Prime) -> uint {
    sum_of_divisors(num, ps) - num
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::iterator::{ IteratorUtil };

    static PRIMES_BELOW100: &'static [uint] = &[
        2,  3,  5,  7, 11, 13, 17, 19, 23, 29, 31, 37, 41,
        43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97
    ];

    #[test]
    fn test_prime_opidx () {
        let mut ps = Prime::new();

        // Generated primes
        for PRIMES_BELOW100.eachi() |i, &p| { assert_eq!(ps.get_at(i), p) }
        // Memoized primes
        for PRIMES_BELOW100.eachi() |i, &p| { assert_eq!(ps.get_at(i), p) }
    }

    #[test]
    fn test_prime_each() {
        let table = PRIMES_BELOW100.slice(0, 13);
        let table2 = PRIMES_BELOW100.slice(13, PRIMES_BELOW100.len());
        let mut ps = Prime::new();

        let mut v1 = ~[];
        for ps.each |p| {
            if p > *table.last() { break; }
            v1 += [ p ];
        }
        assert_eq!(table.initn(0), v1.initn(0));

        let mut v2 = ~[];
        for ps.each |p| {
            if p > *table.last() { break; }
            v2 += [ p ];
        }
        assert_eq!(table.initn(0), v2.initn(0));

        let mut v3 = ~[];
        for ps.each |p| {
            if p > *table2.last() { break; }
            v3 += [ p ];
        }
        assert_eq!(~[] + table + table2, v3);
    }

    #[test]
    fn test_prime_is_prime() {
        let mut p = Prime::new();
        assert!(!p.is_prime(0));
        assert!(!p.is_prime(1));
        assert!(p.is_prime(2));
        assert!(p.is_prime(3));
        assert!(!p.is_prime(4));
        assert!(p.is_prime(5));
        assert!(!p.is_prime(6));
        assert!(p.is_prime(7));
        assert!(!p.is_prime(100));
    }

    #[test]
    fn test_prime_iterator() {
        let mut p = Prime::new();
        let mut it = p.iter();

        let mut i = 0;
        let ys = &[ 2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41 ];
        for it.advance |x| {
            if i >= ys.len() { break; }
            assert_eq!(x, ys[i]);
            i += 1;
        }
    }

    #[test]
    fn test_factor_iterator() {
        fn check(n: uint, fs: &[Factor]) {
            let mut ps = Prime::new();
            let mut v = ~[];
            let mut it = ps.factorize(n);
            for it.advance |f| { v.push(f); }

            assert_eq!(v.initn(0), fs);
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
    fn test_factors_to_uint() {
        fn check(n: uint, fs: &[Factor]) {
            assert_eq!(factors_to_uint(fs.iter().transform(|n| *n)), n);
        }

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

        check(1, &[(1, 1)]);
    }

    #[test]
    fn test_comb() {
        let mut ps = Prime::new();
        assert_eq!(comb(2, 2, &mut ps), 1);
        assert_eq!(comb(3, 2, &mut ps), 3);
        assert_eq!(comb(4, 2, &mut ps), 6);
        assert_eq!(comb(5, 2, &mut ps), 10);

        assert_eq!(comb(40, 20, &mut ps), 137846528820);
    }

    #[test]
    fn test_num_of_divisors() {
        let mut ps = Prime::new();

        assert_eq!(num_of_divisors(1, &mut ps), 1);
        assert_eq!(num_of_divisors(2, &mut ps), 2);
        assert_eq!(num_of_divisors(3, &mut ps), 2);
        assert_eq!(num_of_divisors(4, &mut ps), 3);
        assert_eq!(num_of_divisors(5, &mut ps), 2);
        assert_eq!(num_of_divisors(6, &mut ps), 4);
        assert_eq!(num_of_divisors(7, &mut ps), 2);
        assert_eq!(num_of_divisors(8, &mut ps), 4);
        assert_eq!(num_of_divisors(9, &mut ps), 3);
        assert_eq!(num_of_divisors(10, &mut ps), 4);
        assert_eq!(num_of_divisors(11, &mut ps), 2);
        assert_eq!(num_of_divisors(12, &mut ps), 6);

        assert_eq!(num_of_divisors(24, &mut ps), 8);
        assert_eq!(num_of_divisors(36, &mut ps), 9);
        assert_eq!(num_of_divisors(48, &mut ps), 10);
        assert_eq!(num_of_divisors(60, &mut ps), 12);

        assert_eq!(num_of_divisors(50, &mut ps), 6);
    }

    #[test]
    fn test_sum_of_divisors() {
        let mut ps = Prime::new();

        assert_eq!(sum_of_divisors(1, &mut ps), 1);
        assert_eq!(sum_of_divisors(2, &mut ps), 3);
        assert_eq!(sum_of_divisors(3, &mut ps), 4);
        assert_eq!(sum_of_divisors(4, &mut ps), 7);
        assert_eq!(sum_of_divisors(5, &mut ps), 6);
        assert_eq!(sum_of_divisors(6, &mut ps), 12);
        assert_eq!(sum_of_divisors(7, &mut ps), 8);
        assert_eq!(sum_of_divisors(8, &mut ps), 15);
        assert_eq!(sum_of_divisors(9, &mut ps), 13);
        assert_eq!(sum_of_divisors(10, &mut ps), 18);
        assert_eq!(sum_of_divisors(11, &mut ps), 12);
        assert_eq!(sum_of_divisors(12, &mut ps), 28);

        assert_eq!(sum_of_divisors(24, &mut ps), 60);
        assert_eq!(sum_of_divisors(36, &mut ps), 91);
        assert_eq!(sum_of_divisors(48, &mut ps), 124);
        assert_eq!(sum_of_divisors(60, &mut ps), 168);

        assert_eq!(sum_of_divisors(50, &mut ps), 93);
    }
}
