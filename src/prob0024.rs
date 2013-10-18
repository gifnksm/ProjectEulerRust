#[link(name = "prob0024", vers = "0.0")];
#[crate_type = "lib"];

extern mod extra;
extern mod common;
extern mod math;

use std::either::{Either, Left, Right};
use std::cmp::{Eq, Ord};
use std::to_bytes::{IterBytes};
use std::hash::{Hash};
use std::hashmap::{HashMap};
use extra::sort;
use common::calc;
use math::numconv;

pub static EXPECTED_ANSWER: &'static str = "2783915460";

fn get_at<K: IterBytes + Hash + Eq + Ord + Clone>(hist: &HashMap<K, uint>, n: uint) -> Either<uint, ~[K]> {
    if hist.is_empty() {
        if n == 1 {
            return Right(~[]);
        } else {
            return Left(0);
        }
    }

    let perm = calc::num_of_permutations(hist);
    if perm < n { return Left(perm); }

    let mut kv = hist.iter().map(|(k, v)| (k.clone(), *v)).to_owned_vec();
    sort::quick_sort(kv, |a, b| a.first() <= b.first());

    let mut idx = 0;
    for (i, &(ref all_k, ref all_v)) in kv.iter().enumerate() {
        let mut new_hist = HashMap::new();
        for &(ref k, ref v) in kv.slice(0, i).iter() {
            new_hist.insert(k.clone(), *v);
        }
        if *all_v > 1 {
            new_hist.insert(all_k.clone(), *all_v - 1);
        }
        for &(ref k, ref v) in kv.slice(i + 1, kv.len()).iter() {
            new_hist.insert(k.clone(), *v);
        }

        match get_at(&new_hist, n - idx) {
            Left(cnt) => idx += cnt,
            Right(ans) => return Right(ans + &[all_k.clone()])
        }
    }

    unreachable!();
}

pub fn solve() -> ~str {
    let nums = calc::histogram::<uint>(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    let ds = get_at(&nums, 1000000).unwrap_right();
    return numconv::from_digits(ds, 10).to_str();
}
