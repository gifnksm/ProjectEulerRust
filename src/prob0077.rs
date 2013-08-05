#[link(name = "prob0077", vers = "0.0")];
#[crate_type = "lib"];

extern mod common;

use std::iterator::Counter;
use std::hashmap::HashMap;
use common::prime;

pub static EXPECTED_ANSWER: &'static str = "71";

fn count_way(sum: uint, map: &mut HashMap<(uint, uint), uint>) -> uint {
    let cnt = count_sub(sum, 0, map);

    if prime::contains(sum) {
        return cnt - 1;
    } else {
        return cnt;
    }

    fn count_sub(
        sum: uint, min_idx: uint, map: &mut HashMap<(uint, uint), uint>
    ) -> uint {
        let mut cnt = 0;
        for i in Counter::new(min_idx, 1) {
            let p = prime::nth(i);
            if p >= sum {
                if p == sum  {
                    map.insert((p, i), 1);
                    cnt += 1;
                }
                map.insert((sum, i), cnt);
                break;
            }

            cnt += match map.find(&(sum - p, i)).map(|v| **v) {
                Some(n) => n,
                None    => count_sub(sum - p, i, map)
            };
        }

        return cnt;
    }
}

pub fn solve() -> ~str {
    let mut map = HashMap::new();
    return Counter::new::<uint>(1, 1)
        .skip_while(|&n| count_way(n, &mut map) <= 5000)
        .next()
        .get()
        .to_str();
}
