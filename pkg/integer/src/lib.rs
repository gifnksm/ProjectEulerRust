//! Integer operations and traits.

#![warn(unused, bad_style,
        missing_doc, unnecessary_qualification, unnecessary_typecast,
        unused_result)]

extern crate num;

use std::num::{One, Zero};

/// Extension methods for num::Integer trait.
pub trait Integer: num::Integer + Clone {

    /// Divide two numbers, return the result, rounded up.
    ///
    /// # Arguments
    ///
    /// * x - an integer
    /// * y - an integer distinct from 0u
    ///
    /// # Return value
    ///
    /// The smallest integer `q` such that `x/y <= q`.
    ///
    fn div_ceil(&self, other: &Self) -> Self {
        let div = *self / *other;
        if self.is_multiple_of(other) {
            div
        } else {
            div + One::one()
        }
    }

    /// Divide two numbers, return the result, rounded to the closest integer.
    ///
    /// # Arguments
    ///
    /// * x - an integer
    /// * y - an integer distinct from 0u
    ///
    /// # Return value
    ///
    /// The integer `q` closest to `x/y`.
    ///
    fn div_round(&self, other: &Self) -> Self {
        let (div, rem) = self.div_rem(other);
        if rem + rem < *other {
            div
        } else {
            div + One::one()
        }
    }

    /// Creates an iterator to enumerate each digit from the lower of the number.
    ///
    /// # Example
    ///
    /// ```rust
    /// use integer::Integer;
    ///
    /// let mut it = 12345i.into_digits(10);
    /// assert_eq!(Some(5), it.next());
    /// assert_eq!(Some(4), it.next());
    /// assert_eq!(Some(3), it.next());
    /// assert_eq!(Some(2), it.next());
    /// assert_eq!(Some(1), it.next());
    /// assert_eq!(None,    it.next());
    /// ```
    #[inline]
    fn into_digits(self, radix: Self) -> Digits<Self> {
        Digits::new(self, radix)
    }

    /// Creates a palindromic number from `self`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use integer::Integer;
    ///
    /// assert_eq!(12321, 123i.into_palindromic(10, false));
    /// assert_eq!(123321, 123i.into_palindromic(10, true));
    /// ```
    #[inline]
    fn into_palindromic(self, radix: Self, duplicate_middle: bool) -> Self {
        let digits = self.into_digits(radix.clone());
        let mut rv = digits.clone().rev();
        if !duplicate_middle { let _ = rv.next_back(); }
        rv.chain(digits).fold(Zero::zero(), |sum: Self, i| sum * radix + i)
    }
}

impl Integer for num::BigUint {}
impl Integer for num::BigInt {}
impl Integer for i8 {}
impl Integer for i16 {}
impl Integer for i32 {}
impl Integer for i64 {}
impl Integer for int {}
impl Integer for u8 {}
impl Integer for u16 {}
impl Integer for u32 {}
impl Integer for u64 {}
impl Integer for uint {}

/// An iterator that enumerates each digit of a number.
#[deriving(Clone)]
pub struct Digits<T> { num: T, radix: T, order: T }

impl<T: num::Integer> Digits<T> {
    fn new(num: T, radix: T) -> Digits<T> {
        let mut order: T;
        if num.is_zero() {
            order = Zero::zero();
        } else {
            order = One::one();
            while order * radix <= num {
                order = order * radix;
            }
        }
        Digits { num: num, radix: radix, order: order }
    }
}

impl<T: num::Integer> Iterator<T> for Digits<T> {
    #[inline]
    fn next(&mut self) -> Option<T> {
        if self.order.is_zero() { return None; }
        let (d, r) = self.num.div_rem(&self.radix);
        self.num   = d;
        self.order = self.order / self.radix;
        Some(r)
    }
}

impl<T: num::Integer> DoubleEndedIterator<T> for Digits<T> {
    #[inline]
    fn next_back(&mut self) -> Option<T> {
        if self.order.is_zero() { return None; }
        let (d, r) = self.num.div_rem(&self.order);
        self.num   = r;
        self.order = self.order / self.radix;
        Some(d)
    }
}

#[cfg(test)]
mod tests {
    use super::Integer;
    use num::Integer as SInteger;

    #[test]
    fn div() {
        assert_eq!(0u, 0u.div_floor(&3));
        assert_eq!(0u, 0u.div_round(&3));
        assert_eq!(0u, 0u.div_ceil(&3));

        assert_eq!(0u, 1u.div_floor(&3));
        assert_eq!(0u, 1u.div_round(&3));
        assert_eq!(1u, 1u.div_ceil(&3));

        assert_eq!(0u, 2u.div_floor(&3));
        assert_eq!(1u, 2u.div_round(&3));
        assert_eq!(1u, 2u.div_ceil(&3));

        assert_eq!(1u, 3u.div_floor(&3));
        assert_eq!(1u, 3u.div_round(&3));
        assert_eq!(1u, 3u.div_ceil(&3));

        assert_eq!(1u, 4u.div_floor(&3));
        assert_eq!(1u, 4u.div_round(&3));
        assert_eq!(2u, 4u.div_ceil(&3));

        assert_eq!(1u, 5u.div_floor(&3));
        assert_eq!(2u, 5u.div_round(&3));
        assert_eq!(2u, 5u.div_ceil(&3));

        assert_eq!(3u, 5u.div_round(&2));
    }

    #[test]
    fn digits() {
        fn check(n: uint, v: &[uint], radix: uint) {
            assert_eq!(v, n.into_digits(radix).collect::<Vec<_>>().as_slice());
            let mut rev = n.into_digits(radix).rev().collect::<Vec<_>>();
            rev.reverse();
            assert_eq!(v, rev.as_slice())
        }

        check(0, [], 10);
        check(1, [1], 10);
        check(3, [3], 10);
        check(12345, [5, 4, 3, 2, 1], 10);
        check(0x12345, [5, 4, 3, 2, 1], 16);
        check(0xabcde, [0xe, 0xd, 0xc, 0xb, 0xa], 16);
        check(0b11100, [0, 0, 1, 1, 1], 2);
    }

    #[test]
    fn to_palindromic() {
        assert_eq!(101, 10i.into_palindromic(10, false));
        assert_eq!(1001, 10i.into_palindromic(10, true));

        assert_eq!(99999, 999i.into_palindromic(10, false));
        assert_eq!(999999, 999i.into_palindromic(10, true));

        assert_eq!(99099, 990i.into_palindromic(10, false));
        assert_eq!(990099, 990i.into_palindromic(10, true));

        assert_eq!(1100011, 1100i.into_palindromic(10, false));
        assert_eq!(11000011, 1100i.into_palindromic(10, true));

        assert_eq!(0xabcba, 0xabci.into_palindromic(16, false));
        assert_eq!(0xabccba, 0xabci.into_palindromic(16, true));
    }
}
