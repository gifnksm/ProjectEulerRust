#[link(name = "prob0060", vers = "0.0")];
#[crate_type = "lib"];

extern mod common;

use std::{util, uint};
use std::hashmap::HashMap;
use std::iterator::AdditiveIterator;
use common::prime;
use common::problem::Problem;

pub static problem: Problem<'static> = Problem {
    id: 60,
    answer: "26033",
    solver: solve
};

fn union_vec(v1: &[uint], v2: &[uint]) -> ~[uint] {
    let mut result = ~[];
    let mut i1 = 0;
    let mut i2 = 0;
    let l1 = v1.len();
    let l2 = v2.len();
    while i1 < l1 && i2 < l2 {
        if v1[i1] < v2[i2] { i1 += 1; loop; }
        if v1[i1] > v2[i2] { i2 += 1; loop; }
        result.push(v1[i1]);
        i1 += 1;
        i2 += 1;
    }
    return result;
}

fn find_chain(nums: &[uint], set: ~[uint], map: &HashMap<uint, ~[uint]>) -> ~[~[uint]] {
    if nums.is_empty() { return ~[ set ]; }

    let mut result = ~[];

    for nums.each |&n| {
        let union_nums = union_vec(nums, *map.find(&n).get());
        result += find_chain(union_nums, ~[n] + set, map);
    }

    return result;
}

fn each_pair_set(map: &mut HashMap<uint, ~[uint]>, f: &fn(&[uint]) -> bool) -> bool {
    for prime::each |n| {
        let mut pairs = ~[];

        let n_str = n.to_str();
        for prime::each |m| {
            if m > n { break; }
            let m_str = m.to_str();

            let nm = uint::from_str(n_str + m_str).get();
            if !prime::contains(nm) { loop; }

            let mn = uint::from_str(m_str + n_str).get();
            if !prime::contains(mn) { loop; }

            pairs.push(m);
        }

        for find_chain(pairs, ~[n], map).each |&cs| {
            if !f(cs) { return false; }
        }

        map.insert(n, pairs);
    }
    util::unreachable();
}

pub fn solve() -> ~str {
    let mut map = HashMap::new::<uint, ~[uint]>();

    for each_pair_set(&mut map) |set| {
        if set.len() >= 5 {
            return set.iter().transform(|&x| x).sum().to_str();
        }
    }

    util::unreachable();
}

