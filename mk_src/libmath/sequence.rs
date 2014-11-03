use std::mem;
use std::num::One;
use num::Integer;

pub fn triangle<T: One + Add<T, T>>() -> Triangle<T> {
    let one: T = One::one();
    Triangle { diff: one + one, next: one }
}

pub struct Triangle<T> { diff: T, next: T }

impl<T: Add<T, T> + One> Iterator<T> for Triangle<T> {
    #[inline]
    fn next(&mut self) -> Option<T> {
        let new_next = self.next + self.diff;
        self.diff = self.diff + One::one();
        Some(mem::replace(&mut self.next, new_next))
    }
}

pub fn prim_pythagorean(m: uint) -> PrimPythagoreanIterator {
    let n0 = if m.is_even() { 1 } else { 2 };
    PrimPythagoreanIterator { m: m, n: n0 }
}

pub struct PrimPythagoreanIterator { m: uint, n: uint }

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
                return Some((a, b, c))
            } else {
                return Some((b, a, c))
            }
        };
        None
    }
}

#[cfg(test)]
mod tests {
    use std::fmt;
    fn check<T: Eq + fmt::Show, I: Iterator<T>>(expected: &[T], mut it: I) {
        assert_eq!(expected, it.collect::<Vec<_>>().as_slice());
    }

    #[test]
    fn test_triangle() {
        let tri = &[1u, 3, 6, 10, 15, 21];
        check(tri, super::triangle::<uint>().take(tri.len()));
    }

    #[test]
    fn test_prim_pythagorean_iterator() {
        fn check(m: uint, v: &[(uint, uint, uint)]) {
            assert_eq!(super::prim_pythagorean(m).collect::<Vec<_>>().as_slice(), v);
        }

        check(2, [(3, 4, 5)]);
        check(3, [(5, 12, 13)]);
        check(4, [(8, 15, 17), (7, 24, 25)]);
    }
}
