use std;
import std::map::chained::hashmap;

impl map for std::map::chained::t<uint, uint> {
    fn get_at(n: uint) -> uint {
        alt self.find(n) {
          some(x) { ret x; }
          none    {
            let x = if n % 2u == 0u {
                self.get_at(n / 2u) + 1u
            } else {
                self.get_at(3u * n + 1u) + 1u
            };
            self.insert(n, x);
            ret x;
          }
        }
        
    }
}

fn main() {
    let map = std::map::new_uint_hash::<uint>();
    map.insert(1u, 1u);
    let max     = 1u;
    let max_idx = 1u;
    uint::range(2u, 1000000u) { |n|
        let x = map.get_at(n);
        if x > max {
            max     = x;
            max_idx = n;
        }
    };
    std::io::println(#fmt("%u => %u", max_idx, max));
}
