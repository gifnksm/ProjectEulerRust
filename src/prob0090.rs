#![crate_name = "prob0090"]
#![crate_type = "rlib"]

extern crate data;

use std::collections::bitv::BitvSet;
use data::extiter::Comb;

pub static EXPECTED_ANSWER: &'static str = "1217";

pub fn solve() -> String {
    let all_combs = Comb::new(6, 10)
        .map(|mut set| {
            match (set.contains(&6), set.contains(&9)) {
                (false, true) => { set.insert(6); },
                (true, false) => { set.insert(9); },
                _ => {}
            }
            set
        }).collect::<Vec<BitvSet>>();

    let nums = Vec::from_fn(9, |i| {
        let n = (i + 1) * (i + 1);
        (n / 10, n % 10)
    });

    let mut cnt = 0u;
    for (i, set1) in all_combs.iter().enumerate() {
        for set2 in  all_combs.slice_from(i + 1).iter() {
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
