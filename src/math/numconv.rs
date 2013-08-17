use std::num::Zero;

#[deriving(Clone)]
struct DigitIterator { num: uint, radix: uint, order: uint }

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
    v.rev_iter().fold(0, |accum, &n| accum * radix + n)
}

pub fn to_digit_histogram(n: uint) -> [uint, ..10] {
    let mut hist = [0, ..10];
    for i in to_digits(n, 10) {
        hist[i] += 1;
    }
    hist
}

pub fn to_palindromic(n: uint, radix: uint, dup_flag: bool) -> uint {
    let digits = to_digits(n, radix);
    let mut rv = digits.invert();
    if dup_flag { rv.next_back(); }
    return rv.chain(digits).fold(0, |sum, i| sum * radix + i);
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
mod test {
    use super::*;

    #[test]
    fn test_conv_digits() {
        fn check(n: uint, v: ~[uint], radix: uint) {
            assert_eq!(from_digits(v, radix), n);
            assert_eq!(to_digits(n, radix).to_owned_vec(), v.clone());
            assert_eq!(to_digits(n, radix).invert().to_owned_vec(),
                       v.move_rev_iter().to_owned_vec())
        }

        check(0, ~[], 10);
        check(1, ~[1], 10);
        check(3, ~[3], 10);
        check(12345, ~[5, 4, 3, 2, 1], 10);
        check(0x12345,  ~[5, 4, 3, 2, 1], 16);
        check(0xabcde, ~[0xe, 0xd, 0xc, 0xb, 0xa], 16);
        check(0b11100, ~[0, 0, 1, 1, 1], 2);
    }

    #[test]
    fn test_from_digits() {
        assert_eq!(from_digits(&[0, 0, 1, 2, 3], 10), 32100);
        assert_eq!(from_digits(&[1, 2, 3, 0, 0], 10), 321);
    }

    #[test]
    fn test_to_digit_histogram() {
        assert_eq!(to_digit_histogram(123), [0, 1, 1, 1, 0, 0, 0, 0, 0, 0]);
        assert_eq!(to_digit_histogram(111), [0, 3, 0, 0, 0, 0, 0, 0, 0, 0]);
        assert_eq!(to_digit_histogram(0), [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    }

    #[test]
    fn test_to_palindromic() {
        fn check(n: uint, radix: uint, is_dup: bool, result: uint) {
            let ret = to_palindromic(n, radix, is_dup);
            assert_eq!(ret, result);
            assert!(is_palindromic(ret, radix));
        }

        check(10, 10, true,  101);
        check(10, 10, false, 1001);

        check(999, 10, true,  99999);
        check(999, 10, false, 999999);

        check(990, 10, true,  99099);
        check(990, 10, false, 990099);

        check(1100, 10, true,  1100011);
        check(1100, 10, false, 11000011);

        check(0xabc, 16, true,  0xabcba);
        check(0xabc, 16, false, 0xabccba);
    }

    #[test]
    fn test_is_palindromic() {
        assert!(is_palindromic(0, 10));
        assert!(is_palindromic(1, 10));
        assert!(is_palindromic(9, 10));
        assert!(is_palindromic(11, 10));
        assert!(is_palindromic(121, 10));
        assert!(!is_palindromic(123, 10));
        assert!(is_palindromic(1221, 10));
        assert!(is_palindromic(12321, 10));
    }
}
