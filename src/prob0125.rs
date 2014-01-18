#[crate_type = "rlib"];

extern mod math;

use std::{iter, num};
use std::hashmap::HashSet;
use math::numconv;

pub static EXPECTED_ANSWER: &'static str = "2906969179";

fn palindromic_sum_set(limit: uint) -> HashSet<uint> {
    let mut set = HashSet::new();
    let mut sq_sums: ~[uint] = ~[];

    let mut it = iter::count(1u, 1)
        .map(|n| n * n)
        .take_while(|&pow| pow < limit);

    for pow in it {
        for j in range(0, sq_sums.len()).invert() {
            let s = sq_sums[j] + pow;
            if s >= limit { break; }

            if numconv::is_palindromic(s, 10) { set.insert(s); }
            sq_sums[j] = s;
        }
        sq_sums.push(pow);
    }

    set
}

pub fn solve() -> ~str {
    let limit = num::pow(10u, 8);
    let set = palindromic_sum_set(limit);
    set.iter().fold(0, |x, &y| x + y).to_str()
}

#[cfg(test)]
mod test {
    #[test]
    fn palindromic_sum_below_1000() {
        let set = super::palindromic_sum_set(1000);
        assert_eq!(4164, set.iter().fold(0, |x, &y| x + y));
    }
}
