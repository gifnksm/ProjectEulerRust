//! Integer operations and traits.

#![warn(
    bad_style,
    missing_docs,
    unused,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

#[cfg(feature = "num-bigint")]
use num_bigint::{BigInt, BigUint};
use num_traits::{FromPrimitive, One, ToPrimitive, Zero};
use std::cmp::Ordering;

/// Extension methods for num::Integer trait.
pub trait Integer: num_integer::Integer + Clone + FromPrimitive + ToPrimitive {
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
        if rem.clone() + rem < other.clone() {
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
    /// let mut it = 12345.into_digits(10);
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

    /// Creates a histogram of the number.
    ///
    /// # Example
    ///
    /// ```rust
    /// use integer::Integer;
    /// assert!([0, 1, 1, 1, 0, 0, 0, 0, 0, 0] == 123.into_digit_histogram());
    /// assert!([0, 3, 0, 0, 0, 0, 0, 0, 0, 0] == 111.into_digit_histogram());
    /// assert!([0, 0, 0, 0, 0, 0, 0, 0, 0, 0] == 0.into_digit_histogram());
    /// ```
    #[inline]
    fn into_digit_histogram(self) -> [u32; 10] {
        let mut hist = [0; 10];
        let ten = FromPrimitive::from_u32(10).unwrap();
        for d in self.into_digits(ten) {
            hist[d.to_usize().unwrap()] += 1;
        }
        hist
    }

    /// Creates an integer from an iterator to enumerate each digit from the lower.
    ///
    /// # Example
    ///
    /// ```rust
    /// use integer::Integer;
    ///
    /// assert_eq!(321, Integer::from_digits(vec![1, 2, 3].into_iter(), 10));
    /// assert_eq!(0x321, Integer::from_digits(vec![1, 2, 3].into_iter(), 16));
    /// assert_eq!(0, Integer::from_digits(vec![].into_iter(), 10));
    /// ```
    #[inline]
    fn from_digits<T: Iterator<Item = Self>>(digits: T, radix: Self) -> Self {
        let mut result: Self = Zero::zero();
        let mut order: Self = One::one();
        for d in digits {
            result = result + order.clone() * d;
            order = order * radix.clone();
        }
        result
    }

    /// Creates a palindromic number from `self`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use integer::Integer;
    ///
    /// assert_eq!(12321, 123.into_palindromic(10, false));
    /// assert_eq!(123321, 123.into_palindromic(10, true));
    /// ```
    #[inline]
    fn into_palindromic(self, radix: Self, duplicate_middle: bool) -> Self {
        let digits = self.into_digits(radix.clone());
        let mut rv = digits.clone().rev();
        if !duplicate_middle {
            let _ = rv.next_back();
        }
        rv.chain(digits)
            .fold(Zero::zero(), |sum: Self, i| sum * radix.clone() + i)
    }

    /// Returns `true` if the number is palindromic.
    ///
    /// # Example
    ///
    /// ```
    /// use integer::Integer;
    ///
    /// assert_eq!(true, 12321.is_palindromic(10));
    /// assert_eq!(false, 12345.is_palindromic(10));
    /// ```
    fn is_palindromic(self, radix: Self) -> bool {
        let mut digits = self.into_digits(radix);
        loop {
            let next = digits.next();
            let next_back = digits.next_back();
            if next.is_none() || next_back.is_none() {
                return true;
            }
            if next != next_back {
                return false;
            }
        }
    }

    /// Takes the square root of the number.
    #[inline]
    fn sqrt(&self) -> Self {
        let one: Self = One::one();
        let two: Self = one.clone() + one.clone();
        let mut min: Self = Zero::zero();
        let mut max: Self = self.clone();

        while min < max {
            let mid = (min.clone() + max.clone() + one.clone()) / two.clone();
            let mid2 = mid.clone() * mid.clone();
            match mid2.partial_cmp(self).unwrap() {
                Ordering::Equal => return mid,
                Ordering::Greater => max = mid - one.clone(),
                Ordering::Less => min = mid,
            }
        }

        min
    }

    /// Gets the factorial of the number.
    ///
    /// # Example
    ///
    /// ```rust
    /// use integer::Integer;
    ///
    /// assert_eq!(1, 0.factorial());
    /// assert_eq!(1, 1.factorial());
    /// assert_eq!(2, 2.factorial());
    /// assert_eq!(6, 3.factorial());
    /// assert_eq!(24, 4.factorial());
    /// ```
    fn factorial(&self) -> Self {
        assert!(*self >= Zero::zero());

        let mut p: Self = One::one();
        let mut i: Self = One::one();
        while i <= *self {
            p = p * i.clone();
            i = i + One::one();
        }
        p
    }

    /// Takes the modular exponentation of the number.
    fn mod_pow(&self, exp: &Self, modulo: &Self) -> Self {
        let zero = Zero::zero();
        let one: Self = One::one();
        let two: Self = one.clone() + one.clone();
        if *self == zero {
            return zero;
        }

        let mut result = one;
        let mut base = self.clone();
        let mut exp = exp.clone();
        let modulo = modulo.clone();

        while exp > zero {
            if exp.is_odd() {
                result = (result * base.clone()) % modulo.clone();
            }
            exp = exp / two.clone();
            base = (base.clone() * base) % modulo.clone();
        }
        result
    }
}

#[cfg(feature = "num-bigint")]
impl Integer for BigUint {}
#[cfg(feature = "num-bigint")]
impl Integer for BigInt {}

impl Integer for i8 {}
impl Integer for i16 {}
impl Integer for i32 {}
impl Integer for i64 {}
impl Integer for isize {}
impl Integer for u8 {}
impl Integer for u16 {}
impl Integer for u32 {}
impl Integer for u64 {}
impl Integer for usize {}

/// An iterator that enumerates each digit of a number.
#[derive(Clone)]
pub struct Digits<T> {
    num: T,
    radix: T,
    order: T,
}

impl<T: num_integer::Integer + Clone> Digits<T> {
    fn new(num: T, radix: T) -> Digits<T> {
        let mut order: T;
        if num.is_zero() {
            order = Zero::zero();
        } else {
            order = One::one();
            let mut prod = order.clone() * radix.clone();
            while prod <= num {
                order = prod;
                prod = order.clone() * radix.clone();
            }
        }
        Digits { num, radix, order }
    }
}

impl<T: num_integer::Integer + Clone> Iterator for Digits<T> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<T> {
        if self.order.is_zero() {
            return None;
        }
        let (d, r) = self.num.div_rem(&self.radix);
        self.num = d;
        self.order = self.order.clone() / self.radix.clone();
        Some(r)
    }
}

