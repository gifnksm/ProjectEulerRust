//! [Problem 132](https://projecteuler.net/problem=132) solver.
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

#![warn(
    bad_style,
    unused,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

use integer::Integer;
use prime::PrimeSet;

pub fn solve() -> String {
    PrimeSet::new()
        .iter()
        .filter(|&p| 10.mod_pow(&10u64.pow(9), &(9 * p)) == 1)
        .take(40)
        .sum::<u64>()
        .to_string()
}

common::problem!("843296", solve);
