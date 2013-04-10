use core::hashmap::{ HashMap };

use common::prime::{ Prime };
use common::problem::{ Problem };

pub static problem: Problem<'static> = Problem {
    id: 77,
    answer: "71",
    solver: solve
};

fn count_way(
    sum: uint, map: &mut HashMap<(uint, uint), uint>, ps: &mut Prime
) -> uint {

    let cnt = count_sub(sum, 0, map, ps);
    if ps.is_prime(sum) {
        return cnt - 1;
    } else {
        return cnt;
    }

    fn count_sub(
        sum: uint, min_idx: uint, map: &mut HashMap<(uint, uint), uint>,
        ps: &mut Prime
    ) -> uint {
        let mut cnt = 0;
        let mut i = min_idx;
        loop {
            let mut p = ps.get_at(i);
            if p == sum {
                map.insert((p, i), 1);
                cnt += 1;
                break;
            }
            if sum < 2 * p { break; }

            cnt += match map.find(&(sum - p, i)).map(|v| **v) {
                Some(n) => n,
                None    => count_sub(sum - p, i, map, ps)
            };
            i += 1;
        }

        map.insert((sum, i), cnt);
        return cnt;
    }
}

fn solve() -> ~str {
    let mut ps = Prime::new();
    let mut map = HashMap::new();
    let mut n = 1;
    loop {
        let cnt = count_way(n, &mut map, &mut ps);
        if cnt > 5000 { return n.to_str(); }
        n += 1;
    }
}
