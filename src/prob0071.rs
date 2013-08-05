#[link(name = "prob0071", vers = "0.0")];
#[crate_type = "lib"];



use std::uint;

pub static EXPECTED_ANSWER: &'static str = "428570";

pub fn solve() -> ~str {
    let limit = 1000000;
    let mut max_n = 0;
    let mut max_d = 1;
    do uint::range_rev(limit, limit - 7) |d| {
        let n = if 3 * d % 7 == 0 { 3 * d / 7 - 1 } else { 3 * d / 7 };
        if n *max_d > max_n * d {
            max_n = n;
            max_d = d;
        }
        true
    };
    return max_n.to_str();
}
