//! Problem 132 (https://projecteuler.net/problem=132)
//!
//! # 解析
//!
//! ``` {.math .notrust}
//! R(k) = (10^k - 1) / 9
//! ```
//!
//! 素数 `p` が `R(k)` の因数の場合、以下が成立する。
//!
//! ``` {.math .notrust}
//! (10^k - 1) / 9 \equiv 0 (mod p)
//! => 10^k - 1 \equiv 0 (mod 9p)
//! => 10^k \qeuiv 1 (mod 9p)
//! ```
//!
//! # 解法
//!
//! 冪剰余 (Modular exponation) を求める。

#![crate_id = "prob0132"]
#![crate_id = "prob0132"]
#![crate_type = "rlib"]
#![crate_type = "rlib"]

extern crate math;

use std::num;
use std::iter::AdditiveIterator;
use math::arith;
use math::prime::Prime;

pub static EXPECTED_ANSWER: &'static str = "843296";

pub fn solve() -> ~str {
    Prime::new().iter()
        .filter(|&p| arith::mod_pow(10, num::pow(10u, 9), 9 * p) == 1)
        .take(40)
        .sum()
        .to_str()
}

