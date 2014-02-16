#[crate_id = "prob0112"];
#[crate_type = "rlib"];

#[cfg(test)]
extern crate extra;
extern crate math;

use std::iter;
use math::numconv;

pub static EXPECTED_ANSWER: &'static str = "1587000";

fn is_increasing_with<T: Iterator<uint>>(mut ds: T, mut prev: uint) -> bool {
    for n in ds {
        if n < prev { return false }
        prev = n;
    }
    true
}

fn is_decreasing_with<T: Iterator<uint>>(mut ds: T, mut prev: uint) -> bool {
    for n in ds {
        if n > prev { return false }
        prev = n;
    }
    true
}
#[cfg(test)]
fn is_increasing<T: Iterator<uint>>(ds: T) -> bool { is_increasing_with(ds, 0) }
#[cfg(test)]
fn is_decreasing<T: Iterator<uint>>(ds: T) -> bool { is_decreasing_with(ds, std::uint::MAX) }

fn is_bouncy<T: Iterator<uint>>(mut ds: T) -> bool {
    let prev = match ds.next() { Some(x) => x, None => return false };
    loop {
        let n = match ds.next() { Some(x) => x, None => return false };
        if prev < n { return !is_increasing_with(ds, n) }
        if prev > n { return !is_decreasing_with(ds, n) }
    }
}

pub fn solve() -> ~str {
    let mut num_bouncy = 0;
    for n in iter::count(1u, 1) {
        if is_bouncy(numconv::to_digits(n, 10)) { num_bouncy += 1; }
        if n * 99 == 100 * num_bouncy {
            return n.to_str();
        }
    }
    unreachable!()
}

#[cfg(test)]
mod test {

    mod is_increasing {
        use super::super::is_increasing;

        fn check(result: bool, input: ~[uint]) {
            assert_eq!(result, is_increasing(input.move_iter()));
        }

        #[test] fn empty_is_increasing()       { check(true, ~[]) }
        #[test] fn one_digit_is_increasing()   { check(true, ~[1]) }
        #[test] fn increasing_number()         { check(true, ~[4, 5, 6]) }
        #[test] fn decreasing_number()         { check(false, ~[5, 4, 3]) }
        #[test] fn bouncy_number()             { check(false, ~[4, 5, 3]) }
        #[test] fn increasing_with_same_sdigit() { check(true, ~[1, 3, 4, 4, 6, 8]) }
    }

    mod is_decreasing {
        use super::super::is_decreasing;

        fn check(result: bool, input: ~[uint]) {
            assert_eq!(result, is_decreasing(input.move_iter()));
        }
        #[test] fn empty_is_decreasing()     { check(true, ~[]) }
        #[test] fn one_digit_is_decreasing() { check(true, ~[1]) }
        #[test] fn increasing_number()       { check(false, ~[4, 5, 6]) }
        #[test] fn decreasing_number()       { check(true, ~[5, 4, 3]) }
        #[test] fn bouncy_number()           { check(false, ~[4, 5, 3]) }
        #[test] fn decreasing_with_same_digit() { check(true, ~[6, 6, 4, 2, 0]) }
    }

    mod is_bouncy {
        use super::super::is_bouncy;

        fn check(result: bool, input: ~[uint]) {
            assert_eq!(result, is_bouncy(input.move_iter()));
        }
        #[test] fn empty_is_not_bouncy()     { check(false, ~[]) }
        #[test] fn one_digit_is_not_bouncy() { check(false, ~[1]) }
        #[test] fn increasing_number()       { check(false, ~[4, 5, 6]) }
        #[test] fn decreasing_number()       { check(false, ~[5, 4, 3]) }
        #[test] fn bouncy_number()           { check(true, ~[4, 5, 3]) }
        #[test] fn bouncy_with_same_digit()  { check(true, ~[1, 5, 5, 3, 4, 9]) }
    }
}

#[cfg(test)]
mod bench {
    use extra::test::BenchHarness;
    use std::iter;
    use math::numconv;
    use super::is_bouncy;

    #[bench]
    fn bouncy_50percent(bh: &mut BenchHarness) {
        bh.iter(|| {
            let mut num_bouncy = 0;
            for n in iter::count(1u, 1) {
                if is_bouncy(numconv::to_digits(n, 10)) { num_bouncy += 1; }
                if n * 50 == 100 * num_bouncy {
                    assert_eq!(538, n);
                    break;
                }
            }
        })
    }

    #[bench]
    fn bouncy_90percent(bh: &mut BenchHarness) {
        bh.iter(|| {
            let mut num_bouncy = 0;
            for n in iter::count(1u, 1) {
                if is_bouncy(numconv::to_digits(n, 10)) { num_bouncy += 1; }
                if n * 90 == 100 * num_bouncy {
                    assert_eq!(21780, n);
                    break;
                }
            }
        })
    }
}
