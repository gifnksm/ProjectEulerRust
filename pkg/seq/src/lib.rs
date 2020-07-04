//! Iterators representing mathematical sequences.

#![warn(
    bad_style,
    missing_docs,
    unused,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

use num_integer::Integer;
use num_traits::One;
use std::{mem, ops::Add};

/// Fibonacci sequence iterator.
pub struct Fibonacci<T> {
    current: T,
    next: T,
}

impl<T> Default for Fibonacci<T>
where
    T: One,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T: One> Fibonacci<T> {
    /// Creates a new Fibonacci sequence iterator.
    ///
    /// The reccurence formula for the $$n$$th term of the Fibonacci number
    /// sequence $$a(n)$$:
    ///
    /// * $$ a(0) = 1 $$
    /// * $$ a(1) = 1 $$
    /// * $$ a(n) = a(n - 1) + a(n - 2) $$
    ///
    /// # Example
    ///
    /// ```
    /// use seq::Fibonacci;
    /// let mut it = Fibonacci::new();
    /// assert_eq!(Some(1), it.next());
    /// assert_eq!(Some(1), it.next());
    /// assert_eq!(Some(2), it.next());
    /// assert_eq!(Some(3), it.next());
    /// assert_eq!(Some(5), it.next());
    /// assert_eq!(Some(8), it.next());
    /// ```
    #[inline]
    pub fn new() -> Fibonacci<T> {
        Fibonacci::with_init(One::one(), One::one())
    }

    /// Creates a new Fibonacci sequence iterator with specifying initial two
    /// terms.
    ///
    /// # Example
    ///
    /// ```
    /// use seq::Fibonacci;
    /// let mut it = Fibonacci::with_init(4, 2);
    /// assert_eq!(Some(4), it.next());
    /// assert_eq!(Some(2), it.next());
    /// assert_eq!(Some(6), it.next());
    /// assert_eq!(Some(8), it.next());
    /// assert_eq!(Some(14), it.next());
    /// assert_eq!(Some(22), it.next());
    /// ```
    #[inline]
    pub fn with_init(a0: T, a1: T) -> Fibonacci<T> {
        Fibonacci {
            current: a0,
            next: a1,
        }
    }
}

impl<T: Add<T, Output = T> + Clone> Iterator for Fibonacci<T> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<T> {
        let new_next = self.current.clone() + self.next.clone();
        let new_current = mem::replace(&mut self.next, new_next);
        let retval = mem::replace(&mut self.current, new_current);
        Some(retval)
    }
}

/// Collatz sequence iterator.
pub struct Collatz<T> {
    next: T,
}

impl<T> Collatz<T> {
    /// Creates a new Collatz sequence iterator starting from the `init`.
    ///
    /// The reccurence formula for the $$n$$th term of the Collatz number
    /// sequence $$a(n)$$ with initial value $$k$$:
    ///
    /// * $$ a(0) = k $$
    /// * $$ a(n) = \frac{a(n - 1)}{2}$$ if $$a(n-1)$$ is even
    /// * $$ a(n) = 3a(n - 1) - 1 $$ if $$a(n-1)$$ is odd
    ///
    /// # Example
    ///
    /// ```
    /// use seq::Collatz;
    ///
    /// let mut it = Collatz::new(13);
    /// assert_eq!(Some(13), it.next());
    /// assert_eq!(Some(40), it.next());
    /// assert_eq!(Some(20), it.next());
    /// assert_eq!(Some(10), it.next());
    /// assert_eq!(Some(5),  it.next());
    /// assert_eq!(Some(16), it.next());
    /// assert_eq!(Some(8),  it.next());
    /// assert_eq!(Some(4),  it.next());
    /// assert_eq!(Some(2),  it.next());
    /// assert_eq!(Some(1),  it.next());
    /// ```
    #[inline]
    pub fn new(init: T) -> Collatz<T> {
        Collatz { next: init }
    }
}

impl<T: Integer + Clone> Iterator for Collatz<T> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<T> {
        let one: T = One::one();
        let two: T = one.clone() + one.clone();
        let three: T = two.clone() + one.clone();
        let next = if self.next.is_even() {
            self.next.clone() / two
        } else {
            three * self.next.clone() + one
        };
        Some(mem::replace(&mut self.next, next))
    }
}

/// Triangular numbers sequence iterator.
pub struct TriangularNums<T> {
    diff: T,
    next: T,
}

