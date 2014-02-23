use std::mem;
use collections::HashSet;
use num::Integer;
use arith::isqrt;

pub fn sqrt(n: uint) -> (uint, ~[uint]) {
    let mut a0 = 0;
    let mut an = ~[];
    let mut set = HashSet::new();

    for (a, pqr) in A::new(n) {
        if a == 0 || set.contains(&(a, pqr)) {
            break;
        }

        set.insert((a, pqr));
        if set.len() == 1 {
            a0 = a;
        } else {
            an.push(a);
        }
    }
    return (a0, an);

    struct A {
        n: uint,
        sqn: uint,
        pqr: (uint, uint, uint)
    }
    impl A {
        fn new(n: uint) -> A {
            A { n: n, sqn: isqrt(n), pqr: (1, 0, 1) }
        }

        // a <= f_n(p, q, r) < a + 1
        // r a - q <= p sqrt(n) < r (a + 1) - pq
        // (ar - q)^2 <= np^2 < ((a+1)r - q)^2
        fn calc_a(&self) -> uint {
            // g(a, r, q) := (ar - q)^2
            #[inline]
            fn g(a: uint, r: uint, q: uint) -> uint {
                let s = a * r - q;
                return s * s;
            }

            let &A { n, sqn, pqr: (p, q, r) } = self;
            let np2 = n * p * p;
            let estim_a = (p * sqn + q) / r;
            let mut a = estim_a;
            while g(a + 1, r, q) <= np2 {
                a = a + 1;
            }
            return a;
        }
    }

    impl Iterator<(uint, (uint, uint, uint))> for A {
        // f_n (p, q, r) := (p sqrt(n) + q)/ r
        //                = a + (1 / (rp sqrt(n) + rb) / (np^2 - b^2))
        // a := |f_n(p, q, r)|
        // b := ar - q
        // (p, q, r) := (rp / m, rb / m, (np^2 - b^2) / m)
        #[inline]
        fn next(&mut self) -> Option<(uint, (uint, uint, uint))> {
            let a = self.calc_a();
            let &A { n, pqr: (p, q, r), ..} = self;

            self.pqr = if a * a == n || p == 0 {
                (0, 0, 1)
            } else {
                let b = a * r - q;
                let (p2, q2, r2) = (r*p, r*b, n*p*p - b*b);
                let m = p2.gcd(&q2).gcd(&r2);
                (p2 / m, q2 / m, r2 / m)
            };

            Some((a, self.pqr))
        }
    }
}

pub fn fold<T: FromPrimitive + Add<T, T> + Mul<T, T>>(an: &[uint]) -> (T, T) {
    let mut numer: T = FromPrimitive::from_int(1).unwrap();
    let mut denom: T = FromPrimitive::from_int(0).unwrap();

    for &a in an.rev_iter() {
        mem::swap(&mut numer, &mut denom);
        let num: T = FromPrimitive::from_int(a as int).unwrap();
        numer = numer + num * denom;
    }

    return (numer, denom);
}

/// solve pel equation x^2 - d y^2 = 1
pub fn solve_pel<T: FromPrimitive + Add<T, T> + Mul<T, T>>(d: uint) -> (T, T) {
    let (a0, an) = sqrt(d);
    if an.is_empty() {
        fail!("{} is square", d)
    } else if an.len() % 2 == 0 {
        fold::<T>(~[a0] + an.init())
    } else {
        fold::<T>(~[a0] + an + an.init())
    }
}

/// solve pel equation x^2 - d y^2 = -1
pub fn solve_pel_neg<T: FromPrimitive + Add<T, T> + Mul<T, T>>(d: uint) -> (T, T) {
    let (a0, an) = sqrt(d);
    if an.len() % 2 == 0 {
        return fold::<T>(~[a0] + an + an.init());
    } else {
        return fold::<T>(~[a0] + an.init());
    }
}


/// iterates all (x, y) sufficient x^2 - d y^2 = 1
pub struct PelIterator<T> {
    d: T,
    x1y1: (T, T),
    xy: (T, T)
}

impl<T: Clone + FromPrimitive + Add<T, T> + Mul<T, T>> PelIterator<T> {
    #[inline]
    pub fn new(d: uint) -> PelIterator<T> {
        let x1y1 = solve_pel(d);
        let xy   = x1y1.clone();
        PelIterator {
            d: FromPrimitive::from_uint(d).unwrap(),
            x1y1: x1y1, xy: xy
        }
    }
}

