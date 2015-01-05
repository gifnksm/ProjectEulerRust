//! [Problem 133](https://projecteuler.net/problem=133) solver.
//!
//! # 解析
//!
//! ## 定理 1
//! `R(km)` (`k > 1`) は `R(m)` の倍数である。
//!
//! ## 証明
//!
//! `R(k)` は以下のように定義される。
//!
//! ```math
//! R(k) = (10^k - 1) / 9
//! ```
//!
//! 上記より、
//!
//! ```math
//! R(km) / R(k) = (10^(km) - 1) / 9R(a)
//! ```
//!
//! である。ここで
//!
//! ```math
//! 9R(k) + 1 = 10^k
//! ```
//!
//! と書けることから、両辺を `m` 乗して
//!
//! ```math
//! 10^(km) = (9R(k) + 1)^m
//! ```
//!
//! を得る。上記を代入することで以下を得る。
//!
//! ```math
//! R(km) / R(k) = ((9R(k) + 1)^m - 1) / 9R(a)
//! ```
//!
//! 右辺は整数であるため、`R(km)` は `R(k)` で割り切れる■
//!
//! 定理 1 を `R(10^n)` に適用すると、以下となる。
//!
//! `n \geq m` のとき、 `R(10^n)` は `R(10^m) で割り切れる。
//!
//! problem 129 で求めた `A(n)` の素因数が 2 または 5 のみの場合に、
//! `R(n)` は `R(10^k)` を割り切る。

#![warn(bad_style,
        unused, unused_extern_crates, unused_import_braces,
        unused_qualifications, unused_results, unused_typecasts)]

#![feature(phase)]

#[phase(plugin, link)] extern crate common;
extern crate prime;

use std::iter::{self, AdditiveIterator};
use prime::{Factorize, PrimeSet};

fn a(n: u64) -> u64 {
    if n == 1 { return 1 }

    iter::Unfold::new((1, 1), |st| {
            let (x, k) = *st;
            *st = ((x * 10 + 1) % n, k + 1);
            Some((x, k))
        }).find(|&(x, _)| x == 0)
        .unwrap()
        .1
}

fn solve() -> String {
    let ps = PrimeSet::new();

    let sum = ps.iter()
        .skip_while(|&p| p <= 5)
        .take_while(|&p| p < 100000)
        .filter(|&p| {
            a(p).factorize(&ps)
                .any(|(b, _e)| b != 2 && b != 5)
        }).sum();

    (sum + 2 + 3 + 5).to_string()
}

problem!("453647705", solve);
