#[link(name = "prob0060", vers = "0.0", package_id = "prob0060")];
#[crate_type = "lib"];

extern mod math;

use std::hashmap::HashMap;
use std::iter::AdditiveIterator;
use math::prime;

pub static EXPECTED_ANSWER: &'static str = "26033";

fn union_vec(v1: &[uint], v2: &[uint]) -> ~[uint] {
    let mut result = ~[];
    let mut i1 = 0;
    let mut i2 = 0;
    let l1 = v1.len();
    let l2 = v2.len();
    while i1 < l1 && i2 < l2 {
        if v1[i1] < v2[i2] { i1 += 1; continue }
        if v1[i1] > v2[i2] { i2 += 1; continue }
        result.push(v1[i1]);
        i1 += 1;
        i2 += 1;
    }
    result
}

fn find_chain(nums: &[uint], set: ~[uint], map: &HashMap<uint, ~[uint]>) -> ~[~[uint]] {
    if nums.is_empty() { return ~[ set ]; }

    let mut result = ~[];

    for &n in nums.iter() {
        let union_nums = union_vec(nums, *map.find(&n).unwrap());
        result.push_all(find_chain(union_nums, ~[n] + set, map));
    }

    result
}

fn each_pair_set(map: &mut HashMap<uint, ~[uint]>, f: &fn(&[uint]) -> bool) -> bool {
    for n in prime::iter() {
        let mut pairs = ~[];

        let n_str = n.to_str();
        for m in prime::iter() {
            if m > n { break; }
            let m_str = m.to_str();

            let nm = from_str(n_str + m_str).unwrap();
            if !prime::contains(nm) { continue }

            let mn = from_str(m_str + n_str).unwrap();
            if !prime::contains(mn) { continue }

            pairs.push(m);
        }

        let chain = find_chain(pairs, ~[n], map);
        for cs in chain.iter() {
            if !f(*cs) { return false; }
        }

        map.insert(n, pairs);
    }
    unreachable!();
}

pub fn solve() -> ~str {
    let mut map = HashMap::<uint, ~[uint]>::new();

    let mut sum = 0;
    do each_pair_set(&mut map) |set| {
        if set.len() >= 5 {
            sum = set.iter().map(|&x| x).sum();
            false
        } else {
            true
        }
    };
    sum.to_str()
}

