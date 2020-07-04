//! [Problem 130](https://projecteuler.net/problem=130) solver.

#![warn(
    bad_style,
    unused,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

use num_integer::Integer;
use prime::PrimeSet;

fn a(n: u64) -> u64 {
    if n == 1 {
        return 1;
    }

    let mut x = 1;
    let mut k = 1;
    loop {
        x = (x * 10 + 1) % n;
        k += 1;
        if x == 0 {
            return k;
        }
    }
}

fn solve() -> String {
    let ps = PrimeSet::new();
    (3..)
        .step_by(2)
        .filter(|&n| !n.is_multiple_of(&5))
        .filter(|&n| !ps.contains(n))
        .filter(|&n| (n - 1).is_multiple_of(&a(n)))
        .take(25)
        .sum::<u64>()
        .to_string()
}

common::problem!("149253", solve);

#[cfg(test)]
mod tests {
    use num_integer::Integer;
    use prime::PrimeSet;

    mod naive {
        use num_bigint::BigUint;
        use num_integer::Integer;
        use num_traits::{FromPrimitive, One, Zero};

        pub fn r(k: u64) -> BigUint {
            let mut r: BigUint = Zero::zero();
            let ten: BigUint = FromPrimitive::from_u64(10).unwrap();
            let one: BigUint = One::one();
            for _ in 0..k {
                r = &r * &ten + &one;
            }
            r
        }

        pub fn a(n: u64) -> u64 {
            let n = FromPrimitive::from_u64(n).unwrap();
            (1..).find(|&k| r(k).is_multiple_of(&n)).unwrap()
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
        for n in (1..100).step_by(2) {
            if n.is_multiple_of(&5) {
                continue;
            }
            assert_eq!(naive::a(n), super::a(n));
        }
    }

    #[test]
    fn a() {
        assert_eq!(6, super::a(7));
        assert_eq!(5, super::a(41));
    }

    #[test]
    fn first5() {
        let ps = PrimeSet::new();
        let mut it = (3..)
            .step_by(2)
            .filter(|&n| !n.is_multiple_of(&5))
            .filter(|&n| !ps.contains(n))
            .filter(|&n| (n - 1).is_multiple_of(&super::a(n)));

        assert_eq!(Some(91), it.next());
        assert_eq!(Some(259), it.next());
        assert_eq!(Some(451), it.next());
        assert_eq!(Some(481), it.next());
        assert_eq!(Some(703), it.next());
    }
}
