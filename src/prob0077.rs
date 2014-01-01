#[crate_type = "rlib"];

extern mod math;

use std::iter;
use std::hashmap::HashMap;
use math::prime::Prime;

pub static EXPECTED_ANSWER: &'static str = "71";

fn count_way(prime: &Prime, sum: uint, map: &mut HashMap<(uint, uint), uint>) -> uint {
    let cnt = count_sub(prime, sum, 0, map);

    if prime.contains(sum) {
        return cnt - 1;
    } else {
        return cnt;
    }

    fn count_sub(
        prime: &Prime, sum: uint, min_idx: uint, map: &mut HashMap<(uint, uint), uint>
    ) -> uint {
        let mut cnt = 0;
        for i in iter::count(min_idx, 1) {
            let p = prime.nth(i);
            if p >= sum {
                if p == sum  {
                    map.insert((p, i), 1);
                    cnt += 1;
                }
                map.insert((sum, i), cnt);
                break;
            }

            cnt += match map.find(&(sum - p, i)) {
                Some(&n) => n,
                None     => count_sub(prime, sum - p, i, map)
            };
        }

        return cnt;
    }
}

pub fn solve() -> ~str {
    let prime = Prime::new();
    let mut map = HashMap::new();
    iter::count(1u, 1)
        .skip_while(|&n| count_way(&prime, n, &mut map) <= 5000)
        .next()
        .unwrap()
        .to_str()
}
