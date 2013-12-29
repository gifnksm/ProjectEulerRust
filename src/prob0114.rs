#[crate_type = "rlib"];

use std::iter;
use std::hashmap::HashMap;

pub static EXPECTED_ANSWER: &'static str = "16475640049";

fn get_cnt(n: uint, map: &mut HashMap<uint, uint>) -> uint {
    let mut sum = 0;
    match map.find(&n) {
        Some(&x) => return x,
        None     => {}
    }

    if n < 3 { map.insert(n, 1); return 1; }

    for len in iter::range_inclusive(3, n) { // most left red block length
        for i in iter::range_inclusive(0, n - len) { // most left red block position
            if n > len + i {
                sum += get_cnt(n - (len + i) - 1, map); // red block and black block
            } else {
                sum += 1;
            }
        }
    }
    sum += 1; // all black block
    map.insert(n, sum);

    sum
}

pub fn solve() -> ~str {
    let mut map = HashMap::new();
    get_cnt(50, &mut map).to_str()
}

#[cfg(test)]
mod test {
    use std::hashmap::HashMap;
    use super::get_cnt;

    #[test]
    fn small_len() {
        let mut map = HashMap::new();
        assert_eq!(1, get_cnt(1, &mut map));
        assert_eq!(1, get_cnt(2, &mut map));
        assert_eq!(2, get_cnt(3, &mut map));
        assert_eq!(4, get_cnt(4, &mut map));
        assert_eq!(17, get_cnt(7, &mut map));
    }
}
