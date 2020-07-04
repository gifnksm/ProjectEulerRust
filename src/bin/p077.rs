//! [Problem 77](https://projecteuler.net/problem=77) solver.

#![warn(
    bad_style,
    unused,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

use prime::PrimeSet;
use std::collections::HashMap;

fn count_way(ps: &PrimeSet, sum: u64, map: &mut HashMap<(u64, usize), u32>) -> u32 {
    let cnt = count_sub(ps, sum, 0, map);

    if ps.contains(sum) {
        return cnt - 1;
    } else {
        return cnt;
    }

    fn count_sub(
        ps: &PrimeSet,
        sum: u64,
        min_idx: usize,
        map: &mut HashMap<(u64, usize), u32>,
    ) -> u32 {
        let mut cnt = 0;
        for i in min_idx.. {
            let p = ps.nth(i);
            if p >= sum {
                if p == sum {
                    let _ = map.insert((p, i), 1);
                    cnt += 1;
                }

                let _ = map.insert((sum, i), cnt);
                break;
            }

            cnt += match map.get(&(sum - p, i)) {
                Some(&n) => n,
                None => count_sub(ps, sum - p, i, map),
            };
        }

        cnt
    }
}

fn solve() -> String {
    let ps = PrimeSet::new();
    let mut map = HashMap::new();
    (1..)
        .find(|&n| count_way(&ps, n, &mut map) > 5000)
        .unwrap()
        .to_string()
}

common::problem!("71", solve);

#[cfg(test)]
mod tests {
    use prime::PrimeSet;
    use std::collections::HashMap;

    #[test]
    fn ten() {
        let ps = PrimeSet::new();
        let mut map = HashMap::new();
        assert_eq!(5, super::count_way(&ps, 10, &mut map));
    }
}
