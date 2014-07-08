//! Problem 131 (https://projecteuler.net/problem=131)
//! # 解析
//!
//! ```notrust math
//! n^3 + n^2p = m^3
//! ```
//!
//! とおく。
//!
//! ## 定理 1
//! `n` と `p` は互いに素である。
//!
//! ## 証明
//!
//! `p` は素数なので、`n` と `p` が互いに素でない場合、
//! ある自然数 `k` を使って `n = kp` と書ける。
//! このとき、
//!
//! ```notrust math
//! n^3 + n^2p = p^3k^2(k + 1) = m^3
//! k^3 + k^2 = (m / p)^3
//! ```
//!
//! となる。`k^3` の次に大きい立方数は `(k+1)^3` なので、
//! `k^3 + k^2` は立方数ではなく、矛盾する。
//! よって、`n` と `p` は互いに素である■
//!
//! ## 定理 2
//!
//! `n` は立方数である。また、`p` は立方数の差として表される。
//!
//! ## 証明
//!
//! `n` は、互いに素である因数 `s0`, `s1`, `s2` を用いて、以下のように書ける。
//!
//! ```notrust math
//! n = s0^(3e0) * s1^(3e1+1) * s2^(3e2+2)
//! ```
//!
//! このとき、`n` と `p` は互いに素であるため、
//! `n+p` は以下のように因数分解できなければならない。
//!
//! ```notrust math
//! n + p = s0^(3e0) * s1^(3e1+1) * s2^(3e2+2) + p
//!       = s1^(3e'1+1) * s2^(3e'2+2) * k^3
//! ```
//!
//! 上式を整理して、以下を得る。
//!
//! ```notrust math
//! p = s1^(3e''1+1) * s2^(3e''2+2) * (k^3 - p^3)
//! ```
//!
//! 右辺は合成数ではないため、`s1^(3e''1+1) * s2(3e''2+2) = 1` である。
//! すなわち、`n = s0^(3e0)` と書け、立方数である■
//! ## 定理3
//!
//! `p` は任意の数 `q` を用いて以下のように表される。
//!
//! `p = 3q^2 + 3q + 1`
//!
//! ## 証明
//!
//! 定理2 より、`p` は立方根の差として表される素数である。
//! `p = r^3 - q^3` と置くと、以下を得る。
//!
//! ```notrust math
//! p = (r-q)(r^2+rq+q^2)
//! ```
//!
//! `r^2 + rq + q^2 > 1` より、 `r - q = 1` である。
//! すなわち、
//!
//! ```notrust math
//! p = (q+1)^2 + q(q+1) + q^2
//!   = 3q^2 + 3q + 1
//! ```
//!
//! である ■
//!
//! # 解法
//!
//! `3q^2 + 3q + 1` を `q` について計算し、素数のものを列挙する。
//! `q` が

#![crate_name = "prob0131"]
#![crate_type = "rlib"]

extern crate math;

use std::iter;
use math::prime::Prime;

pub static EXPECTED_ANSWER: &'static str = "173";

pub fn solve() -> String {
    let limit = 1000000;
    let ps = Prime::new();

    iter::count(1u, 1)
        .map(|q| 3*q*q + 3*q + 1)
        .take_while(|&p| p <= limit)
        .filter(|&p| ps.contains(p) )
        .count()
        .to_str()
}
