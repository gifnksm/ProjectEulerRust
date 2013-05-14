use core::iterator::{ Iterator, IteratorUtil, Counter };
use core::util;
use core::local_data;

use extiter::{ Range, ExtIteratorUtil, MultiplicativeIterator };
use calc::{ pow };
use monoid::{ Sum, MergeMonoidIterator, MergeMultiMonoidIterator, Wrap };

static PRIMES_BELOW100: &'static [uint] = &[
    2,  3,  5,  7, 11, 13, 17, 19, 23, 29, 31, 37, 41,
    43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97
];

fn tls_prime_nums(_v: @~[uint]) {}

#[inline(always)]
fn get_task_prime() -> @mut ~[uint] {
    fn task_prime_key(_v: @@mut ~[uint]) {}

    let r = unsafe { local_data::local_data_get(task_prime_key) };
    let nums = match r {
        None => {
            let nums = @mut PRIMES_BELOW100.to_owned();
            unsafe { local_data::local_data_set(task_prime_key, @nums); }
            nums
        }
        Some(@nums) => nums
    };

    return nums;
}

#[inline(always)]
fn with_task_prime<T>(f: &fn(&mut ~[uint]) -> T) -> T {
    let nums = get_task_prime();
    return f(nums);
}

#[inline(always)]
priv fn is_coprime(nums: &[uint], n: uint) -> bool {
    for nums.each |&p| {
        if p * p > n  { return true; }
        if n % p == 0 { return false; }
    }
    return true;
}

#[inline(always)]
priv fn grow(nums: &mut ~[uint], len: uint) {
    if nums.len() >= len { return; }

    let mut it = Counter::new(vec::last(*nums) + 2, 2);
    for it.advance |n| {
        if !is_coprime(*nums, n) { loop; }

        nums.push(n);
        if nums.len() >= len { return; }
    }
}

#[inline(always)]
pub fn iter() -> PrimeIterator { PrimeIterator::new() }

#[inline(always)]
pub fn each(f: &fn(uint) -> bool) -> bool {
    let nums = get_task_prime();

    let mut it = Counter::new::<uint>(0, 1);
    for it.advance |i| {
        grow(nums, i + 1);
        if !f(nums[i]) { return false; }
    }
    return true;
}

#[inline(always)]
pub fn contains(n: uint) -> bool {
    fn inner(nums: &mut ~[uint], n: uint) -> bool {
        let len = vec::uniq_len(nums);
        let last = nums[len - 1];;
        if n < last {
            return vec::bsearch_elem(*nums, &n).is_some();
        }

        let mut it = Counter::new::<uint>(0, 1);
        for it.advance |i| {
            grow(nums, i + 1);
            let p = nums[i];
            if p * p > n { return true; }
            if n % p == 0 { return false; }
        }
        util::unreachable();
    }
    return with_task_prime(|nums| inner(nums, n));
}

#[inline(always)]
pub fn nth(i: uint) -> uint {
    let nums = get_task_prime();

    grow(nums, i + 1);
    return nums[i];
}

priv struct PrimeIterator {
    priv idx: uint
}

impl PrimeIterator {
    #[inline(always)]
    pub fn new() -> PrimeIterator { PrimeIterator { idx: 0 } }
}

impl Iterator<uint> for PrimeIterator {
    #[inline(always)]
    fn next(&mut self) -> Option<uint> {
        let p = nth(self.idx);
        self.idx += 1;
        return Some(p);
    }
}

#[inline(always)]
pub fn factorize(n: uint) -> FactorIterator { FactorIterator::new(n) }

pub type Factor = (uint, int);

priv struct FactorIterator {
    priv num: uint,
    priv prime_iter: PrimeIterator
}

impl FactorIterator {
    #[inline(always)]
    pub fn new(num: uint) -> FactorIterator {
        FactorIterator { num: num, prime_iter: PrimeIterator::new() }
    }
}

