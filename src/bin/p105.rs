//! [Problem 105](https://projecteuler.net/problem=105) solver.

#![warn(
    bad_style,
    unused,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

use std::{
    cmp::Ordering,
    fs::File,
    io::{self, prelude::*, BufReader},
};

fn is_sss(nums: &mut [u32]) -> bool {
    nums.sort();

    let len = nums.len();
    let len_hd = (len + 1) / 2;
    let len_tl = len_hd - 1;
    let hd = nums[..len_hd].iter().copied().sum::<u32>();
    let tl = nums[len - len_tl..].iter().copied().sum::<u32>();
    if hd <= tl {
        return false;
    }

    let mut sums = vec![0];
    for &n in &*nums {
        let mut i = 0;
        let mut j = 0;
        let len = sums.len();
        let mut new_sums = Vec::with_capacity(len * 2);
        while i < len {
            assert!(j <= i);
            match sums[i].cmp(&(sums[j] + n)) {
                Ordering::Equal => {
                    return false;
                }
                Ordering::Less => {
                    new_sums.push(sums[i]);
                    i += 1;
                }
                Ordering::Greater => {
                    new_sums.push(sums[j] + n);
                    j += 1;
                }
            }
        }
        while j < len {
            new_sums.push(sums[j] + n);
            j += 1;
        }
        sums = new_sums;
    }

    true
}

fn solve(file: File) -> io::Result<String> {
    let mut sum = 0;
    for line in BufReader::new(file).lines() {
        let mut nums = line?
            .trim()
            .split(',')
            .filter_map(|s| s.parse::<u32>().ok())
            .collect::<Vec<_>>();

        if is_sss(&mut nums) {
            sum += nums.iter().sum::<u32>();
        }
    }

    Ok(sum.to_string())
}

common::problem!("73702", "p105_sets.txt", solve);
