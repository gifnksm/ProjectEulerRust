use std::num::IntConvertible;
use std::hashmap::HashSet;
use std::util;

use arith::isqrt;

pub fn sqrt(n: uint) -> (uint, ~[uint]) {
    let mut a0 = 0;
    let mut an = ~[];
    let mut set = HashSet::new();

    do each_a(n) |a, pqr| {
        if a == 0 || set.contains(&(a, pqr)) {
            false
        } else {
            set.insert((a, pqr));
            if set.len() == 1 {
                a0 = a;
            } else {
                an.push(a);
            }
            true
        }
    };
    return (a0, an);

    // f_n (p, q, r) := (p sqrt(n) + q)/ r
    //                = a + (1 / (rp sqrt(n) + rb) / (np^2 - b^2))
    // a := |f_n(p, q, r)|
    // b := ar - q
    // (p, q, r) := (rp / m, rb / m, (np^2 - b^2) / m)
    #[inline(always)]
    fn each_a(n: uint, f: &fn(uint, (uint, uint, uint)) -> bool) -> bool {
        let sqn = isqrt(n);
        let mut p = 1;
        let mut q = 0;
        let mut r = 1;
        loop {
            let a = calc_a(n, sqn, (p, q, r));
            if a * a == n || p == 0 {
                p = 0; q = 0; r = 1;
            } else {
                let b = a * r - q;
                let (p2, q2, r2) = (r*p, r*b, n*p*p - b*b);
                let m = p2.gcd(&q2).gcd(&r2);
                p = p2 / m;
                q = q2 / m;
                r = r2 / m;
            }
            if !f(a, (p, q, r)) { return false; }
        }
    }


    // a <= f_n(p, q, r) < a + 1
    // r a - q <= p sqrt(n) < r (a + 1) - pq
    // (ar - q)^2 <= np^2 < ((a+1)r - q)^2
    #[inline(always)]
    fn calc_a(n: uint, sqn: uint, (p, q, r): (uint, uint, uint)) -> uint {
        // g(a, r, q) := (ar - q)^2
        #[inline(always)]
        fn g(a: uint, r: uint, q: uint) -> uint {
            let s = a * r - q;
            return s * s;
        }

        let np2 = n * p * p;
        let estim_a = (p * sqn + q) / r;
        let mut a = estim_a;
        while g(a + 1, r, q) <= np2 {
            a = a + 1;
        }
        return a;
    }
}

pub fn fold<T: IntConvertible + Add<T, T> + Mul<T, T>>(an: &[uint]) -> (T, T) {
    let mut numer = IntConvertible::from_int::<T>(1);
    let mut denom = IntConvertible::from_int::<T>(0);

    for &a in an.rev_iter() {
        util::swap(&mut numer, &mut denom);
        numer = numer + IntConvertible::from_int::<T>(a as int) * denom;
    }

    return (numer, denom);
}

/// solve pel equation x^2 - y^2 = 1
pub fn solve_pel<T: IntConvertible + Add<T, T> + Mul<T, T>>(d: uint) -> (T, T) {
    let (a0, an) = sqrt(d);
    if an.len() % 2 == 0 {
        return fold::<T>(~[a0] + an.init());
    } else {
        return fold::<T>(~[a0] + an + an.init());
    }
}

/// each (x, y) sufficient x^2 - y^2 = 1
pub fn each_pel<
    T: IntConvertible + Add<T, T> + Mul<T, T> + Clone
    >(d: uint, f: &fn(&T, &T)->bool) -> bool {
    let n = IntConvertible::from_int::<T>(d as int);
    let (x1, y1) = solve_pel::<T>(d);
    let mut xk = x1.clone();
    let mut yk = y1.clone();
    loop {
        // x[k] + y[k]sqrt(n) = (x[1] + y[1]*sqrt(n))^k
        // x[k+1] + y[k+1]sqrt(n) = (x[k] + y[k]sqrt(n)) * (x[1] + y[1]*sqrt(n))
        //                        = (x[k]x[1] + n*y[k]y[1]) + (x[1]y[k] + x[k]y[1])sqrt(n)
        if !f(&xk, &yk) { return false; }
        let xk_1 = xk * x1 + n * yk * y1;
        let yk_1 = x1 * yk + xk * y1;
        xk = xk_1;
        yk = yk_1;
    }
}

