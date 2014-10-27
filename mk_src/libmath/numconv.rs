use std::num::Zero;
use num::Integer;

#[deriving(Clone)]
pub struct DigitIterator { num: uint, radix: uint, order: uint }

pub fn to_digits(num: uint, radix: uint) -> DigitIterator {
    let mut order;
    if num.is_zero() {
        order = 0u;
    } else {
        order = 1u;
        while order * radix <= num {
            order *= radix;
        }
    }
    DigitIterator { num: num, radix: radix, order: order }
}

impl Iterator<uint> for DigitIterator {
    #[inline]
    fn next(&mut self) -> Option<uint> {
        if self.order.is_zero() { return None; }
        let (d, r) = self.num.div_rem(&self.radix);
        self.num    = d;
        self.order /= self.radix;
        Some(r)
    }
}

impl DoubleEndedIterator<uint> for DigitIterator {
    #[inline]
    fn next_back(&mut self) -> Option<uint> {
        if self.order.is_zero() { return None; }
        let (d, r) = self.num.div_rem(&self.order);
        self.num    = r;
        self.order /= self.radix;
        Some(d)
    }
}

pub fn from_digits(v: &[uint], radix: uint) -> uint {
    v.iter().rev().fold(0, |accum, &n| accum * radix + n)
}

pub fn to_digit_histogram(n: uint) -> [uint, ..10] {
    let mut hist = [0, ..10];
    for i in to_digits(n, 10) {
        hist[i] += 1;
    }
    hist
}

pub fn is_palindromic(n: uint, radix: uint) -> bool {
    let mut digits = to_digits(n, radix);
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

#[cfg(test)]
mod tests {
    #[test]
    fn test_conv_digits() {
        fn check(n: uint, v: &[uint], radix: uint) {
            assert_eq!(super::from_digits(v, radix), n);
            assert_eq!(super::to_digits(n, radix).collect::<Vec<uint>>().as_slice(), v);
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
    fn test_from_digits() {
        assert_eq!(super::from_digits(&[0, 0, 1, 2, 3], 10), 32100);
        assert_eq!(super::from_digits(&[1, 2, 3, 0, 0], 10), 321);
    }

    #[test]
    fn test_to_digit_histogram() {
        assert_eq!(super::to_digit_histogram(123).as_slice(),
                   [0, 1, 1, 1, 0, 0, 0, 0, 0, 0].as_slice());
        assert_eq!(super::to_digit_histogram(111).as_slice(),
                   [0, 3, 0, 0, 0, 0, 0, 0, 0, 0].as_slice());
        assert_eq!(super::to_digit_histogram(0).as_slice(),
                   [0, 0, 0, 0, 0, 0, 0, 0, 0, 0].as_slice());
    }

    #[test]
    fn test_is_palindromic() {
        assert!(super::is_palindromic(0, 10));
        assert!(super::is_palindromic(1, 10));
        assert!(super::is_palindromic(9, 10));
        assert!(super::is_palindromic(11, 10));
        assert!(super::is_palindromic(121, 10));
        assert!(!super::is_palindromic(123, 10));
        assert!(super::is_palindromic(1221, 10));
        assert!(super::is_palindromic(12321, 10));
    }
}
