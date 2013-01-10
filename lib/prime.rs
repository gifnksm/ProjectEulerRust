use core::util::unreachable;

pub pure fn Prime() -> Prime {
    Prime { vec: ~[] }
}

pub struct Prime {
    priv mut vec: ~[uint],
}

impl Prime {
    priv pure fn is_coprime(num: uint) -> bool {
        for uint::range(0, self.vec.len()) |i| {
            let p = self.vec[i];
            if p * p > num  { return true; }
            if num % p == 0 { return false; }
        }
        return true;
    }

    priv fn grow_until(cond: fn() -> bool) {
        if cond() { return }

        let mut num = match vec::last_opt(self.vec) {
          None    => { self.vec =  ~[2]; return self.grow_until(cond); }
          Some(2) => { self.vec += ~[3]; return self.grow_until(cond); }
          Some(x) => { x + 2 }
        };

        while !cond() {
            if self.is_coprime(num) {
                self.vec += ~[num];
            }
            num += 2;
        }
    }

    fn get_at(idx: uint) -> uint {
        self.grow_until(|| idx < self.vec.len());
        return self.vec[idx];
    }

    fn is_prime(num: uint) -> bool {
        if num < 2 { return false; }

        for self.each |p| {
            if p * p > num  { return true;  }
            if num % p == 0 { return false; }
        }
        unreachable();
    }

    fn each(f: fn(uint) -> bool) {
        let init_len = self.vec.len();
        for uint::range(0, init_len) |i| {
            let p = self.vec[i];
            if !f(p) { return; }
        }

        let mut idx = init_len;
        loop {
            if !f(self.get_at(idx)) { return; }
            idx += 1;
        }
    }
}

priv fn div_multi(num: &mut uint, f: uint) -> uint {
    let mut exp = 0;
    while (*num % f == 0) {
        exp += 1;
        *num /= f;
    }
    return exp;
}

pub fn factors(num: uint, primes: &Prime, f: fn((uint, uint)) -> bool) {
    if num == 0 { return; }
    let mut itr = num;
    for primes.each |p| {
        let exp = div_multi(&mut itr, p);
        if exp > 0 {
            if !f((p, exp)) { break; }
        }
        if itr == 1  { break; }
    };
}

pub fn num_of_divisors(num: uint, primes: &Prime) -> uint {
    if num == 0 { return 0; }
    let mut prod = 1;
    for factors(num, primes) |f| {
        let (_base, exp) = f;
        prod *= exp + 1;
    }
    return prod;
}

pub fn sum_of_divisors(num: uint, primes: &Prime) -> uint {
    if num == 0 { return 0; }
    let mut sum = 1;
    for factors(num, primes) |f| {
        let (base, exp) = f;
        sum *= (int::pow(base as int, exp + 1) as uint - 1) / (base - 1);
    }
    return sum;
}

pub fn sum_of_proper_divisors(num: uint, primes: &Prime) -> uint {
    sum_of_divisors(num, primes) - num
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_prime_opidx () {
        let table  = [  2,  3,  5,  7, 11, 13, 17, 19, 23, 29, 31, 37, 41 ];
        let ps = Prime();

        // Generated primes
        for table.eachi() |i, p| { assert ps.get_at(i) == *p; }
        // Memoized primes
        for table.eachi() |i, p| { assert ps.get_at(i) == *p; }
    }

    #[test]
    fn test_prime_each() {
        let table  = ~[  2,  3,  5,  7, 11, 13, 17, 19, 23, 29, 31, 37, 41 ];
        let table2 = ~[ 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97 ];
        let ps = Prime();

        let mut v1 = ~[];
        for ps.each |p| {
            if p > table.last() { break; }
            v1 += [ p ];
        }
        assert table == v1;

        let mut v2 = ~[];
        for ps.each |p| {
            if p > table.last() { break; }
            v2 += [ p ];
        }
        assert table == v2;

        let mut v3 = ~[];
        for ps.each |p| {
            if p > table2.last() { break; }
            v3 += [ p ];
        }
        assert table + table2 == v3;
    }

    #[test]
    fn test_prime_is_prime() {
        let p = Prime();
        assert !p.is_prime(0);
        assert !p.is_prime(1);
        assert p.is_prime(2);
        assert p.is_prime(3);
        assert !p.is_prime(4);
        assert p.is_prime(5);
        assert !p.is_prime(6);
        assert p.is_prime(7);
        assert !p.is_prime(100);
    }

    #[test]
    fn test_factors() {
        let p = Prime();
        for factors(1, &p) |_f| {
            fail;
        }

        for factors(8, &p) |f| {
            assert f == (2, 3);
        }

        let mut v = ~[(2, 3), (3, 3)];
        for factors(8 * 27, &p) |f| {
            assert f == v.shift();
        }
    }
}
