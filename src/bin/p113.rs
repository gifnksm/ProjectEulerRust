//! [Problem 113](https://projecteuler.net/problem=113) solver.

#![warn(bad_style,
        unused, unused_extern_crates, unused_import_braces,
        unused_qualifications, unused_results, unused_typecasts)]

#[macro_use(problem)] extern crate common;

use std::iter::{self, AdditiveIterator};

fn num_increasing(len: uint) -> uint {
    let mut buf = iter::repeat([0u; 10]).take(len).collect::<Vec<_>>();

    for d in range(0, buf[0].len()) {
        buf[0][d] = 1;
    }
    for i in range(1, len) {
        let mut s = 0;
        for d in range(0, buf[i].len()).rev() {
            s += buf[i - 1][d];
            buf[i][d] = s;
        }
    }

    let sum = range(0, buf[len - 1].len())
        .map(|d| buf[len - 1][d])
        .sum();
    sum - 1 // all zero
}

fn num_decreasing(len: uint) -> uint {
    let mut buf = iter::repeat([0u; 11]).take(len).collect::<Vec<_>>(); // 0, 1, 2, .., 9, A

    for d in range(0, buf[0].len()) {
        buf[0][d] = 1;
    }
    for i in range(1, len) {
        let mut s = 0;
        for d in range(0, buf[i].len()) {
            s += buf[i - 1][d];
            buf[i][d] = s;
        }
    }

    let sum = range(0, buf[len - 1].len())
        .map(|d| buf[len - 1][d])
        .sum();

    sum - len // A のみからなるものを取り除く
        - 1   // all zero
}

fn num_nonbouncy(len: uint) -> uint {
    let num_incr = num_increasing(len);
    let num_decr = num_decreasing(len);
    let num_incr_and_decr = 9 * len;
    num_incr + num_decr - num_incr_and_decr
}

fn solve() -> String {
    num_nonbouncy(100).to_string()
}

problem!("51161058134250", solve);


#[cfg(test)]
mod tests {
    #[test]
    fn test_nonbouncy() {
        assert_eq!(12951,  super::num_nonbouncy(6));
        assert_eq!(277032, super::num_nonbouncy(10));
    }
}