impl<T: Add<T, T> + Mul<T, T>> Iterator<(T, T)> for PelIterator<T> {
    // x[k] + y[k]sqrt(n) = (x[1] + y[1]*sqrt(n))^k
    // x[k+1] + y[k+1]sqrt(n) = (x[k] + y[k]sqrt(n)) * (x[1] + y[1]*sqrt(n))
    //                        = (x[k]x[1] + n*y[k]y[1]) + (x[1]y[k] + x[k]y[1])sqrt(n)
    #[inline]
    fn next(&mut self) -> Option<(T, T)> {
        let next = {
            let ref d = self.d;
            let (ref x1, ref y1) = self.x1y1;
            let (ref xk, ref yk) = self.xy;
            ((*xk) * (*x1) + d * (*yk) * (*y1),
             (*yk) * (*x1) +     (*xk) * (*y1))
        };

        Some(mem::replace(&mut self.xy, next))
    }
}


/// iterates all (x, y) sufficient x^2 - d y^2 = -1
pub struct PelNegIterator<T> {
    d: T,
    x1y1: (T, T),
    xy: (T, T)
}

impl<T: Clone + FromPrimitive + Add<T, T> + Mul<T, T>> PelNegIterator<T> {
    #[inline]
    pub fn new(d: uint) -> PelNegIterator<T> {
        let x1y1 = solve_pel_neg(d);
        let xy   = x1y1.clone();
        PelNegIterator {
            d: FromPrimitive::from_uint(d).unwrap(),
            x1y1: x1y1, xy: xy
        }
    }
}

impl<T: Add<T, T> + Mul<T, T>> Iterator<(T, T)> for PelNegIterator<T> {
    #[inline]
    fn next(&mut self) -> Option<(T, T)> {
        let next = {
            let ref d = self.d;
            let (ref x1, ref y1) = self.x1y1;
            let (ref xk, ref yk) = self.xy;
            let (xk, yk) = ((*xk) * (*x1) + d * (*yk) * (*y1),
                            (*yk) * (*x1) +     (*xk) * (*y1));
            (xk * (*x1) + d * yk * (*y1),
             yk * (*x1) +     xk * (*y1))
        };

        Some(mem::replace(&mut self.xy, next))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_sqrt() {
        assert_eq!(super::sqrt(1), (1, ~[]));
        assert_eq!(super::sqrt(2), (1, ~[2]));
        assert_eq!(super::sqrt(3), (1, ~[1,2]));
        assert_eq!(super::sqrt(4), (2, ~[]));
        assert_eq!(super::sqrt(5), (2, ~[4]));
        assert_eq!(super::sqrt(6), (2, ~[2,4]));
        assert_eq!(super::sqrt(7), (2, ~[1,1,1,4]));
        assert_eq!(super::sqrt(8), (2, ~[1,4]));
        assert_eq!(super::sqrt(9), (3, ~[]));
        assert_eq!(super::sqrt(10), (3, ~[6]));
        assert_eq!(super::sqrt(11), (3, ~[3,6]));
        assert_eq!(super::sqrt(12), (3, ~[2,6]));
        assert_eq!(super::sqrt(13), (3, ~[1,1,1,1,6]));
    }

    #[deriving(Eq)]
    struct Uint(uint);

    impl Uint {
        fn unwrap(&self) -> uint {
            let Uint(n) = *self;
            n
        }
    }

    impl FromPrimitive for Uint {
        fn from_i64(n: i64) -> Option<Uint> { FromPrimitive::from_i64(n).map(Uint) }
        fn from_u64(n: u64) -> Option<Uint> { FromPrimitive::from_u64(n).map(Uint) }
    }
    impl Add<Uint, Uint> for Uint {
        fn add(&self, other: &Uint) -> Uint { Uint(self.unwrap() + other.unwrap()) }
    }
    impl Mul<Uint, Uint> for Uint {
        fn mul(&self, other: &Uint) -> Uint { Uint(self.unwrap() * other.unwrap()) }
    }

    #[test]
    fn test_fold() {

        fn test(an: &[uint], (n, d): (uint, uint)) {
            assert_eq!(super::fold::<Uint>(an), (Uint(n), Uint(d)));
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

    #[test]
    fn test_solve_pel() {
        assert_eq!(super::solve_pel(2), (3, 2));
        assert_eq!(super::solve_pel(3), (2, 1));
        assert_eq!(super::solve_pel(5), (9, 4));
        assert_eq!(super::solve_pel(6), (5, 2));
        assert_eq!(super::solve_pel(7), (8, 3));
    }
    #[test] #[should_fail]
    fn test_solve_pel_1() { super::solve_pel::<uint>(1); }
    #[test] #[should_fail]
    fn test_solve_pel_4() { super::solve_pel::<uint>(4); }
}
