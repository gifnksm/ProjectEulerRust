//! Continued fraction generator and related functions.

#![warn(
    bad_style,
    missing_docs,
    unused,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

use integer::Integer;
use num_integer::Integer as NumInteger;
use num_traits::FromPrimitive;
use std::{
    collections::HashSet,
    mem,
    ops::{Add, Mul},
};

/// Calculates a square root of the number as continued fraction form.
///
/// `(1, vec![2])` represents 1 + 1 / (2 + 1 / (2 + 1 / (2 + ... )))
/// ```rust
/// assert_eq!((1, vec![2]), cont_frac::sqrt(2));
/// assert_eq!((1, vec![1, 2]), cont_frac::sqrt(3));
/// assert_eq!((2, vec![]), cont_frac::sqrt(4));
/// ```
pub fn sqrt(n: u32) -> (u32, Vec<u32>) {
    let mut a0 = 0;
    let mut an = Vec::new();
    let mut set = HashSet::new();

    for (a, pqr) in A::new(n) {
        if a == 0 || set.contains(&(a, pqr)) {
            break;
        }

        let _ = set.insert((a, pqr));
        if set.len() == 1 {
            a0 = a;
        } else {
            an.push(a);
        }
    }
    return (a0, an);

    struct A {
        n: u32,
        sqn: u32,
        pqr: (u32, u32, u32),
    }
    impl A {
        fn new(n: u32) -> A {
            A {
                n,
                sqn: n.sqrt(),
                pqr: (1, 0, 1),
            }
        }

        // a <= f_n(p, q, r) < a + 1
        // r a - q <= p sqrt(n) < r (a + 1) - pq
        // (ar - q)^2 <= np^2 < ((a+1)r - q)^2
        fn calc_a(&self) -> u32 {
            // g(a, r, q) := (ar - q)^2
            #[inline]
            fn g(a: u32, r: u32, q: u32) -> u32 {
                let s = a * r - q;
                s * s
            }

            let &A {
                n,
                sqn,
                pqr: (p, q, r),
            } = self;
            let np2 = n * p * p;
            let estim_a = (p * sqn + q) / r;
            let mut a = estim_a;
            while g(a + 1, r, q) <= np2 {
                a += 1;
            }
            a
        }
    }

    impl Iterator for A {
        type Item = (u32, (u32, u32, u32));

        // f_n (p, q, r) := (p sqrt(n) + q)/ r
        //                = a + (1 / (rp sqrt(n) + rb) / (np^2 - b^2))
        // a := |f_n(p, q, r)|
        // b := ar - q
        // (p, q, r) := (rp / m, rb / m, (np^2 - b^2) / m)
        #[inline]
        #[allow(clippy::many_single_char_names)]
        fn next(&mut self) -> Option<(u32, (u32, u32, u32))> {
            let a = self.calc_a();
            let &mut A {
                n, pqr: (p, q, r), ..
            } = self;

            self.pqr = if a * a == n || p == 0 {
                (0, 0, 1)
            } else {
                let b = a * r - q;
                let (p2, q2, r2) = (r * p, r * b, n * p * p - b * b);
                let m = p2.gcd(&q2).gcd(&r2);
                (p2 / m, q2 / m, r2 / m)
            };

            Some((a, self.pqr))
        }
    }
}

/// Calculates convergent of an input iterator.
pub fn fold<T, I>(an: I) -> (T, T)
where
    T: FromPrimitive + Add<T, Output = T> + Mul<T, Output = T> + Clone,
    I: Iterator<Item = u32> + DoubleEndedIterator,
{
    let mut numer: T = FromPrimitive::from_u32(1).unwrap();
    let mut denom: T = FromPrimitive::from_u32(0).unwrap();

    for a in an.rev() {
        mem::swap(&mut numer, &mut denom);
        let num: T = FromPrimitive::from_u32(a).unwrap();
        numer = numer + num * denom.clone();
    }

    (numer, denom)
}

