//! Problem 132 (https://projecteuler.net/problem=132)
//!
//! # 解析
//!
//! ```math
//! R(k) = (10^k - 1) / 9
//! ```
//!
//! 素数 `p` が `R(k)` の因数の場合、以下が成立する。
//!
//! ```math
//! (10^k - 1) / 9 \equiv 0 (mod p)
//! => 10^k - 1 \equiv 0 (mod 9p)
//! => 10^k \qeuiv 1 (mod 9p)
//! ```
//!
//! # 解法
//!
//! 冪剰余 (Modular exponation) を求める。

#[crate_id = "prob0132"];
#[crate_type = "rlib"];

extern mod math;

use std::num;
use std::iter::AdditiveIterator;
use math::prime::Prime;

pub static EXPECTED_ANSWER: &'static str = "843296";

fn mod_pow(mut base: uint, mut exp: uint, modulo: uint) -> uint {
    let mut result = 1;

    while exp > 0 {
        if exp.is_odd() {
            result = (result * base) % modulo;
        }
        exp >>= 1;
        base = (base * base) % modulo;
    }
    result
}

pub fn solve() -> ~str {
    Prime::new().iter()
        .filter(|&p| mod_pow(10, num::pow(10u, 9), 9 * p) == 1)
        .take(40)
        .sum()
        .to_str()
}

#[cfg(test)]
mod test {
    use std::num;

    #[test]
    fn mod_pow() {
        for b in range(1u, 10) {
            for e in range(0u, 5) {
                for r in range(10u, 100) {
                    assert_eq!(num::pow(b, e) % r, super::mod_pow(b, e, r));
                }
            }
        }
    }
}
