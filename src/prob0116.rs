#![crate_id = "prob0116"]
#![crate_type = "rlib"]

extern crate collections;
use std::iter;
use collections::HashMap;

pub static EXPECTED_ANSWER: &'static str = "20492570929";

fn count(len: uint, unit: uint, map: &mut HashMap<(uint, uint), uint>) -> uint {
    match map.find(&(len, unit)) {
        Some(&x) => return x,
        None => {}
    }

    if len < unit { map.insert((len, unit), 1); return 1; }

    let mut sum = 0;
    for i in iter::range_inclusive(0, len - unit) { // most left block position
        sum += count(len - (unit + i), unit, map);
    }
    sum += 1;
    map.insert((len, unit), sum);
    sum
}

fn count_red(len: uint, map: &mut HashMap<(uint, uint), uint>) -> uint { count(len, 2, map) - 1 }
fn count_green(len: uint, map: &mut HashMap<(uint, uint), uint>) -> uint { count(len, 3, map) - 1 }
fn count_blue(len: uint, map: &mut HashMap<(uint, uint), uint>) -> uint { count(len, 4, map) - 1 }
fn count_all(len: uint, map: &mut HashMap<(uint, uint), uint>) -> uint {
    count_red(len, map) + count_green(len, map) + count_blue(len, map)
}

pub fn solve() -> ~str {
    let mut map = HashMap::new();
    count_all(50, &mut map).to_str()
}

#[cfg(test)]
mod tests {
    use super::{count_red, count_green, count_blue, count_all};
    use collections::HashMap;

    #[test]
    fn count() {
        let mut map = HashMap::new();
        assert_eq!(7, count_red(5, &mut map));
        assert_eq!(3, count_green(5, &mut map));
        assert_eq!(2, count_blue(5, &mut map));
        assert_eq!(12, count_all(5, &mut map));
    }
}
