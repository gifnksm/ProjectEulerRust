#[crate_id = "prob0087"];
#[crate_type = "rlib"];

extern crate collections;
extern crate math;

use collections::HashSet;
use math::prime::Prime;

pub static EXPECTED_ANSWER: &'static str = "1097343";

pub fn solve() -> ~str {
    let limit = 50000000;

    let prime = Prime::new();
    let mut cnt = 0u;
    let mut set = HashSet::with_capacity(2000000);

    for p in prime.iter() {
        let p4 = p * p * p * p;
        if p4 >= limit { break }
        for q in prime.iter() {
            let q3 = q * q * q;
            if p4 + q3 >= limit { break }
            for r in prime.iter() {
                let r2 = r * r;
                let s = p4 + q3 + r2;
                if s >= limit { break }
                if set.contains(&s) { continue }
                set.insert(s);
                cnt += 1;
            }
        }
    }

    cnt.to_str()
}
