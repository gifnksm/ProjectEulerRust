#[link(name = "prob0049", vers = "0.0")];
#[crate_type = "lib"];

extern mod extra;
extern mod common;

use common::prime;
use common::calc;

pub static expected_answer: &'static str = "296962999629";

pub fn solve() -> ~str {
    let d = 3330;
    let (p1, p2, p3) = prime::iter()
        .skip_while(|&p| p < 1000)
        .take_while(|&p| p <= 9999 - 2 * d)
        .filter(|&p| p != 1487)
        .transform(|p| (p, p + d, p + d + d))
        .filter(|&(_p1, p2, p3)| prime::contains(p3) && prime::contains(p2))
        .filter(|&(p1, p2, p3)| {
            let hs1 = calc::digit_histogram(p1);
            (hs1 == calc::digit_histogram(p2)) &&
                (hs1 == calc::digit_histogram(p3))
        }).next().unwrap();
    return fmt!("%u%u%u", p1, p2, p3)
}
