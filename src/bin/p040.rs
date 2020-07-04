//! [Problem 40](https://projecteuler.net/problem=40) solver.

#![warn(
    bad_style,
    unused,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

use integer::Integer;
use num_integer::Integer as NumInteger;

struct Group {
    num_len: usize,
    radix: usize,
    idx: (usize, usize),
    num: (usize, usize),
}

// Group 0: num: [1, 10),     idx: [0, 1 * 9)
// Group 1: num: [10, 100),   idx: [9 + 0, 9 + 1 * 90)
// Group 2: num: [100, 1000), idx: [189 + 0, 189 + 3 * 900)
impl Group {
    fn new(radix: usize) -> Group {
        Group::new_with_init(1, radix, 0, 1)
    }

    fn new_with_init(num_len: usize, radix: usize, min_idx: usize, min_num: usize) -> Group {
        let num_elem = min_num * (radix - 1);
        Group {
            num_len,
            radix,
            idx: (min_idx, min_idx + num_len * num_elem),
            num: (min_num, min_num + num_elem),
        }
    }

    fn next(&self) -> Group {
        Group::new_with_init(self.num_len + 1, self.radix, self.idx.1, self.num.1)
    }

    fn get_nth_digit(&self, idx: usize) -> Option<usize> {
        if idx < self.idx.0 || self.idx.1 <= idx {
            return None;
        }
        let (d, r) = (idx - self.idx.0).div_rem(&self.num_len);
        (self.num.0 + d).into_digits(self.radix).rev().nth(r)
    }
}

fn nth_digit(n: usize, radix: usize) -> usize {
    let mut g = Group::new(radix);
    loop {
        if let Some(d) = g.get_nth_digit(n) {
            return d;
        }
        g = g.next()
    }
}

fn compute(idxs: &[usize], radix: usize) -> usize {
    idxs.iter().map(|&i| nth_digit(i, radix)).product()
}

fn solve() -> String {
    let idxs = &[0, 9, 99, 999, 9999, 99999, 999999];
    compute(idxs, 10).to_string()
}

common::problem!("210", solve);

#[cfg(test)]
mod tests {
    use super::Group;

    #[test]
    fn group() {
        let g0 = Group::new(10);
        assert_eq!((0, 9), g0.idx);
        assert_eq!((1, 10), g0.num);
        let g1 = g0.next();
        assert_eq!((9, 189), g1.idx);
        assert_eq!((10, 100), g1.num);
        let g2 = g1.next();
        assert_eq!((189, 2889), g2.idx);
        assert_eq!((100, 1000), g2.num);
    }

    #[test]
    fn d12() {
        let radix = 10;
        assert_eq!(1, super::nth_digit(11, radix));
    }
}
