use core::hashmap::linear::{ LinearMap };

fn get_len(map: &mut LinearMap<uint, uint>, n: uint) -> uint {
    let v = map.find(&n).map(|& &n| n);
    match v {
      Some(x) => return x,
      None    => {
        let x = if n % 2 == 0 {
            get_len(map, n / 2) + 1
        } else {
            get_len(map, 3 * n + 1) + 1
        };
        map.insert(n, x);
        return x;
      }
    }
}

pub fn solve() -> uint {
    let mut map = LinearMap::new();
    map.insert(1u, 1u);
    let mut max     = 1u;
    let mut max_idx = 1u;
    for uint::range(2u, 1000000u) |n| {
        let x = get_len(&mut map, n);
        if x > max {
            max     = x;
            max_idx = n;
        }
    }

    return max_idx;
}
