//! [Problem 88](https://projecteuler.net/problem=88) solver.

#![warn(
    bad_style,
    unused,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

use num_integer::Integer;
use std::u32;

fn each_sum_product<F>(prod_start: u32, prod_end: u32, f: &mut F)
where
    F: FnMut(u32, u32, usize),
{
    sub(2, prod_start, prod_end, 0, 1, 0, f);

    fn sub<F>(
        min_n: u32,
        prod_start: u32,
        prod_end: u32,
        sum_base: u32,
        prod_base: u32,
        len_base: usize,
        f: &mut F,
    ) where
        F: FnMut(u32, u32, usize),
    {
        for n in min_n..prod_end.div_ceil(&prod_base) {
            let prod = prod_base * n;
            let sum = sum_base + n;
            let len = len_base + 1;
            if len > 1 && prod >= prod_start {
                (*f)(sum, prod, len)
            }
            sub(n, prod_start, prod_end, sum, prod, len, f)
        }
    }
}

fn each_product_sum_number<F>(start: u32, end: u32, f: &mut F)
where
    F: FnMut(u32, usize),
{
    each_sum_product(start, end, &mut |sum, prod, len| {
        let len = (prod - sum) as usize + len;
        (*f)(prod, len)
    })
}

fn compute(limit: usize) -> u32 {
    let mut start = 2;
    let mut cnt = limit - 1;
    let mut nums = vec![u32::MAX; limit + 1];

    while cnt > 0 {
        let end = start * 2;
        each_product_sum_number(start, end, &mut |n, len| {
            if len <= limit && n < nums[len] {
                if nums[len] == u32::MAX {
                    cnt -= 1;
                }
                nums[len] = n;
            }
        });
        start *= 2;
    }

    nums.sort();
    nums.dedup();
    if *nums.last().unwrap() == u32::MAX {
        let _ = nums.pop();
    }
    nums.into_iter().sum()
}

fn solve() -> String {
    compute(12000).to_string()
}

common::problem!("7587457", solve);

#[cfg(test)]
mod tests {
    #[test]
    fn each_sum_product() {
        let mut triples = vec![];
        super::each_sum_product(2, 5, &mut |sum, prod, len| triples.push((sum, prod, len)));
        assert_eq!(triples, vec![(4, 4, 2)]);

        let mut triples = vec![];
        super::each_sum_product(2, 10, &mut |sum, prod, len| triples.push((sum, prod, len)));
        assert_eq!(
            triples,
            vec![(4, 4, 2), (6, 8, 3), (5, 6, 2), (6, 8, 2), (6, 9, 2)]
        );

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
