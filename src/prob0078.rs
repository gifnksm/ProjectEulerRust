#[crate_type = "lib"];

use std::hashmap::HashMap;

pub static EXPECTED_ANSWER: &'static str = "55374";

static MILLION: int = 1000000;

#[inline(always)]
fn penta(n: int) -> int { n * (3 * n - 1) / 2 }

#[inline(always)]
fn each_penta(f: |int| -> bool) -> bool {
    let mut i = 1;
    loop {
        if !f(penta(i)) { return false; }
        if !f(penta(-i)) { return false; }
        i += 1;
    }
}

#[inline(always)]
fn each_way(f: |int, int| -> bool) -> bool {
    let mut v = HashMap::new();
    v.insert(0, 1);

    let mut n = 1;
    loop {
        let mut way = 0;
        let mut i = 0;
        each_penta(|p| {
            if p > n { false } else {
                let sign = if i % 4 > 1 { -1 } else { 1 };
                way += sign * *v.get(&(n - p));
                way %= MILLION;
                i += 1;
                true
            }
        });

        if !f((n + MILLION) % MILLION, way) { return false; }
        v.insert(n, way);
        n += 1;
    }
}

pub fn solve() -> ~str {
    let mut ans = 0;
    each_way(|n, way| {
        if way % MILLION == 0 {
            ans = n;
            false
        } else {
            true
        }
    });
    ans.to_str()
}
