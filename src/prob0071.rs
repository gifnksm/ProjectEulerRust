#![crate_id = "prob0071"]
#![crate_id = "prob0071"]
#![crate_type = "rlib"]
#![crate_type = "rlib"]

pub static EXPECTED_ANSWER: &'static str = "428570";

pub fn solve() -> ~str {
    let limit = 1000000;
    let mut max_n = 0;
    let mut max_d = 1;
    for d in range(limit - 7, limit).rev() {
        let n = if 3 * d % 7 == 0 { 3 * d / 7 - 1 } else { 3 * d / 7 };
        if n *max_d > max_n * d {
            max_n = n;
            max_d = d;
        }
    };
    return max_n.to_str();
}
