#[link(name = "prob0024", vers = "0.0")];
#[crate_type = "lib"];

extern mod extra;
extern mod common;

use std::{either, util};
use std::either::{Either, Left, Right};
use std::cmp::{Eq, Ord};
use std::to_bytes::{IterBytes};
use std::hash::{Hash};
use std::hashmap::{HashMap};
use extra::sort;
use common::calc;

pub static EXPECTED_ANSWER: &'static str = "2783915460";

fn get_at<K: IterBytes + Hash + Eq + Ord + Clone>(hist: &HashMap<K, uint>, n: uint) -> Either<uint, ~[K]> {
    if hist.is_empty() {
        return if n == 1 { Right(~[]) } else { Left(0) }
    }

    let perm = calc::num_of_permutations(hist);
    if perm < n { return Left(perm); }


    let mut kv = hist.iter().transform(|(k, v)| (k.clone(), *v)).collect::<~[(K, uint)]>();
    sort::quick_sort(kv, |a, b| a.first() <= b.first());

    let mut idx = 0;
    for kv.iter().enumerate().advance |(i, &(ref all_k, ref all_v))| {
        let mut new_hist = HashMap::new();
        for kv.slice(0, i).iter().advance |&(ref k, ref v)| {
            new_hist.insert(k.clone(), *v);
        }
        if *all_v > 1 {
            new_hist.insert(all_k.clone(), *all_v - 1);
        }
        for kv.slice(i + 1, kv.len()).iter().advance |&(ref k, ref v)| {
            new_hist.insert(k.clone(), *v);
        }

        match get_at(&new_hist, n - idx) {
            Left(cnt) => idx += cnt,
            Right(ans) => return Right(~[all_k.clone()] + ans)
        }
    }

    util::unreachable();
}

pub fn solve() -> ~str {
    let nums = calc::histogram::<uint>(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    let ds = either::unwrap_right(get_at(&nums, 1000000));
    return calc::digits_to_num(ds, 10).to_str();
}
