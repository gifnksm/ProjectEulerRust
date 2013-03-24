use core::either::{ Either, Left, Right };
use core::cmp::{ Eq, Ord };
use core::to_bytes::{ IterBytes };
use core::hash::{ Hash };
use core::hashmap::linear::{ LinearMap };
use core::util::{ unreachable };

use std::sort::{ quick_sort };

use euler::calc::{ histogram, num_of_permutations, digits_to_num };

fn get_at<K: IterBytes + Hash + Eq + Ord + Copy>(hist: &LinearMap<K, uint>, n: uint) -> Either<uint, ~[K]> {
    if hist.is_empty() {
        return if n == 1 { Right(~[]) } else { Left(0) }
    }

    let perm = num_of_permutations(hist);
    if perm < n { return Left(perm); }


    let mut kv = iter::to_vec(hist);
    quick_sort(kv, |a, b| a.first() <= b.first());

    let mut idx = 0;
    for kv.eachi |i, &(k, v)| {
        let mut new_hist = LinearMap::new();
        for kv.slice(0, i).each |&(&k, &v)| {
            new_hist.insert(k, v);
        }
        if *v > 1 {
            new_hist.insert(*k, v - 1);
        }
        for kv.slice(i + 1, kv.len()).each |&(&k, &v)| {
            new_hist.insert(k, v);
        }

        match get_at(&new_hist, n - idx) {
            Left(cnt) => idx += cnt,
            Right(ans) => return Right(~[*k] + ans)
        }
    }

    unreachable();
}

pub fn solve() -> uint {
    let nums = histogram::<uint>(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    let ds = either::unwrap_right(get_at(&nums, 1000000));
    return digits_to_num(ds, 10);
}
