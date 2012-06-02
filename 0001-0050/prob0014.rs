use std;
import std::map::hashmap;

impl map for hashmap<uint, uint> {
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
    let map = std::map::uint_hash();
    map.insert(1u, 1u);
    let mut max     = 1u;
    let mut max_idx = 1u;
    for uint::range(2u, 1000000u) { |n|
        let x = map.get_at(n);
        if x > max {
            max     = x;
            max_idx = n;
        }
    };
    io::println(#fmt("%u => %u", max_idx, max));
}
