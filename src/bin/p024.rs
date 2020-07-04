//! [Problem 24](https://projecteuler.net/problem=24) solver.

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

fn compute(mut idx: u64, mut set: Vec<u64>) -> u64 {
    let mut result = vec![];
    while !set.is_empty() {
        let perm = (set.len() as u64 - 1).factorial();
        let (rm_idx, rest) = idx.div_rem(&perm);
        idx = rest;
        result.push(set.remove(rm_idx as usize));
    }
    Integer::from_digits(result.into_iter().rev(), 10)
}

fn solve() -> String {
    compute(1000000 - 1, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]).to_string()
}

common::problem!("2783915460", solve);

#[cfg(test)]
mod tests {
    #[test]
    fn four() {
        assert_eq!(12, super::compute(0, vec![0, 1, 2]));
        assert_eq!(21, super::compute(1, vec![0, 1, 2]));
        assert_eq!(102, super::compute(2, vec![0, 1, 2]));
        assert_eq!(120, super::compute(3, vec![0, 1, 2]));
        assert_eq!(201, super::compute(4, vec![0, 1, 2]));
        assert_eq!(210, super::compute(5, vec![0, 1, 2]));
    }
}
