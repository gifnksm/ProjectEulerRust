//! Iterators representing mathematical sequences.

#![warn(unused, bad_style,
        missing_doc, unnecessary_qualification, unnecessary_typecast,
        unused_result)]

extern crate num;

use std::mem;
use std::num::One;
use num::Integer;

/// Fibonacci sequence iterator.
pub struct Fibonacci<T> { current: T, next: T }

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
    /// assert_eq!(Some(1u), it.next());
    /// assert_eq!(Some(1u), it.next());
    /// assert_eq!(Some(2u), it.next());
    /// assert_eq!(Some(3u), it.next());
    /// assert_eq!(Some(5u), it.next());
    /// assert_eq!(Some(8u), it.next());
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
    /// let mut it = Fibonacci::with_init(4u, 2u);
    /// assert_eq!(Some(4u), it.next());
    /// assert_eq!(Some(2u), it.next());
    /// assert_eq!(Some(6u), it.next());
    /// assert_eq!(Some(8u), it.next());
    /// assert_eq!(Some(14u), it.next());
    /// assert_eq!(Some(22u), it.next());
    /// ```
    #[inline]
    pub fn with_init(a0: T, a1: T) -> Fibonacci<T> {
        Fibonacci { current: a0, next: a1 }
    }
}

impl<T: Add<T, T>> Iterator<T> for Fibonacci<T> {
    #[inline]
    fn next(&mut self) -> Option<T> {
        let new_next    = self.current + self.next;
        let new_current = mem::replace(&mut self.next,    new_next);
        let retval      = mem::replace(&mut self.current, new_current);
        Some(retval)
    }
}

/// Collatz sequence iterator.
pub struct Collatz<T> { next: T }

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
    /// let mut it = Collatz::new(13u);
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
    pub fn new(init: T) -> Collatz<T> { Collatz { next: init } }
}

impl <T: Integer> Iterator<T> for Collatz<T> {
    #[inline]
    fn next(&mut self) -> Option<T> {
        let one: T   = One::one();
        let two: T   = one + one;
        let three: T = two + one;
        let next = if self.next.is_even() {
            self.next / two
        } else {
            three * self.next + one
        };
        Some(mem::replace(&mut self.next, next))
    }
}

/// Triangular numbers sequence iterator.
pub struct TriangularNums<T> { diff: T, next: T }

impl<T: One + Add<T, T>> TriangularNums<T> {
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
    /// let mut it = TriangularNums::<uint>::new();
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
        TriangularNums { diff: one + one, next: one }
    }
}

impl<T: Add<T, T> + One> Iterator<T> for TriangularNums<T> {
    #[inline]
    fn next(&mut self) -> Option<T> {
        let new_next = self.next + self.diff;
        self.diff = self.diff + One::one();
        Some(mem::replace(&mut self.next, new_next))
    }
}

#[cfg(test)]
mod tests {
    use std::num::One;
    use std::fmt::Show;
    use num::bigint::ToBigInt;

    fn check<T: Eq + Show, I: Iterator<T>>(expected: &[T], mut it: I) {
        assert_eq!(expected, it.collect::<Vec<_>>().as_slice());
    }

    #[test]
    fn test_fibonacci() {
        use super::Fibonacci;

        fn check_with_init<T: Clone + Eq + Show + One + Add<T, T>>(fib: &[T]) {
            let a0 = fib[0].clone();
            let a1 = fib[1].clone();
            check(fib, Fibonacci::with_init(a0, a1).take(fib.len()));
        }

        let fib = &[ 1u, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233 ];
        check(fib, Fibonacci::<uint>::new().take(fib.len()));

        check_with_init(&[ 0u, 0, 0, 0, 0, 0, 0]);
        check_with_init(&[ 1u, 5, 6, 11, 17, 28, 45, 73, 118, 191, 309, 500]);
        check_with_init(&[ -1i, -1, -2, -3, -5, -8, -13, -21, -34, -55, -89, -144, -233 ]);
        check_with_init(&[ -10i.to_bigint().unwrap(),  8i.to_bigint().unwrap(),
                            -2i.to_bigint().unwrap(), 6i.to_bigint().unwrap(),
                            4i.to_bigint().unwrap(), 10i.to_bigint().unwrap(),
                            14i.to_bigint().unwrap(), 24i.to_bigint().unwrap(),
                            38i.to_bigint().unwrap(), 62i.to_bigint().unwrap() ]);
    }
}
