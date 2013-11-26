#[link(name = "prob0049", vers = "0.0", package_id = "prob0049")];
#[crate_type = "lib"];

extern mod extra;
extern mod math;

use math::{numconv, prime};

pub static EXPECTED_ANSWER: &'static str = "296962999629";

pub fn solve() -> ~str {
    let d = 3330;
    let (p1, p2, p3) = prime::iter()
        .skip_while(|&p| p < 1000)
        .take_while(|&p| p <= 9999 - 2 * d)
        .filter(|&p| p != 1487)
        .map(|p| (p, p + d, p + d + d))
        .filter(|&(_p1, p2, p3)| prime::contains(p3) && prime::contains(p2))
        .filter(|&(p1, p2, p3)| {
            let hs1 = numconv::to_digit_histogram(p1);
            (hs1 == numconv::to_digit_histogram(p2)) &&
                (hs1 == numconv::to_digit_histogram(p3))
        }).next().unwrap();
    return format!("{}{}{}", p1, p2, p3)
}
