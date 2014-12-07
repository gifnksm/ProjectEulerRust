#![warn(bad_style,
        unused, unused_extern_crates, unused_import_braces,
        unused_qualifications, unused_results, unused_typecasts)]

#![feature(slicing_syntax)]

extern crate common;
extern crate iter;

use common::Solver;
use iter::BitCombination;

fn solve() -> String {
    let all_combs = BitCombination::new(6, 10)
        .map(|mut set| {
            match (set.contains(&6), set.contains(&9)) {
                (false, true) => { set.insert(6); },
                (true, false) => { set.insert(9); },
                _ => {}
            }
            set
        }).collect::<Vec<_>>();

    let nums = Vec::from_fn(9, |i| {
        let n = (i + 1) * (i + 1);
        (n / 10, n % 10)
    });

    let mut cnt = 0u;
    for (i, set1) in all_combs.iter().enumerate() {
        for set2 in  all_combs[i + 1 ..].iter() {
            let cond = nums.iter()
                .all(|&(a, b)| {
                    (set1.contains(&a) && set2.contains(&b)) ||
                        (set1.contains(&b) && set2.contains(&a))
                });
            if cond { cnt += 1; }
        }
    }
    cnt.to_string()
}

fn main() { Solver::new("1217", solve).run(); }
