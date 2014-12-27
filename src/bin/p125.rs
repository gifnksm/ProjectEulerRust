#![warn(bad_style,
        unused, unused_extern_crates, unused_import_braces,
        unused_qualifications, unused_results, unused_typecasts)]

#![feature(phase)]

#[phase(plugin, link)] extern crate common;
extern crate integer;

use std::iter;
use std::num::Int;
use std::collections::HashSet;
use integer::Integer;

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

            if s.is_palindromic(10) { set.insert(s); }
            sq_sums[j] = s;
        }
        sq_sums.push(pow);
    }

    set
}

fn solve() -> String {
    let limit = 10u.pow(8);
    let set = palindromic_sum_set(limit);
    set.iter().fold(0, |x, &y| x + y).to_string()
}

problem!("2906969179", solve);

#[cfg(test)]
mod tests {
    #[test]
    fn palindromic_sum_below_1000() {
        let set = super::palindromic_sum_set(1000);
        assert_eq!(4164, set.iter().fold(0, |x, &y| x + y));
    }
}
