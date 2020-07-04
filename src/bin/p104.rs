//! [Problem 104](https://projecteuler.net/problem=104) solver.

#![warn(
    bad_style,
    unused,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

use num_integer::Integer;

fn is_pandigit(n: u64) -> bool {
    let mut hist = [false; 10];
    let mut cnt = 0;
    let mut itr = n;
    while itr > 0 {
        let (d, r) = itr.div_rem(&10);
        if r == 0 || hist[r as usize] {
            return false;
        }
        hist[r as usize] = true;
        itr = d;
        cnt += 1;
    }
    cnt == 9
}

struct FibFirst {
    base: u64,
    phi: f64,
    curr: u64,
    cnt: usize,
}

impl FibFirst {
    fn new(len: u32) -> FibFirst {
        assert!(len > 0);
        FibFirst {
            base: 10u64.pow(len),
            phi: (1.0 + (5.0f64).sqrt()) / 2.0,
            curr: 1,
            cnt: 1,
        }
    }
}

impl Iterator for FibFirst {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        let next = match self.cnt {
            0 => 1,
            1 => 1,
            2 => 2,
            3 => 3,
            4 => 5,
            _ => {
                let mut f = ((self.curr as f64) * self.phi + 0.5) as u64;
                while f > self.base * self.base {
                    f /= 10;
                }
                f
            }
        };

        let mut curr = self.curr;
        self.curr = next;
        self.cnt += 1;
        while curr > self.base {
            curr /= 10;
        }
        Some(curr)
    }
}

struct FibLast {
    base: u64,
    curr: u64,
    next: u64,
}

impl FibLast {
    fn new(len: u32) -> FibLast {
        assert!(len > 0);
        FibLast {
            base: 10u64.pow(len),
            curr: 1,
            next: 1,
        }
    }
}

impl Iterator for FibLast {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        let next = (self.curr + self.next) % self.base;
        let curr = self.curr;
        self.curr = self.next;
        self.next = next;
        Some(curr)
    }
}

fn solve() -> String {
    let len = 9;
    let first = FibFirst::new(len);
    let last = FibLast::new(len);

    let (k, _) = first
        .zip(last)
        .enumerate()
        .find(|&(_, (f, l))| is_pandigit(f) && is_pandigit(l))
        .unwrap();
    (k + 1).to_string()
}

common::problem!("329468", solve);

#[cfg(test)]
mod tests {
    use super::{FibFirst, FibLast};
    use num_bigint::BigUint;
    use seq::Fibonacci;

    #[test]
    fn fib() {
        let len = 9;
        let it = Fibonacci::<BigUint>::new()
            .zip(FibFirst::new(len as u32).zip(FibLast::new(len as u32)));
        for (bu, (fst, lst)) in it.take(100) {
            let bus = bu.to_string();
            if bus.len() < len {
                assert_eq!(bus, fst.to_string());
                assert_eq!(bus, lst.to_string());
            } else {
                assert_eq!(bus[..len].parse::<u64>().unwrap(), fst);
                assert_eq!(bus[bus.len() - len..].parse::<u64>().unwrap(), lst);
            }
        }
    }
}
