#![crate_id = "prob0115"]
#![crate_type = "rlib"]

extern crate collections;
extern crate prob0114;

use std::iter;
use collections::HashMap;
use prob0114::get_cnt;

pub static EXPECTED_ANSWER: &'static str = "168";

pub fn solve() -> String {
    let mut map = HashMap::new();
    iter::count(1u, 1)
        .filter(|&n| get_cnt(n, 50, &mut map) > 1000000)
        .next()
        .unwrap()
        .to_str()
}

#[cfg(test)]
mod tests {
    use collections::HashMap;
    use prob0114::get_cnt;

    #[test]
    fn small_len() {
        let mut map = HashMap::new();
        assert_eq!(1, get_cnt(1, 3, &mut map));
        assert_eq!(1, get_cnt(2, 3, &mut map));
        assert_eq!(2, get_cnt(3, 3 , &mut map));
        assert_eq!(4, get_cnt(4, 3, &mut map));
        assert_eq!(17, get_cnt(7, 3, &mut map));
        assert_eq!(673135, get_cnt(29, 3, &mut map));
        assert_eq!(1089155, get_cnt(30, 3, &mut map));
        assert_eq!(880711, get_cnt(56, 10, &mut map));
        assert_eq!(1148904, get_cnt(57, 10, &mut map));
    }
}
