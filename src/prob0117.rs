#![crate_id = "prob0117"]
#![crate_type = "rlib"]

extern crate collections;

use std::iter;
use collections::HashMap;

pub static EXPECTED_ANSWER: &'static str = "100808458960497";

fn count(len: uint, map: &mut HashMap<uint, uint>) -> uint {
    match map.find(&len) {
        Some(&x) => return x,
        None => {}
    }

    let mut sum = 0;
    for i in iter::range_inclusive(0, len) { // most left block position
        if len - i >= 2 { sum += count(len - i - 2, map); } // red
        if len - i >= 3 { sum += count(len - i - 3, map); } // green
        if len - i >= 4 { sum += count(len - i - 4, map); } // blue
    }
    sum += 1; // all black
    map.insert(len, sum);
    sum
}

pub fn solve() -> ~str {
    let mut map = HashMap::new();
    count(50, &mut map).to_str()
}

#[cfg(test)]
mod tests {
    use super::count;
    use collections::HashMap;

    #[test]
    fn couunt_test() {
        let mut map = HashMap::new();
        assert_eq!(15, count(5, &mut map));
    }
}
