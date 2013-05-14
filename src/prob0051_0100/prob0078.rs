#[link(name = "prob0078", vers = "0.0")];
#[crate_type = "lib"];

extern mod common;

use core::hashmap::{ HashMap };
use core::util::{ unreachable };
use common::problem::{ Problem };

pub static problem: Problem<'static> = Problem {
    id: 78,
    answer: "55374",
    solver: solve
};

static million: int = 1000000;

#[inline(always)]
fn penta(n: int) -> int { n * (3 * n - 1) / 2 }

#[inline(always)]
fn each_penta(f: &fn(int) -> bool) -> bool {
    let mut i = 1;
    loop {
        if !f(penta(i)) { return false; }
        if !f(penta(-i)) { return false; }
        i += 1;
    }
}

#[inline(always)]
fn each_way(f: &fn(int, int) -> bool) -> bool {
    let mut v = HashMap::new();
    v.insert(0, 1);

    let mut n = 1;
    loop {
        let mut way = 0;
        let mut i = 0;
        for each_penta |p| {
            if p > n { break; }

            let sign = if i % 4 > 1 { -1 } else { 1 };
            way += sign * *v.get(&(n - p));
            way %= million;
            i += 1;
        }

        if !f((n + million) % million, way) { return false; }
        v.insert(n, way);
        n += 1;
    }
}

pub fn solve() -> ~str {
    for each_way |n, way| {
        if way % million == 0 {
            return n.to_str();
        }
    }

    unreachable();
}