impl<T> Default for TriangularNums<T>
where
    T: One + Add<T, Output = T> + Clone,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T: One + Add<T, Output = T> + Clone> TriangularNums<T> {
    /// Creates a new Triangular number sequence iterator that enumerates each
    /// triangular number.
    ///
    /// The reccurence formula for the $$n$$th term of the triangular number
    /// sequence $$a(n)$$:
    ///
    /// * $$ a(0) = 1 $$
    /// * $$ a(n) = a(n - 1) + n + 1 $$
    ///
    /// # Example
    ///
    /// ```
    /// use seq::TriangularNums;
    ///
    /// let mut it = TriangularNums::<u32>::new();
    /// assert_eq!(Some(1), it.next());
    /// assert_eq!(Some(3), it.next());
    /// assert_eq!(Some(6), it.next());
    /// assert_eq!(Some(10), it.next());
    /// assert_eq!(Some(15), it.next());
    /// assert_eq!(Some(21), it.next());
    /// ```
    #[inline]
    pub fn new() -> TriangularNums<T> {
        let one: T = One::one();
        TriangularNums {
            diff: one.clone() + one.clone(),
            next: one,
        }
    }
}

impl<T: Add<T, Output = T> + One + Clone> Iterator for TriangularNums<T> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<T> {
        let new_next = self.next.clone() + self.diff.clone();
        self.diff = self.diff.clone() + One::one();
        Some(mem::replace(&mut self.next, new_next))
    }
}

/// Pritmitive Pythagorean numbers iterator.
pub struct PrimitivePythagoreans<T> {
    m: T,
    n: T,
}

impl<T: Integer + Clone> PrimitivePythagoreans<T> {
    /// Creates a new Primitive Pythagorean number iterator that enumerates each
    /// primitive Pythagorean triples `(a, b, c)` that are generated by
    /// following formula.
    ///
    /// ```math
    /// a = \min(m^2 - n^2, 2mn)
    /// b = \max(m^2 - n^2, 2mn)
    /// c = m^n + n^2
    /// ```
    ///
    /// # Example
    ///
    /// ```
    /// use seq::PrimitivePythagoreans;
    ///
    /// let mut it = PrimitivePythagoreans::<u32>::new(5);
    /// assert_eq!(Some((20, 21, 29)), it.next()); // n == 2
    /// assert_eq!(Some(( 9, 40, 41)), it.next()); // n == 4
    /// assert_eq!(None, it.next());
    /// ```
    pub fn new(m: T) -> PrimitivePythagoreans<T> {
        let one: T = One::one();
        let n = if m.is_even() { one } else { one.clone() + one };
        PrimitivePythagoreans { m, n }
    }
}

impl<T: Integer + Clone> Iterator for PrimitivePythagoreans<T> {
    type Item = (T, T, T);

    #[allow(clippy::many_single_char_names)]
    fn next(&mut self) -> Option<(T, T, T)> {
        let one: T = One::one();
        let two = one.clone() + one.clone();

        let m = &self.m;
        while self.n < *m {
            let n = {
                let new_n = self.n.clone() + two.clone();
                mem::replace(&mut self.n, new_n)
            };

            if m.gcd(&n) != one {
                continue;
            }

            let (m2, n2) = (m.clone() * m.clone(), n.clone() * n.clone());
            let (a, b, c) = (m2.clone() - n2.clone(), two * m.clone() * n, m2 + n2);
            if a < b {
                return Some((a, b, c));
            } else {
                return Some((b, a, c));
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use num_bigint::ToBigInt;
    use num_traits::One;
    use std::{fmt::Debug, ops::Add};

    fn check<T: Eq + Debug, I: Iterator<Item = T>>(expected: &[T], it: I) {
        assert_eq!(expected, &it.collect::<Vec<_>>()[..]);
    }

    #[test]
    fn fibonacci() {
        use super::Fibonacci;

        fn check_with_init<T: Clone + Eq + Debug + One + Add<T, Output = T>>(fib: &[T]) {
            let a0 = fib[0].clone();
            let a1 = fib[1].clone();
            check(fib, Fibonacci::with_init(a0, a1).take(fib.len()));
        }

        let fib = &[1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233];
        check(fib, Fibonacci::<u32>::new().take(fib.len()));

        check_with_init(&[0, 0, 0, 0, 0, 0, 0]);
        check_with_init(&[1, 5, 6, 11, 17, 28, 45, 73, 118, 191, 309, 500]);
        check_with_init(&[-1, -1, -2, -3, -5, -8, -13, -21, -34, -55, -89, -144, -233]);
        check_with_init(&[
            -10.to_bigint().unwrap(),
            8.to_bigint().unwrap(),
            -2.to_bigint().unwrap(),
            6.to_bigint().unwrap(),
            4.to_bigint().unwrap(),
            10.to_bigint().unwrap(),
            14.to_bigint().unwrap(),
            24.to_bigint().unwrap(),
            38.to_bigint().unwrap(),
            62.to_bigint().unwrap(),
        ]);
    }

    #[test]
    fn prim_pythagorean() {
        use super::PrimitivePythagoreans;

        fn check(m: u32, v: &[(u32, u32, u32)]) {
            assert_eq!(PrimitivePythagoreans::new(m).collect::<Vec<_>>(), v);
        }

        check(2, &[(3, 4, 5)]);
        check(3, &[(5, 12, 13)]);
        check(4, &[(8, 15, 17), (7, 24, 25)]);
    }
}
