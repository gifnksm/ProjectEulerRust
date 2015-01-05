//! [Problem 88](https://projecteuler.net/problem=88) solver.

#![warn(bad_style,
        unused, unused_extern_crates, unused_import_braces,
        unused_qualifications, unused_results, unused_typecasts)]

#![feature(phase, slicing_syntax)]

#[phase(plugin, link)] extern crate common;
extern crate integer;

use std::uint;
use std::iter::{self, AdditiveIterator};
use integer::Integer;

fn each_sum_product(prod_start: uint, prod_end: uint, f: &mut |uint, uint, uint|) {
    sub(2, prod_start, prod_end, 0, 1, 0, f);

    fn sub(min_n: uint, prod_start: uint, prod_end: uint,
           sum_base: uint, prod_base: uint, len_base: uint,
           f: &mut |uint, uint, uint|) {

        for n in range(min_n, prod_end.div_ceil(&prod_base)) {
            let prod = prod_base * n;
            let sum  = sum_base  + n;
            let len  = len_base  + 1;
            if len > 1 && prod >= prod_start {
                (*f)(sum, prod, len)
            }
            sub(n, prod_start, prod_end, sum, prod, len, f)
        }

    }
}

fn each_product_sum_number(start: uint, end: uint, f: &mut |uint, uint|) {
    each_sum_product(start, end, &mut |sum, prod, len| {
        let len = prod - sum + len;
        (*f)(prod, len)
    })
}

fn compute(limit: uint) -> uint {
    let mut start = 2;
    let mut cnt   = limit - 1;
    let mut nums  = iter::repeat(uint::MAX).take(limit + 1).collect::<Vec<_>>();

    while cnt > 0 {
        let end = start * 2;
        each_product_sum_number(start, end, &mut |n, len| {
            if len <= limit && n < nums[len] {
                if nums[len] == uint::MAX { cnt -= 1; }
                nums[len] = n;
            }
        });
        start *= 2;
    }

    nums.sort();
    nums.dedup();
    if *nums.last().unwrap() == uint::MAX {
        let _ = nums.pop();
    }
    nums.into_iter().sum()
}

fn solve() -> String {
    compute(12000).to_string()
}

problem!("7587457", solve);

#[cfg(test)]
mod tests {
    #[test]
    fn each_sum_product() {
        let mut triples = vec![];
        super::each_sum_product(2, 5, &mut |sum, prod, len| triples.push((sum, prod, len)));
        assert_eq!(triples, vec![(4, 4, 2)]);

        let mut triples = vec![];
        super::each_sum_product(2, 10, &mut |sum, prod, len| triples.push((sum, prod, len)));
        assert_eq!(triples, vec![(4, 4, 2), (6, 8, 3), (5, 6, 2), (6, 8, 2), (6, 9, 2)]);

        let mut triples = vec![];
        super::each_sum_product(5, 10, &mut |sum, prod, len| triples.push((sum, prod, len)));
        assert_eq!(triples, vec![(6, 8, 3), (5, 6, 2), (6, 8, 2), (6, 9, 2)]);
    }

    #[test]
    fn each_product_sum_number() {
        let mut pairs = vec![];
        super::each_product_sum_number(2, 5, &mut |n, len| pairs.push((n, len)));
        assert_eq!(pairs, vec![(4, 2)]);

        let mut pairs = vec![];
        super::each_product_sum_number(2, 10, &mut |n, len| pairs.push((n, len)));
        assert_eq!(pairs, vec![(4, 2), (8, 5), (6, 3), (8, 4), (9, 5)]);

        let mut pairs = vec![];
        super::each_product_sum_number(5, 10, &mut |n, len| pairs.push((n, len)));
        assert_eq!(pairs, vec![(8, 5), (6, 3), (8, 4), (9, 5)]);
    }

    #[test]
    fn compute() {
        assert_eq!(30, super::compute(6));
        assert_eq!(61, super::compute(12));
    }
}