/// solve pel equation x^2 - d y^2 = 1
pub fn solve_pel<T>(d: u32) -> (T, T)
where
    T: FromPrimitive + Add<T, Output = T> + Mul<T, Output = T> + Clone,
{
    let (a0, an) = sqrt(d);
    if an.is_empty() {
        panic!("{} is square", d)
    }

    let len = an.len();
    let mut v = vec![a0];
    if len % 2 == 0 {
        v.extend(&an[..len - 1])
    } else {
        v.extend(&an);
        v.extend(&an[..len - 1])
    }
    fold(v.into_iter())
}

/// solve pel equation x^2 - d y^2 = -1
pub fn solve_pel_neg<T>(d: u32) -> (T, T)
where
    T: FromPrimitive + Add<T, Output = T> + Mul<T, Output = T> + Clone,
{
    let (a0, an) = sqrt(d);

    let len = an.len();
    let mut v = vec![a0];
    if len % 2 == 0 {
        v.extend(&an);
        v.extend(&an[..len - 1]);
    } else {
        v.extend(&an[..len - 1]);
    }
    fold(v.into_iter())
}

/// iterates all (x, y) sufficient x^2 - d y^2 = 1
pub struct PelRoots<T> {
    d: T,
    x1y1: (T, T),
    xy: (T, T),
}

impl<T> PelRoots<T>
where
    T: Clone + FromPrimitive + Add<T, Output = T> + Mul<T, Output = T>,
{
    /// Creates a new `PelRoots` iterator
    #[inline]
    pub fn new(d: u32) -> PelRoots<T> {
        let x1y1 = solve_pel(d);
        let xy = x1y1.clone();
        PelRoots {
            d: FromPrimitive::from_u32(d).unwrap(),
            x1y1,
            xy,
        }
    }
}

impl<T> Iterator for PelRoots<T>
where
    T: Add<T, Output = T> + Mul<T, Output = T> + Clone,
{
    type Item = (T, T);

    // x[k] + y[k]sqrt(n) = (x[1] + y[1]*sqrt(n))^k
    // x[k+1] + y[k+1]sqrt(n) = (x[k] + y[k]sqrt(n)) * (x[1] + y[1]*sqrt(n))
    //                        = (x[k]x[1] + n*y[k]y[1]) + (x[1]y[k] + x[k]y[1])sqrt(n)
    #[inline]
    fn next(&mut self) -> Option<(T, T)> {
        let next = {
            let d = &self.d;
            let (ref x1, ref y1) = self.x1y1;
            let (ref xk, ref yk) = self.xy;
            (
                xk.clone() * x1.clone() + d.clone() * yk.clone() * y1.clone(),
                yk.clone() * x1.clone() + xk.clone() * y1.clone(),
            )
        };

        Some(mem::replace(&mut self.xy, next))
    }
}

/// iterates all (x, y) sufficient x^2 - d y^2 = -1
pub struct PelNegRoots<T> {
    d: T,
    x1y1: (T, T),
    xy: (T, T),
}

impl<T> PelNegRoots<T>
where
    T: Clone + FromPrimitive + Add<T, Output = T> + Mul<T, Output = T>,
{
    /// Creates a new `PelNegRoots` iterator
    #[inline]
    pub fn new(d: u32) -> PelNegRoots<T> {
        let x1y1 = solve_pel_neg(d);
        let xy = x1y1.clone();
        PelNegRoots {
            d: FromPrimitive::from_u32(d).unwrap(),
            x1y1,
            xy,
        }
    }
}

impl<T> Iterator for PelNegRoots<T>
where
    T: Add<T, Output = T> + Mul<T, Output = T> + Clone,
{
    type Item = (T, T);

    #[inline]
    fn next(&mut self) -> Option<(T, T)> {
        let next = {
            let d = &self.d;
            let (ref x1, ref y1) = self.x1y1;
            let (ref xk, ref yk) = self.xy;
            let (xk, yk) = (
                xk.clone() * x1.clone() + d.clone() * yk.clone() * y1.clone(),
                yk.clone() * x1.clone() + xk.clone() * y1.clone(),
            );
            (
                xk.clone() * x1.clone() + d.clone() * yk.clone() * y1.clone(),
                yk * x1.clone() + xk * y1.clone(),
            )
        };

        Some(mem::replace(&mut self.xy, next))
    }
}

