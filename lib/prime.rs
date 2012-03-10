export prime, init, grow, get_at, iterable_prime, loopable_prime, factors, iterable_factors;

use std;

type prime = {
    mutable vec: [u64]
};

impl iterable_prime of iter::iterable<u64> for prime {
    fn iter(blk: fn(&&u64)) {
        let i = 0u;
        while true {
            blk(get_at_vec(self.vec, i));
            i += 1u;
        }
    }
}

impl loopable_prime for prime {
    fn iterate(blk: fn(&&u64) -> bool) {
        let i = 0u;
        while blk(get_at_vec(self.vec, i)) {
            i += 1u;
        }
    }
}

impl iterable_factors of iter::iterable<(u64, i64)> for fn@(fn((u64, i64))) {
    fn iter(blk: fn((u64, i64))) {
        self(blk);
    }
}


fn init() -> prime {
    { mutable vec: [] }
}

fn grow(ps: prime, n: uint) {
    grow_vec(ps.vec, n);
}

fn get_at(&ps: prime, n: u64) -> u64 {
    ret get_at_vec(ps.vec, n);
}

fn grow_vec(&v: [u64], n: uint) {
    if n == 0u {
        ret;
    }
    let num = alt vec::last_opt(v) {
      none       { v = [2u64];  grow_vec(v, n - 1u); ret }
      some(2u64) { v += [3u64]; grow_vec(v, n - 1u); ret }
      some(x)    { x + 2u64 }
    };

    let i = 0u;
    while i < n {
        for p in v {
            if p * p > num {
                v += [num];
                i += 1u;
                break;
            }
            if num % p == 0u64 {
                break;
            }
        }
        num += 2u;
    }
}

fn get_at_vec(&v: [u64], n: u64) -> u64 {
    if vec::len(v) <= n {
        grow_vec(v, n - vec::len(v) + 1u);
    }
    ret v[n];
}

fn div_multi(&num: u64, f: u64) -> u64 {
    let exp = 0u64;
    while (num % f == 0u64) {
        exp += 1u64;
        num /= f;
    }
    ret exp;
}

fn factors(num: u64, &primes: prime) -> fn@(fn((u64, i64))) {
    ret { |blk|
        let itr = num;
        primes.iterate { |p|
            let exp = div_multi(itr, p);
            if exp > 0u64 {
                blk((p, exp as i64));
            }
            ret itr != 1u;
        };
    };
}