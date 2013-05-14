#[link(name = "prob0024", vers = "0.0")];
#[crate_type = "lib"];

extern mod std;
extern mod common;

use core::either::{ Either, Left, Right };
use core::cmp::{ Eq, Ord };
use core::to_bytes::{ IterBytes };
use core::hash::{ Hash };
use core::hashmap::{ HashMap };
use core::util;
use std::sort;
use common::calc;
use common::problem::{ Problem };

pub static problem: Problem<'static> = Problem {
    id: 24,
    answer: "2783915460",
    solver: solve
};

fn get_at<K: IterBytes + Hash + Eq + Ord + Copy>(hist: &HashMap<K, uint>, n: uint) -> Either<uint, ~[K]> {
    if hist.is_empty() {
        return if n == 1 { Right(~[]) } else { Left(0) }
    }

    let perm = calc::num_of_permutations(hist);
    if perm < n { return Left(perm); }


    let mut kv = ~[];
    for hist.each |&k, &v| { kv.push((k, v)); }
    sort::quick_sort(kv, |a, b| a.first() <= b.first());

    let mut idx = 0;
    for kv.eachi |i, &(k, v)| {
        let mut new_hist = HashMap::new();
        for kv.slice(0, i).each |&(k, v)| {
            new_hist.insert(k, v);
        }
        if v > 1 {
            new_hist.insert(k, v - 1);
        }
        for kv.slice(i + 1, kv.len()).each |&(k, v)| {
            new_hist.insert(k, v);
        }

        match get_at(&new_hist, n - idx) {
            Left(cnt) => idx += cnt,
            Right(ans) => return Right(~[k] + ans)
        }
    }

    util::unreachable();
}

pub fn solve() -> ~str {
    let nums = calc::histogram::<uint>(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    let ds = either::unwrap_right(get_at(&nums, 1000000));
    return calc::digits_to_num(ds, 10).to_str();
}