/// solve pel equation x^2 - y^2 = -1
pub fn solve_pel_neg<T: IntConvertible + Add<T, T> + Mul<T, T>>(d: uint) -> (T, T) {
    let (a0, an) = sqrt(d);
    if an.len() % 2 == 0 {
        return fold::<T>(~[a0] + an + an.init());
    } else {
        return fold::<T>(~[a0] + an.init());
    }
}

/// each (x, y) sufficient x^2 - y^2 = -1
pub fn each_pel_neg<
    T: IntConvertible + Add<T, T> + Mul<T, T> + Clone
    >(d: uint, f: &fn(&T, &T)->bool) -> bool {
    let n = IntConvertible::from_int::<T>(d as int);
    let (x1, y1) = solve_pel_neg::<T>(d);
    let mut xk = x1.clone();
    let mut yk = y1.clone();
    let mut cnt = 0u;
    loop {
        // x[k] + y[k]sqrt(n) = (x[1] + y[1]*sqrt(n))^k
        // x[k+1] + y[k+1]sqrt(n) = (x[k] + y[k]sqrt(n)) * (x[1] + y[1]*sqrt(n))
        //                        = (x[k]x[1] + n*y[k]y[1]) + (x[1]y[k] + x[k]y[1])sqrt(n)
        if cnt.is_even() && !f(&xk, &yk) { return false; }
        let xk_1 = xk * x1 + n * yk * y1;
        let yk_1 = x1 * yk + xk * y1;
        xk = xk_1;
        yk = yk_1;
        cnt += 1;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::num::IntConvertible;

    #[test]
    fn test_sqrt() {
        assert_eq!(sqrt(1), (1, ~[]));
        assert_eq!(sqrt(2), (1, ~[2]));
        assert_eq!(sqrt(3), (1, ~[1,2]));
        assert_eq!(sqrt(4), (2, ~[]));
        assert_eq!(sqrt(5), (2, ~[4]));
        assert_eq!(sqrt(6), (2, ~[2,4]));
        assert_eq!(sqrt(7), (2, ~[1,1,1,4]));
        assert_eq!(sqrt(8), (2, ~[1,4]));
        assert_eq!(sqrt(9), (3, ~[]));
        assert_eq!(sqrt(10), (3, ~[6]));
        assert_eq!(sqrt(11), (3, ~[3,6]));
        assert_eq!(sqrt(12), (3, ~[2,6]));
        assert_eq!(sqrt(13), (3, ~[1,1,1,1,6]));
    }

    #[deriving(Eq)]
    struct Uint(uint);

    impl IntConvertible for Uint {
        fn to_int(&self) -> int { **self as int }
        fn from_int(n: int) -> Uint { Uint(n as uint) }
    }
    impl Add<Uint, Uint> for Uint {
        fn add(&self, other: &Uint) -> Uint { Uint(**self + **other) }
    }
    impl Mul<Uint, Uint> for Uint {
        fn mul(&self, other: &Uint) -> Uint { Uint(**self * **other) }
    }

    #[test]
    fn test_fold() {

        fn test(an: &[uint], (n, d): (uint, uint)) {
            assert_eq!(fold::<Uint>(an), (Uint(n), Uint(d)));
        }
        test([1, 2], (3, 2));
        test([1, 2, 2], (7, 5));
        test([1, 2, 2, 2], (17, 12));
        test([1, 2, 2, 2, 2], (41, 29));

        test([2], (2, 1));
        test([2, 1], (3, 1));
        test([2, 1, 2], (8, 3));
        test([2, 1, 2, 1], (11, 4));
        test([2, 1, 2, 1, 1], (19, 7));
        test([2, 1, 2, 1, 1, 4], (87, 32));
        test([2, 1, 2, 1, 1, 4, 1], (106, 39));
        test([2, 1, 2, 1, 1, 4, 1, 1], (193, 71));
        test([2, 1, 2, 1, 1, 4, 1, 1, 6], (1264, 465));
        test([2, 1, 2, 1, 1, 4, 1, 1, 6, 1], (1457, 536));
    }
}
