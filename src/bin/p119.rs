//! [Problem 119](https://projecteuler.net/problem=119) solver.

#![warn(bad_style,
        unused, unused_extern_crates, unused_import_braces,
        unused_qualifications, unused_results)]

#![feature(iter_arith, wrapping)]

#[macro_use(problem)] extern crate common;
extern crate integer;

use std::cmp::Ordering;
use std::num::wrapping::OverflowingOps;
use std::collections::BinaryHeap;
use integer::Integer;

trait IntExt {
    fn checked_pow(self, exp: u32) -> Option<Self>;
}

impl IntExt for u64 {
     fn checked_pow(self, mut exp: u32) -> Option<Self> {
        let mut base = self;
        let mut acc = 1u64;

        let mut prev_base = self;
        let mut base_oflo = false;
        while exp > 0 {
            if (exp & 1) == 1 {
                let new_acc;
                if base_oflo {
                    // ensure overflow occurs in the same manner it
                    // would have otherwise (i.e. signal any exception
                    // it would have otherwise).
                    new_acc = acc.checked_mul(prev_base * prev_base);
                } else {
                    new_acc = acc.checked_mul(base);
                };
                match new_acc {
                    Some(a) => acc = a,
                    None => return None
                }
            }
            prev_base = base;
            let (new_base, new_base_oflo) = base.overflowing_mul(base);
            base = new_base;
            base_oflo = new_base_oflo;
            exp /= 2;
        }
        Some(acc)
    }
}

struct Power(u64, u64, u32);

impl PartialEq for Power {
    fn eq(&self, other: &Power) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}

impl Eq for Power {}

impl PartialOrd for Power {
    fn partial_cmp(&self, other: &Power) -> Option<Ordering> { Some(self.cmp(other)) }
}

impl Ord for Power {
    fn cmp(&self, other: &Power) -> Ordering {
        (self.0, self.1).cmp(&(other.0, other.1)).reverse()
    }
}

struct Powers {
    heap: BinaryHeap<Power>
}

impl Powers {
    fn new() -> Powers {
        let mut heap = BinaryHeap::new();
        heap.push(Power(4, 2, 2));
        Powers { heap: heap }
    }
}

impl Iterator for Powers {
    type Item = (u64, u64, u32);

    fn next(&mut self) -> Option<(u64, u64, u32)> {
        let Power(n, b, e) = self.heap.pop().unwrap();
        if b == 2 { self.heap.push(Power(n * b, b, e + 1)); }
        if b < 99 { // assume base is smaller than 100
            if let Some(new_n) = (b + 1).checked_pow(e) {
                self.heap.push(Power(new_n, b + 1, e));
            }
        }
        Some((n, b, e))
    }
}

fn compute_a(n: usize) -> (u64, u64, u32) {
    Powers::new()
        .skip_while(|&(n, _b, _e)| n < 10)
        .filter(|&(n, b, _e)| n.into_digits(10).sum::<u64>() == b)
        .nth(n - 1)
        .unwrap()
}

fn solve() -> String {
    let (n, _b, _e) = compute_a(30);
    n.to_string()
}

problem!("248155780267521", solve);

#[cfg(test)]
mod tests {
    use super::Powers;

    #[test]
    fn powers() {
        let mut it = Powers::new();
        assert_eq!(Some((4, 2, 2)), it.next());
        assert_eq!(Some((8, 2, 3)), it.next());
        assert_eq!(Some((9, 3, 2)), it.next());
        assert_eq!(Some((16, 2, 4)), it.next());
        assert_eq!(Some((16, 4, 2)), it.next());
        assert_eq!(Some((25, 5, 2)), it.next());
        assert_eq!(Some((27, 3, 3)), it.next());
        assert_eq!(Some((32, 2, 5)), it.next());
        assert_eq!(Some((36, 6, 2)), it.next());
        assert_eq!(Some((49, 7, 2)), it.next());
        assert_eq!(Some((64, 2, 6)), it.next());
        assert_eq!(Some((64, 4, 3)), it.next());
        assert_eq!(Some((64, 8, 2)), it.next());
        assert_eq!(Some((81, 3, 4)), it.next());
        assert_eq!(Some((81, 9, 2)), it.next());
        assert_eq!(Some((100, 10, 2)), it.next());
    }

    #[test]
    fn compute_a() {
        assert_eq!((512, 8, 3), super::compute_a(2));
        assert_eq!((614656, 28, 4), super::compute_a(10));
    }
}