impl Iterator<Factor> for FactorIterator {
    fn next(&mut self) -> Option<Factor> {
        if self.num == 0 || self.num == 1 { return None; }

        while self.num > 1 {
            let p = self.prime_iter.next().get();

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
pub fn comb(n: uint, r: uint) -> uint {
    let numer = MergeMultiMonoidIterator::new(
        Range::new(r + 1, n + 1)
        .transform(factorize)
        .transform(|fs| fs.transform(|(base, exp)| (base, Sum(exp))))
        .to_vec()
    );

    let denom = MergeMultiMonoidIterator::new(
        Range::new(1, n - r + 1)
        .transform(factorize)
        .transform(|fs| fs.transform(|(base, exp)| (base, Sum(-exp))))
        .to_vec()
    );

    return factors_to_uint(
        MergeMonoidIterator::new(numer, denom).transform(|(a, m)| (a, m.unwrap()))
    );
}

#[inline(always)]
pub fn num_of_divisors(n: uint) -> uint {
    if n == 0 { return 0; }
    return factorize(n)
        .transform(|(_base, exp)| (exp as uint) + 1)
        .prod();
}

#[inline(always)]
pub fn num_of_proper_divisors(n: uint) -> uint {
    num_of_proper_divisors(n) - 1
}

#[inline(always)]
pub fn sum_of_divisors(n: uint) -> uint {
    if n == 0 { return 0; }
    return factorize(n)
        .transform(|(base, exp)| {
            (pow(base, (exp as uint) + 1) - 1) / (base - 1)
        }).prod();
}

#[inline(always)]
pub fn sum_of_proper_divisors(n: uint) -> uint {
    sum_of_divisors(n) - n
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::iterator::{ IteratorUtil };

    static PRIMES_BELOW200: &'static [uint] = &[
        2,  3,  5,  7, 11, 13, 17, 19, 23, 29, 31, 37, 41,
        43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97,
        101, 103, 107, 109, 113, 127, 131, 137, 139, 149,
        151, 157, 163, 167, 173, 179, 181, 191, 193, 197, 199
    ];

    #[test]
    fn test_prime_each() {
        let table = PRIMES_BELOW200.slice(0, 13);
        let table2 = PRIMES_BELOW200.slice(13, PRIMES_BELOW200.len());

        let mut v1 = ~[];
        for each |p| {
            if p > *vec::last(table) { break; }
            v1 += [ p ];
        }
        assert_eq!(table.initn(0), v1.initn(0));

        let mut v2 = ~[];
        for each |p| {
            if p > *vec::last(table) { break; }
            v2 += [ p ];
        }
        assert_eq!(table.initn(0), v2.initn(0));

        let mut v3 = ~[];
        for each |p| {
            if p > *vec::last(table2) { break; }
            v3 += [ p ];
        }
        assert_eq!(~[] + table + table2, v3);
    }

    #[test]
    fn test_prime_contains() {
        assert!(!contains(0));
        assert!(!contains(1));
        assert!(contains(2));
        assert!(contains(3));
        assert!(!contains(4));
        assert!(contains(5));
        assert!(!contains(6));
        assert!(contains(7));
        assert!(!contains(100));
    }

    #[test]
    fn test_prime_index() {
        // Generated primes
        for PRIMES_BELOW200.eachi() |i, &p| { assert_eq!(nth(i), p); }
        // Memoized primes
        for PRIMES_BELOW200.eachi() |i, &p| { assert_eq!(nth(i), p); }
    }

    #[test]
    fn test_iter() {
        let mut it = iter();

        let mut i = 0;
        let ys = &[ 2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41 ];
        for it.advance |x| {
            if i >= ys.len() { break; }
            assert_eq!(x, ys[i]);
            i += 1;
        }
    }

    #[test]
    fn test_factorize() {
        fn check(n: uint, fs: &[Factor]) {
            let mut v = ~[];
            let mut it = factorize(n);
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
        assert_eq!(comb(2, 2), 1);
        assert_eq!(comb(3, 2), 3);
        assert_eq!(comb(4, 2), 6);
        assert_eq!(comb(5, 2), 10);

        assert_eq!(comb(40, 20), 137846528820);
    }

    #[test]
    fn test_num_of_divisors() {
        assert_eq!(num_of_divisors(1), 1);
        assert_eq!(num_of_divisors(2), 2);
        assert_eq!(num_of_divisors(3), 2);
        assert_eq!(num_of_divisors(4), 3);
        assert_eq!(num_of_divisors(5), 2);
        assert_eq!(num_of_divisors(6), 4);
        assert_eq!(num_of_divisors(7), 2);
        assert_eq!(num_of_divisors(8), 4);
        assert_eq!(num_of_divisors(9), 3);
        assert_eq!(num_of_divisors(10), 4);
        assert_eq!(num_of_divisors(11), 2);
        assert_eq!(num_of_divisors(12), 6);

        assert_eq!(num_of_divisors(24), 8);
        assert_eq!(num_of_divisors(36), 9);
        assert_eq!(num_of_divisors(48), 10);
        assert_eq!(num_of_divisors(60), 12);

        assert_eq!(num_of_divisors(50), 6);
    }

    #[test]
    fn test_sum_of_divisors() {
        assert_eq!(sum_of_divisors(1), 1);
        assert_eq!(sum_of_divisors(2), 3);
        assert_eq!(sum_of_divisors(3), 4);
        assert_eq!(sum_of_divisors(4), 7);
        assert_eq!(sum_of_divisors(5), 6);
        assert_eq!(sum_of_divisors(6), 12);
        assert_eq!(sum_of_divisors(7), 8);
        assert_eq!(sum_of_divisors(8), 15);
        assert_eq!(sum_of_divisors(9), 13);
        assert_eq!(sum_of_divisors(10), 18);
        assert_eq!(sum_of_divisors(11), 12);
        assert_eq!(sum_of_divisors(12), 28);

        assert_eq!(sum_of_divisors(24), 60);
        assert_eq!(sum_of_divisors(36), 91);
        assert_eq!(sum_of_divisors(48), 124);
        assert_eq!(sum_of_divisors(60), 168);

        assert_eq!(sum_of_divisors(50), 93);
    }
}
