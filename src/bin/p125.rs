//! [Problem 125](https://projecteuler.net/problem=125) solver.

#![warn(
    bad_style,
    unused,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

use integer::Integer;
use std::collections::HashSet;

fn palindromic_sum_set(limit: u32) -> HashSet<u32> {
    let mut set = HashSet::new();
    let mut sq_sums: Vec<u32> = vec![];

    let it = (1..).map(|n| n * n).take_while(|&pow| pow < limit);

    for pow in it {
        for j in (0..sq_sums.len()).rev() {
            let s = sq_sums[j] + pow;
            if s >= limit {
                break;
            }

            if s.is_palindromic(10) {
                let _ = set.insert(s);
            }
            sq_sums[j] = s;
        }
        sq_sums.push(pow);
    }

    set
}

fn solve() -> String {
    let limit = 10u32.pow(8);
    let set = palindromic_sum_set(limit);
    set.iter().sum::<u32>().to_string()
}

common::problem!("2906969179", solve);

#[cfg(test)]
mod tests {
    #[test]
    fn palindromic_sum_below_1000() {
        let set = super::palindromic_sum_set(1000);
        assert_eq!(4164, set.iter().sum::<u32>());
    }
}
