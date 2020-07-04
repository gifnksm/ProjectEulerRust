//! [Problem 112](https://projecteuler.net/problem=112) solver.

#![warn(
    bad_style,
    unused,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

use integer::Integer;
#[cfg(test)]
use std::u32;

fn is_increasing_with<T: Iterator<Item = u32>>(ds: T, mut prev: u32) -> bool {
    for n in ds {
        if n < prev {
            return false;
        }
        prev = n;
    }
    true
}

fn is_decreasing_with<T: Iterator<Item = u32>>(ds: T, mut prev: u32) -> bool {
    for n in ds {
        if n > prev {
            return false;
        }
        prev = n;
    }
    true
}
#[cfg(test)]
fn is_increasing<T: Iterator<Item = u32>>(ds: T) -> bool {
    is_increasing_with(ds, 0)
}
#[cfg(test)]
fn is_decreasing<T: Iterator<Item = u32>>(ds: T) -> bool {
    is_decreasing_with(ds, u32::MAX)
}

fn is_bouncy<T: Iterator<Item = u32>>(mut ds: T) -> bool {
    let prev = match ds.next() {
        Some(x) => x,
        None => return false,
    };
    loop {
        let n = match ds.next() {
            Some(x) => x,
            None => return false,
        };
        if prev < n {
            return !is_increasing_with(ds, n);
        }
        if prev > n {
            return !is_decreasing_with(ds, n);
        }
    }
}

fn compute(percent: u32) -> u32 {
    assert!(percent < 100);
    let mut num_bouncy = 0;
    for n in 1.. {
        if is_bouncy(n.into_digits(10)) {
            num_bouncy += 1;
        }
        if n * percent == 100 * num_bouncy {
            return n;
        }
    }
    unreachable!()
}

fn solve() -> String {
    compute(99).to_string()
}

common::problem!("1587000", solve);

#[cfg(test)]
mod tests {
    mod is_increasing {
        use super::super::is_increasing;

        fn check(result: bool, input: Vec<u32>) {
            assert_eq!(result, is_increasing(input.into_iter()));
        }

        #[test]
        fn empty_is_increasing() {
            check(true, vec![])
        }
        #[test]
        fn one_digit_is_increasing() {
            check(true, vec![1])
        }
        #[test]
        fn increasing_number() {
            check(true, vec![4, 5, 6])
        }
        #[test]
        fn decreasing_number() {
            check(false, vec![5, 4, 3])
        }
        #[test]
        fn bouncy_number() {
            check(false, vec![4, 5, 3])
        }
        #[test]
        fn increasing_with_same_sdigit() {
            check(true, vec![1, 3, 4, 4, 6, 8])
        }
    }

    mod is_decreasing {
        use super::super::is_decreasing;

        fn check(result: bool, input: Vec<u32>) {
            assert_eq!(result, is_decreasing(input.into_iter()));
        }
        #[test]
        fn empty_is_decreasing() {
            check(true, vec![])
        }
        #[test]
        fn one_digit_is_decreasing() {
            check(true, vec![1])
        }
        #[test]
        fn increasing_number() {
            check(false, vec![4, 5, 6])
        }
        #[test]
        fn decreasing_number() {
            check(true, vec![5, 4, 3])
        }
        #[test]
        fn bouncy_number() {
            check(false, vec![4, 5, 3])
        }
        #[test]
        fn decreasing_with_same_digit() {
            check(true, vec![6, 6, 4, 2, 0])
        }
    }

    mod is_bouncy {
        use super::super::is_bouncy;

        fn check(result: bool, input: Vec<u32>) {
            assert_eq!(result, is_bouncy(input.into_iter()));
        }
        #[test]
        fn empty_is_not_bouncy() {
            check(false, vec![])
        }
        #[test]
        fn one_digit_is_not_bouncy() {
            check(false, vec![1])
        }
        #[test]
        fn increasing_number() {
            check(false, vec![4, 5, 6])
        }
        #[test]
        fn decreasing_number() {
            check(false, vec![5, 4, 3])
        }
        #[test]
        fn bouncy_number() {
            check(true, vec![4, 5, 3])
        }
        #[test]
        fn bouncy_with_same_digit() {
            check(true, vec![1, 5, 5, 3, 4, 9])
        }
    }

    #[test]
    fn example() {
        assert_eq!(538, super::compute(50));
        assert_eq!(21780, super::compute(90));
    }
}
