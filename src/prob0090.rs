#[link(name = "prob0090", vers = "0.0", package_id = "prob0090")];
#[crate_type = "lib"];

extern mod common;

use std::vec;
use common::calc::CombinateIterator;

pub static EXPECTED_ANSWER: &'static str = "1217";

pub fn solve() -> ~str {
    let nums = vec::from_fn(10, |i| i);
    let all_combs = CombinateIterator::new(nums, 6)
        .map(|cs| cs.map(|&x| *x))
        .map(|mut cs| {
            match (cs.iter().any(|&x| x == 6), cs.iter().any(|&x| x == 9)) {
                (false, true)  => cs.push(6),
                (true,  false) => cs.push(9),
                _ => {}
            }
            cs
        }).to_owned_vec();

    let nums = vec::from_fn(9, |i| {
        let n = (i + 1) * (i + 1);
        (n / 10, n % 10)
    });

    let mut cnt = 0u;
    for (i, cs1) in all_combs.iter().enumerate() {
        for cs2 in  all_combs.tailn(i + 1).iter() {
            let cond = |&(a, b): &(uint, uint)| {
                (cs1.iter().any(|&x| x == a) && cs2.iter().any(|&x| x == b)) ||
                    (cs1.iter().any(|&x| x == b) && cs2.iter().any(|&x| x == a))
            };
            if nums.iter().all(cond) { cnt += 1; }
        }
    }
    return cnt.to_str();
}
