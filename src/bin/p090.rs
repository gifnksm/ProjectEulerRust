//! [Problem 90](https://projecteuler.net/problem=90) solver.

#![warn(
    bad_style,
    unused,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

use iter::BitCombination;

fn solve() -> String {
    let all_combs = BitCombination::new(6, 10)
        .map(|mut set| {
            match (set.contains(6), set.contains(9)) {
                (false, true) => {
                    let _ = set.insert(6);
                }
                (true, false) => {
                    let _ = set.insert(9);
                }
                _ => {}
            }
            set
        })
        .collect::<Vec<_>>();

    let nums = (0usize..9)
        .map(|i| {
            let n = (i + 1) * (i + 1);
            (n / 10, n % 10)
        })
        .collect::<Vec<_>>();

    let mut cnt = 0;
    for (i, set1) in all_combs.iter().enumerate() {
        for set2 in &all_combs[i + 1..] {
            let cond = nums.iter().all(|&(a, b)| {
                (set1.contains(a) && set2.contains(b)) || (set1.contains(b) && set2.contains(a))
            });
            if cond {
                cnt += 1;
            }
        }
    }
    cnt.to_string()
}

common::problem!("1217", solve);
