export grow, get_at, init, iterable_prime, loopable_prime;

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
    fn loop(blk: fn(&&u64) -> bool) {
        let i = 0u;
        while blk(get_at_vec(self.vec, i)) {
            i += 1u;
        }
    }
}

fn init() -> prime {
    { mutable vec: [] }
}

fn grow_vec(&v: [u64], n: uint) {
    if n == 0u {
        ret;
    }
    let num = alt vec::last(v) {
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

fn grow(ps: prime, n: uint) {
    grow_vec(ps.vec, n);
}

fn get_at(&ps: prime, n: u64) -> u64 {
    if vec::len(ps.vec) <= n {
        grow(ps, n - vec::len(ps.vec));
    }
    ret ps.vec[n];
}
