#![crate_id = "prob0024"]
#![crate_type = "rlib"]

extern crate collections;
extern crate common;
extern crate math;

use std::hash::Hash;
use collections::HashMap;
use common::calc;
use math::numconv;

pub static EXPECTED_ANSWER: &'static str = "2783915460";

enum CountResult<K> {
    Contains(Vec<K>),
    Skip(uint)
}

fn get_at<K: Hash + Eq + Ord + Clone>(hist: &HashMap<K, uint>, n: uint) -> CountResult<K> {
    if hist.is_empty() {
        if n == 1 {
            return Contains(vec![]);
        } else {
            return Skip(0);
        }
    }

    let perm = calc::num_of_permutations(hist);
    if perm < n { return Skip(perm) }

    let mut kv = hist.iter().map(|(k, v)| (k.clone(), *v)).collect::<Vec<(K, uint)>>();
    kv.sort_by(|&(ref a, _), &(ref b, _)| a.cmp(b));

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
            Skip(cnt) => idx += cnt,
            Contains(ans) => return Contains(ans.append([all_k.clone()]))
        }
    }

    unreachable!();
}

pub fn solve() -> String {
    let nums = calc::histogram::<uint>(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    let ds = match get_at(&nums, 1000000) {
        Contains(n) => n,
        _ => fail!()
    };
    numconv::from_digits(ds.as_slice(), 10).to_str()
}
