use core::util::{ unreachable };
use core::hashmap::linear::{ LinearMap };

use common::prime::{ Prime };
use common::problem::{ Problem };

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

fn find_chain(nums: &[uint], set: ~[uint], map: &LinearMap<uint, ~[uint]>) -> ~[~[uint]] {
    if nums.is_empty() { return ~[ set ]; }

    let mut result = ~[];

    for nums.each |&n| {
        let union_nums = union_vec(nums, *map.find(&n).get());
        result += find_chain(union_nums, ~[n] + set, map);
    }

    return result;
}

fn each_pair_set(
    ps: &mut Prime, map: &mut LinearMap<uint, ~[uint]>,
    f: &fn(&[uint]) -> bool
) {
    for ps.each_borrow |n, ps| {
        let mut pairs = ~[];

        let n_str = n.to_str();
        for ps.each_borrow |m, ps| {
            if m > n { break; }
            let m_str = m.to_str();

            let nm = uint::from_str(n_str + m_str).get();
            if !ps.is_prime(nm) { loop; }

            let mn = uint::from_str(m_str + n_str).get();
            if !ps.is_prime(mn) { loop; }

            pairs.push(m);
        }

        for find_chain(pairs, ~[n], map).each |&cs| {
            if !f(cs) { return; }
        }

        map.insert(n, pairs);
    }
}

fn solve() -> ~str {
    let mut ps  = Prime::new();
    let mut map = LinearMap::new::<uint, ~[uint]>();

    for each_pair_set(&mut ps, &mut map) |set| {
        if set.len() >= 5 {
            return set.foldl(0u, |s, &n| s + n).to_str();
        }
    }

    unreachable();
}

