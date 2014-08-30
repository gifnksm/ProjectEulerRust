#![crate_name = "prob0125"]
#![crate_type = "rlib"]

extern crate math;

use std::{iter, num};
use std::collections::HashSet;
use math::numconv;

pub static EXPECTED_ANSWER: &'static str = "2906969179";

fn palindromic_sum_set(limit: uint) -> HashSet<uint> {
    let mut set = HashSet::new();
    let mut sq_sums = Vec::<uint>::new();

    let mut it = iter::count(1u, 1)
        .map(|n| n * n)
        .take_while(|&pow| pow < limit);

    for pow in it {
        for j in range(0, sq_sums.len()).rev() {
            let s = sq_sums[j] + pow;
            if s >= limit { break; }

            if numconv::is_palindromic(s, 10) { set.insert(s); }
            *sq_sums.get_mut(j) = s;
        }
        sq_sums.push(pow);
    }

    set
}

pub fn solve() -> String {
    let limit = num::pow(10u, 8);
    let set = palindromic_sum_set(limit);
    set.iter().fold(0, |x, &y| x + y).to_string()
}

#[cfg(test)]
mod tests {
    #[test]
    fn palindromic_sum_below_1000() {
        let set = super::palindromic_sum_set(1000);
        assert_eq!(4164, set.iter().fold(0, |x, &y| x + y));
    }
}