#[cfg(test)]
mod tests {
    use num_traits::FromPrimitive;
    use std::ops::{Add, Mul};

    #[test]
    fn sqrt() {
        assert_eq!(super::sqrt(1), (1, vec![]));
        assert_eq!(super::sqrt(2), (1, vec![2]));
        assert_eq!(super::sqrt(3), (1, vec![1, 2]));
        assert_eq!(super::sqrt(4), (2, vec![]));
        assert_eq!(super::sqrt(5), (2, vec![4]));
        assert_eq!(super::sqrt(6), (2, vec![2, 4]));
        assert_eq!(super::sqrt(7), (2, vec![1, 1, 1, 4]));
        assert_eq!(super::sqrt(8), (2, vec![1, 4]));
        assert_eq!(super::sqrt(9), (3, vec![]));
        assert_eq!(super::sqrt(10), (3, vec![6]));
        assert_eq!(super::sqrt(11), (3, vec![3, 6]));
        assert_eq!(super::sqrt(12), (3, vec![2, 6]));
        assert_eq!(super::sqrt(13), (3, vec![1, 1, 1, 1, 6]));
    }

    #[derive(Eq, PartialEq, Debug, Clone)]
    struct U32(u32);

    impl U32 {
        fn unwrap(&self) -> u32 {
            let U32(n) = *self;
            n
        }
    }

    impl FromPrimitive for U32 {
        fn from_i64(n: i64) -> Option<U32> {
            FromPrimitive::from_i64(n).map(U32)
        }
        fn from_u64(n: u64) -> Option<U32> {
            FromPrimitive::from_u64(n).map(U32)
        }
    }
    impl Add<U32> for U32 {
        type Output = U32;

        fn add(self, other: U32) -> U32 {
            U32(self.unwrap() + other.unwrap())
        }
    }
    impl Mul<U32> for U32 {
        type Output = U32;

        fn mul(self, other: U32) -> U32 {
            U32(self.unwrap() * other.unwrap())
        }
    }

    #[test]
    fn fold() {
        fn check(an: &[u32], (n, d): (u32, u32)) {
            assert_eq!(super::fold(an.iter().copied()), (U32(n), U32(d)));
        }

        check(&[1, 2], (3, 2));
        check(&[1, 2, 2], (7, 5));
        check(&[1, 2, 2, 2], (17, 12));
        check(&[1, 2, 2, 2, 2], (41, 29));

        check(&[2], (2, 1));
        check(&[2, 1], (3, 1));
        check(&[2, 1, 2], (8, 3));
        check(&[2, 1, 2, 1], (11, 4));
        check(&[2, 1, 2, 1, 1], (19, 7));
        check(&[2, 1, 2, 1, 1, 4], (87, 32));
        check(&[2, 1, 2, 1, 1, 4, 1], (106, 39));
        check(&[2, 1, 2, 1, 1, 4, 1, 1], (193, 71));
        check(&[2, 1, 2, 1, 1, 4, 1, 1, 6], (1264, 465));
        check(&[2, 1, 2, 1, 1, 4, 1, 1, 6, 1], (1457, 536));
    }

    #[test]
    fn solve_pel() {
        assert_eq!(super::solve_pel(2), (3, 2));
        assert_eq!(super::solve_pel(3), (2, 1));
        assert_eq!(super::solve_pel(5), (9, 4));
        assert_eq!(super::solve_pel(6), (5, 2));
        assert_eq!(super::solve_pel(7), (8, 3));
    }
    #[test]
    #[should_panic]
    fn solve_pel_1() {
        let _ = super::solve_pel::<u32>(1);
    }
    #[test]
    #[should_panic]
    fn solve_pel_4() {
        let _ = super::solve_pel::<u32>(4);
    }
}
