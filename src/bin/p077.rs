//! [Problem 77](https://projecteuler.net/problem=77) solver.

#![warn(bad_style,
        unused, unused_extern_crates, unused_import_braces,
        unused_qualifications, unused_results, unused_typecasts)]

#![feature(phase)]

#[phase(plugin, link)] extern crate common;
extern crate prime;

use std::collections::HashMap;
use std::iter;
use prime::PrimeSet;

fn count_way(ps: &PrimeSet, sum: u64, map: &mut HashMap<(u64, uint), uint>) -> uint {
    let cnt = count_sub(ps, sum, 0, map);

    if ps.contains(sum) {
        return cnt - 1;
    } else {
        return cnt;
    }

    fn count_sub(
        ps: &PrimeSet, sum: u64, min_idx: uint, map: &mut HashMap<(u64, uint), uint>
    ) -> uint {
        let mut cnt = 0;
        for i in iter::count(min_idx, 1) {
            let p = ps.nth(i);
            if p >= sum {
                if p == sum {
                    let _ = map.insert((p, i), 1);
                    cnt += 1;
                }

                let _ = map.insert((sum, i), cnt);
                break
            }

            cnt += match map.get(&(sum - p, i)) {
                Some(&n) => n,
                None     => count_sub(ps, sum - p, i, map)
            };
        }

        cnt
    }
}

fn solve() -> String {
    let ps = PrimeSet::new();
    let mut map = HashMap::new();
    iter::count(1u64, 1)
        .skip_while(|&n| count_way(&ps, n, &mut map) <= 5000)
        .next()
        .unwrap()
        .to_string()
}

problem!("71", solve);

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use prime::PrimeSet;

    #[test]
    fn ten() {
        let ps = PrimeSet::new();
        let mut map = HashMap::new();
        assert_eq!(5, super::count_way(&ps, 10, &mut map));
    }
}
