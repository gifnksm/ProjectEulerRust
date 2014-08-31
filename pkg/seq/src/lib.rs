//! Iterators representing mathematical sequences.

#![warn(unused, bad_style,
        missing_doc, unnecessary_qualification, unnecessary_typecast,
        unused_result)]

#[cfg(test)]
extern crate num;

use std::mem;
use std::num::One;

/// Fibonacci sequence iterator.
pub struct Fibonacci<T> { current: T, next: T }

impl<T: One> Fibonacci<T> {
    /// Creates a new `Fibonacci` iterator.
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

    /// Creates a new `Fibonacci` iterator with specifying initial two terms.
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
