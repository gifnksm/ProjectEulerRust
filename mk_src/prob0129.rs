#![crate_name = "prob0129"]
#![crate_type = "rlib"]

extern crate num;

use std::iter;
use num::Integer;

pub static EXPECTED_ANSWER: &'static str = "1000023";

#[inline]
pub fn a(n: uint) -> uint {
    if n == 1 { return 1 }

    iter::Unfold::new((1, 1), |st| {
            let (x, k) = *st;
            *st = ((x * 10 + 1) % n, k + 1);
            Some((x, k))
        }).find(|&(x, _)| x == 0)
        .unwrap()
        .val1()
}

pub fn solve() -> String {
    let limit = 1000001u;

    iter::count(limit, 2)
        .filter(|&n| !n.is_multiple_of(&5))
        .find(|&n| a(n) >= limit)
        .unwrap()
        .to_string()
}

#[cfg(test)]
mod tests {
    use std::iter;
    use num::Integer;

    mod naive {
        use std::iter;
        use std::num::{Zero, One};
        use num::Integer;
        use num::bigint::BigUint;

        pub fn r(k: uint) -> BigUint {
            let mut r: BigUint = Zero::zero();
            let ten = FromPrimitive::from_uint(10).unwrap();
            let one = One::one();
            for _ in range(0, k) {
                r = r * ten + one;
            }
            r
        }

        pub fn a(n: uint) -> uint {
            let n = FromPrimitive::from_uint(n).unwrap();
            iter::count(1u, 1)
                .find(|&k| r(k).is_multiple_of(&n))
                .unwrap()
        }
    }

    #[test]
    fn naive_r() {
        assert_eq!("1".to_string(), naive::r(1).to_string());
        assert_eq!("11".to_string(), naive::r(2).to_string());
        assert_eq!("111".to_string(), naive::r(3).to_string());
    }

    #[test]
    fn naive_a() {
        assert_eq!(6, naive::a(7));
        assert_eq!(5, naive::a(41));
    }

    #[test]
    fn cmp_with_naive() {
        for n in iter::range_step(1u, 100u, 2u) {
            if n.is_multiple_of(&5) { continue; }
            assert_eq!(naive::a(n), super::a(n));
        }
    }

    #[test]
    fn a() {
        assert_eq!(6, super::a(7));
        assert_eq!(5, super::a(41));
    }
}
