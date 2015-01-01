//! Integer operations and traits.

#![warn(bad_style, missing_docs,
        unused, unused_extern_crates, unused_import_braces,
        unused_qualifications, unused_results, unused_typecasts)]

extern crate num;

use num::{One, Zero};

/// Extension methods for num::Integer trait.
pub trait Integer: num::Integer + Clone + FromPrimitive + ToPrimitive {

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
        let div = self.clone() / other.clone();
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
        if rem.clone() + rem.clone() < other.clone() {
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

    /// Creates a histogram of the number.
    ///
    /// # Example
    ///
    /// ```rust
    /// use integer::Integer;
    /// assert!([0, 1, 1, 1, 0, 0, 0, 0, 0, 0] == 123u.into_digit_histogram());
    /// assert!([0, 3, 0, 0, 0, 0, 0, 0, 0, 0] == 111u.into_digit_histogram());
    /// assert!([0, 0, 0, 0, 0, 0, 0, 0, 0, 0] == 0u.into_digit_histogram());
    /// ```
    #[inline]
    fn into_digit_histogram(self) -> [u32, .. 10] {
        let mut hist = [0, .. 10];
        let ten = FromPrimitive::from_u32(10).unwrap();
        for d in self.into_digits(ten) {
            hist[d.to_uint().unwrap()] += 1;
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
    /// assert_eq!(321, Integer::from_digits(vec![1, 2, 3].into_iter(), 10i));
    /// assert_eq!(0x321, Integer::from_digits(vec![1, 2, 3].into_iter(), 16i));
    /// assert_eq!(0, Integer::from_digits(vec![].into_iter(), 10i));
    /// ```
    #[inline]
    fn from_digits<T: Iterator<Self>>(mut digits: T, radix: Self) -> Self {
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
    /// assert_eq!(12321, 123i.into_palindromic(10, false));
    /// assert_eq!(123321, 123i.into_palindromic(10, true));
    /// ```
    #[inline]
    fn into_palindromic(self, radix: Self, duplicate_middle: bool) -> Self {
        let digits = self.into_digits(radix.clone());
        let mut rv = digits.clone().rev();
        if !duplicate_middle { let _ = rv.next_back(); }
        rv.chain(digits).fold(Zero::zero(), |sum: Self, i| sum * radix.clone() + i)
    }

    /// Returns `true` if the number is palindromic.
    ///
    /// # Example
    ///
    /// ```
    /// use integer::Integer;
    ///
    /// assert_eq!(true, 12321i.is_palindromic(10));
    /// assert_eq!(false, 12345i.is_palindromic(10));
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
                Equal => return mid,
                Greater => { max = mid - one.clone() }
                Less    => { min = mid }
            }
        }

        return min
    }

    /// Gets the factorial of the number.
    ///
    /// # Example
    ///
    /// ```rust
    /// use integer::Integer;
    ///
    /// assert_eq!(1u, 0u.factorial());
    /// assert_eq!(1u, 1u.factorial());
    /// assert_eq!(2u, 2u.factorial());
    /// assert_eq!(6u, 3u.factorial());
    /// assert_eq!(24u, 4u.factorial());
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
        let one: Self  = One::one();
        let two: Self  = one.clone() + one.clone();
        if *self == zero { return zero }

        let mut result = one.clone();
        let mut base   = self.clone();
        let mut exp    = exp.clone();
        let     modulo = modulo.clone();

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

impl<T: num::Integer + Clone> Digits<T> {
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
        Digits { num: num, radix: radix, order: order }
    }
}

impl<T: num::Integer + Clone> Iterator<T> for Digits<T> {
    #[inline]
    fn next(&mut self) -> Option<T> {
        if self.order.is_zero() { return None; }
        let (d, r) = self.num.div_rem(&self.radix);
        self.num   = d;
        self.order = self.order.clone() / self.radix.clone();
        Some(r)
    }
}

impl<T: num::Integer + Clone> DoubleEndedIterator<T> for Digits<T> {
    #[inline]
    fn next_back(&mut self) -> Option<T> {
        if self.order.is_zero() { return None; }
        let (d, r) = self.num.div_rem(&self.order);
        self.num   = r;
        self.order = self.order.clone() / self.radix.clone();
        Some(d)
    }
}

#[cfg(test)]
mod tests {
    use super::Integer;
    use num;
    use num::Integer as NumInteger;

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
        fn check(n: u32, v: &[u32], radix: u32) {
            assert_eq!(v, n.into_digits(radix).collect::<Vec<_>>()[]);
            let mut rev = n.into_digits(radix).rev().collect::<Vec<_>>();
            rev.reverse();
            assert_eq!(v, rev[])
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

    #[test]
    fn is_palindromic() {
        assert!(0i.is_palindromic(10));
        assert!(1i.is_palindromic(10));
        assert!(9i.is_palindromic(10));
        assert!(11i.is_palindromic(10));
        assert!(121i.is_palindromic(10));
        assert!(!123i.is_palindromic(10));
        assert!(1221i.is_palindromic(10));
        assert!(12321i.is_palindromic(10));
    }

    #[test]
    fn mod_pow() {
        for b in range(1u, 10) {
            for e in range(0u, 5) {
                for r in range(10u, 100) {
                    assert_eq!(num::pow(b, e) % r, b.mod_pow(&e, &r));
                }
            }
        }
    }
}
