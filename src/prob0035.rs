#![crate_name = "prob0035"]
#![crate_type = "rlib"]

extern crate math;

use math::numconv;
use math::prime::Prime;

pub static EXPECTED_ANSWER: &'static str = "55";

#[inline(always)]
fn is_circular_prime(prime: &Prime, n: uint) -> bool {
    let ds = numconv::to_digits(n, 10).collect::<Vec<uint>>();
    let mut buf = ds.clone();

    for i in range(1, ds.len()) {
        for j in range(0, buf.len()) {
            *buf.get_mut(j) = *ds.get((i + j) % ds.len());
        }
        if !prime.contains(numconv::from_digits(buf.as_slice(), 10)) { return false; }
    }

    true
}

pub fn solve() -> String {
    let prime = Prime::new();
    return prime.iter()
        .take_while(|&p| p < 1000000)
        .filter(|&n| is_circular_prime(&prime, n))
        .count()
        .to_string();
}
