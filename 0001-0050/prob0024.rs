extern mod std;
extern mod euler;

use either::{ Either, Left, Right };
use cmp::{ Eq, Ord };
use core::to_bytes::{ IterBytes };
use core::hash::{ Hash };
use core::util;

use std::map::{ HashMap, hash_from_vec };
use std::sort::{ quick_sort };

use euler::calc::{ histogram, num_of_permutations };

fn get_at<T: Eq Ord IterBytes Hash Const Copy>(hist: HashMap<T, uint>, n: uint) -> Either<uint, ~[T]> {
    if hist.size() == 0 {
        return if n == 1 { Right(~[]) } else { Left(0) }
    }

    let perm = num_of_permutations(hist);
    if perm < n { return Left(perm); }

    let mut kv = ~[];
    for hist.each |k, v| { kv += [(k, v)]; }
    quick_sort(kv, |a, b| a.first() <= b.first());

    let mut idx = 0;
    for kv.eachi |i, tp| {
        let &(k, v) = tp;
        let new_hist = if v > 1 {
            hash_from_vec(vec::slice(kv, 0, i) + [(k, v - 1)] + vec::view(kv, i + 1, kv.len()))
        } else {
            hash_from_vec(vec::slice(kv, 0, i) + vec::view(kv, i + 1, kv.len()))
        };

        match get_at(new_hist, n - idx) {
            Left(cnt) => idx += cnt,
            Right(ans) => return Right(~[k] + ans)
        }
    }

    util::unreachable();
}

fn main() {
    let nums = histogram(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    io::println(fmt!("%?", either::unwrap_right(get_at(nums, 1000000))));
}