impl<T: num_integer::Integer + Clone> DoubleEndedIterator for Digits<T> {
    #[inline]
    fn next_back(&mut self) -> Option<T> {
        if self.order.is_zero() {
            return None;
        }
        let (d, r) = self.num.div_rem(&self.order);
        self.num = r;
        self.order = self.order.clone() / self.radix.clone();
        Some(d)
    }
}

#[cfg(test)]
mod tests {
    use super::Integer;
    use num_integer::Integer as NumInteger;

    #[test]
    fn div() {
        assert_eq!(0, 0.div_floor(&3));
        assert_eq!(0, 0.div_round(&3));
        assert_eq!(0, 0.div_ceil(&3));

        assert_eq!(0, 1.div_floor(&3));
        assert_eq!(0, 1.div_round(&3));
        assert_eq!(1, 1.div_ceil(&3));

        assert_eq!(0, 2.div_floor(&3));
        assert_eq!(1, 2.div_round(&3));
        assert_eq!(1, 2.div_ceil(&3));

        assert_eq!(1, 3.div_floor(&3));
        assert_eq!(1, 3.div_round(&3));
        assert_eq!(1, 3.div_ceil(&3));

        assert_eq!(1, 4.div_floor(&3));
        assert_eq!(1, 4.div_round(&3));
        assert_eq!(2, 4.div_ceil(&3));

        assert_eq!(1, 5.div_floor(&3));
        assert_eq!(2, 5.div_round(&3));
        assert_eq!(2, 5.div_ceil(&3));

        assert_eq!(3, 5.div_round(&2));
    }

    #[test]
    fn digits() {
        fn check(n: u32, v: &[u32], radix: u32) {
            assert_eq!(v, &n.into_digits(radix).collect::<Vec<_>>()[..]);
            let mut rev = n.into_digits(radix).rev().collect::<Vec<_>>();
            rev.reverse();
            assert_eq!(v, &rev[..])
        }

        check(0, &[], 10);
        check(1, &[1], 10);
        check(3, &[3], 10);
        check(12345, &[5, 4, 3, 2, 1], 10);
        check(0x12345, &[5, 4, 3, 2, 1], 16);
        check(0xabcde, &[0xe, 0xd, 0xc, 0xb, 0xa], 16);
        check(0b11100, &[0, 0, 1, 1, 1], 2);
    }

    #[test]
    fn to_palindromic() {
        assert_eq!(101, 10.into_palindromic(10, false));
        assert_eq!(1001, 10.into_palindromic(10, true));

        assert_eq!(99999, 999.into_palindromic(10, false));
        assert_eq!(999999, 999.into_palindromic(10, true));

        assert_eq!(99099, 990.into_palindromic(10, false));
        assert_eq!(990099, 990.into_palindromic(10, true));

        assert_eq!(1100011, 1100.into_palindromic(10, false));
        assert_eq!(11000011, 1100.into_palindromic(10, true));

        assert_eq!(0xabcba, 0xabc.into_palindromic(16, false));
        assert_eq!(0xabccba, 0xabc.into_palindromic(16, true));
    }

    #[test]
    fn is_palindromic() {
        assert!(0.is_palindromic(10));
        assert!(1.is_palindromic(10));
        assert!(9.is_palindromic(10));
        assert!(11.is_palindromic(10));
        assert!(121.is_palindromic(10));
        assert!(!123.is_palindromic(10));
        assert!(1221.is_palindromic(10));
        assert!(12321.is_palindromic(10));
    }

    #[test]
    fn mod_pow() {
        for b in 1u32..10 {
            for e in 0u32..5 {
                for r in 10u32..100 {
                    assert_eq!(num_traits::pow(b, e as usize) % r, b.mod_pow(&e, &r));
                }
            }
        }
    }
}
