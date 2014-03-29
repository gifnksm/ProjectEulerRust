#![crate_id = "prob0076"]
#![crate_id = "prob0076"]
#![crate_type = "rlib"]
#![crate_type = "rlib"]

extern crate collections;

use collections::HashMap;

pub static EXPECTED_ANSWER: &'static str = "190569291";

fn count_way(sum: uint) -> uint {
    let mut map = HashMap::new();
    return count_sub(sum, 1, &mut map) - 1;

    fn count_sub(
        sum: uint, min_n: uint, map: &mut HashMap<(uint, uint), uint>
    ) -> uint {
        let mut cnt = 1; // only sum
        for k in range(min_n, sum / 2 + 1) {
            match map.find(&(sum - k, k)) {
                Some(&n) => cnt += n,
                None     => {
                    let n = count_sub(sum - k, k, map);
                    map.insert((sum - k, k), n);
                    cnt += n;
                }
            }
        }
        return cnt;
    }
}

pub fn solve() -> ~str {
    return count_way(100).to_str();
}
