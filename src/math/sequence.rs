use std::util;
use std::num::One;

pub fn fibonacci<T: One>() -> Fibonacci<T> {
    fibonacci_with_init(One::one(), One::one())
}

pub fn fibonacci_with_init<T>(a0: T, a1: T) -> Fibonacci<T> {
    Fibonacci { current: a0, next: a1 }
}

struct Fibonacci<T> { current: T, next: T }

impl<T: Add<T,T>> Iterator<T> for Fibonacci<T> {
    #[inline]
    fn next(&mut self) -> Option<T> {
        let new_next    = self.current + self.next;
        let new_current = util::replace(&mut self.next, new_next);
        let retval = util::replace(&mut self.current,   new_current);
        Some(retval)
    }
}


pub fn triangle<T: One + Add<T, T>>() -> Triangle<T> {
    let one: T = One::one();
    Triangle { diff: one + one, next: one }
}

struct Triangle<T> { diff: T, next: T }

impl<T: Add<T, T> + One> Iterator<T> for Triangle<T> {
    #[inline]
    fn next(&mut self) -> Option<T> {
        let new_next = self.next + self.diff;
        self.diff = self.diff + One::one();
        Some(util::replace(&mut self.next, new_next))
    }
}

pub fn prim_pythagorean(m: uint) -> PrimPythagoreanIterator {
    let n0 = if m.is_even() { 1 } else { 2 };
    PrimPythagoreanIterator { m: m, n: n0 }
}

struct PrimPythagoreanIterator { priv m: uint, priv n: uint }

impl Iterator<(uint, uint, uint)> for PrimPythagoreanIterator {
    fn next(&mut self) -> Option<(uint, uint, uint)> {
        let m = self.m;
        while self.n < m {
            let n = self.n;
            self.n += 2;

            if m.gcd(&n) != 1 { continue }

            let (m2, n2)  = (m * m, n * n);
            let (a, b, c) = (m2 - n2, 2 * m * n, m2 + n2);
            if a < b {
                return(Some((a, b, c)))
            } else {
                return(Some((b, a, c)))
            }
        };
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibonacci() {
        let fib = ~[ 1u, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233 ];
        assert_eq!(fibonacci::<uint>().take(fib.len()).to_owned_vec(), fib);

        let fib = ~[ 0u, 0, 0, 0, 0, 0, 0];
        assert_eq!(fibonacci_with_init(0u, 0).take(fib.len()).to_owned_vec(), fib);

        let fib = ~[ 1u, 5, 6, 11, 17, 28, 45, 73, 118, 191, 309, 500];
        assert_eq!(fibonacci_with_init(1u, 5).take(fib.len()).to_owned_vec(), fib);
    }

    #[test]
    fn test_triangle() {
        let tri = ~[1u, 3, 6, 10, 15, 21];
        let gen = triangle::<uint>().take(tri.len()).to_owned_vec();
        assert_eq!(gen, tri);
    }

    #[test]
    fn test_prim_pythagorean_iterator() {
        fn check(m: uint, v: ~[(uint, uint, uint)]) {
            assert_eq!(prim_pythagorean(m).to_owned_vec(), v);
        }

        check(2, ~[(3, 4, 5)]);
        check(3, ~[(5, 12, 13)]);
        check(4, ~[(8, 15, 17), (7, 24, 25)]);
    }
}
