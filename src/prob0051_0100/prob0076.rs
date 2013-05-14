use core::hashmap::{ HashMap };

use common::problem::{ Problem };

pub static problem: Problem<'static> = Problem {
    id: 76,
    answer: "190569291",
    solver: solve
};

fn count_way(sum: uint) -> uint {
    let mut map = HashMap::new();
    return count_sub(sum, 1, &mut map) - 1;

    fn count_sub(
        sum: uint, min_n: uint, map: &mut HashMap<(uint, uint), uint>
    ) -> uint {
        let mut cnt = 1; // only sum
        for uint::range(min_n, sum / 2 + 1) |k| {
            match map.find(&(sum - k, k)).map(|v| **v) {
                Some(n) => cnt += n,
                None    => {
                    let n = count_sub(sum - k, k, map);
                    map.insert((sum - k, k), n);
                    cnt += n;
                }
            }
        }
        return cnt;
    }
}

fn solve() -> ~str {
    return count_way(100).to_str();
}
