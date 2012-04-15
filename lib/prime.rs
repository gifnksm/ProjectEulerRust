export prime, factors;

enum grow_cond { count(uint), value(u64) }

class prime {
    priv {
        let mut vec: [u64];

        fn is_coprime(num: u64) -> bool {
            for self.vec.each() {|p|
                if p * p > num {
                    ret true;
                }
                if num % p == 0u64 {
                    ret false;
                }
            }
            ret true;
        }

        fn check_cond(cond: grow_cond) -> bool {
            alt cond {
              count(cnt) { cnt < self.vec.len() }
              value(val) {
                alt vec::last_opt(self.vec) {
                  some(x) { val < x }
                  none    { false }
                }
              }
            }
        }

        fn grow_until(cond: grow_cond) {
            if self.check_cond(cond) {
                ret;
            }

            let mut num = alt vec::last_opt(self.vec) {
              none       { self.vec =  [2u64]; ret self.grow_until(cond); }
              some(2u64) { self.vec += [3u64]; ret self.grow_until(cond); }
              some(x)    { x + 2u64 }
            };

            while !self.check_cond(cond) {
                if self.is_coprime(num) {
                    self.vec += [num];
                }
                num += 2u64;
            }
        }
    }

    new() { self.vec = []; }

    fn [](idx: uint) -> u64 {
        self.grow_until(count(idx + 1u));
        ret self.vec[idx];
    }

    fn is_prime(num: u64) -> bool {
        if num < 2u64 {
            ret false;
        }
        self.grow_until(value(calc::isqrt(num)));

        for self.vec.each() {|p|
            if p * p > num {
                ret true;
            }
            if num % p == 0u {
                ret false;
            }
        };
        ret true;
    }

    fn each(f: fn(&&u64) -> bool) {
        let mut idx = 0u64;
        for self.vec.each {|p|
            if !f(p) {
                ret;
            }
            idx += 1u;
        }
        loop {
            if !f(self[idx]) {
                ret;
            }
            idx += 1u;
        }
    }
}

fn div_multi(&num: u64, f: u64) -> u64 {
    let mut exp = 0u64;
    while (num % f == 0u64) {
        exp += 1u64;
        num /= f;
    }
    ret exp;
}

fn factors(num: u64, primes: prime, blk: fn((u64, i64))) {
    let mut itr = num;
    for primes.each {|p|
        let exp = div_multi(itr, p);
        if exp > 0u64 {
            blk((p, exp as i64));
        }
        if itr == 1u {
            break;
        }
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_prime_opidx () {
        let table  = [  2u64,  3u64,  5u64,  7u64, 11u64, 13u64, 17u64, 19u64, 23u64, 29u64, 31u64, 37u64, 41u64 ];
        let ps = prime();

        // Generated primes
        for table.eachi() {|i, p|
            assert ps[i] == p;
        }
        // Memoized primes
        for table.eachi() {|i, p|
            assert ps[i] == p;
        }
    }

    #[test]
    fn test_prime_each() {
        let table  = [  2u64,  3u64,  5u64,  7u64, 11u64, 13u64, 17u64, 19u64, 23u64, 29u64, 31u64, 37u64, 41u64 ];
        let table2 = [ 43u64, 47u64, 53u64, 59u64, 61u64, 67u64, 71u64, 73u64, 79u64, 83u64, 89u64, 97u64 ];
        let ps = prime();

        let mut v1 = [];
        for ps.each {|p|
            if p > table.last() {
                break;
            }
            v1 += [ p ];
        }
        assert table == v1;

        let mut v2 = [];
        for ps.each {|p|
            if p > table.last() {
                break;
            }
            v2 += [ p ];
        }
        assert table == v2;

        let mut v3 = [];
        for ps.each {|p|
            if p > table2.last() {
                break;
            }
            v3 += [ p ];
        }
        assert table + table2 == v3;
    }

    #[test]
    fn test_prime_is_prime() {
        let p = prime();
        assert !p.is_prime(1u64);
        assert p.is_prime(2u64);
        assert p.is_prime(3u64);
        assert !p.is_prime(4u64);
        assert p.is_prime(5u64);
        assert !p.is_prime(6u64);
        assert p.is_prime(7u64);
        assert !p.is_prime(100u64);
    }
}
