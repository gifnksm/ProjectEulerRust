//! [Problem 134](https://projecteuler.net/problem=134) solver.
//!
//! # 解析
//!
//! 連続する素数 `p1=19`, `p2=23` の場合を例にして考える。
//!
//! `23n が `10k + 9` と書ける場合、
//!
//! ```math
//! 3 * n = 9 (mod 10)
//! => 23n = 23(10k + 3) = 230k + 69
//! ```
//!
//! である。
//!
//! `230k + 69` が `100m + 19` と書ける場合、
//!
//! ```math
//! 3 * k + 6 = 1 (mod 10)
//! => 230k + 69 = 230(10m + 5) + 69 = 2300m + 1219
//! ```
//!
//! よって、`S = 1219` である。
//!
//! # 解法
//!
//! `n`, `m` について以下を満たす `x` を、`x(n, m)` と書く。
//!
//! ```math
//! n * x = m (mod 10)
//! ```
//!
//! `x_i` は `x` の `i` 桁目を表すとする。
//! 以下の漸化式により、`S` を求めることができる。
//! ここで、 `n` は `p1` の桁数である。
//!
//! ```math
//! a[1] = x(p2, p1_1)
//! b[1] = a[1] * p2
//! a[i] = x(p2, p1_i - b[i-1]_i)
//! b[i] = a[i] * p2 * 10^(i-1) + b[i-1]
//! S = b[n]
//! ```
//!
//! ## 例 `p1=19` と `p2=23` の場合
//!
//! ```math
//! a[1] = x(23, 9) = 3
//! b[1] = 3*23 = 69
//! a[2] = x(23, 1 - 6) = 5
//! b[2] = 50*23 + 69 = 1219
//! S = b[2] = 1219
//! ```
//!
//! ## 例 `p1=11` と `p2=13` の場合
//!
//! ```math
//! a[1] = x(13, 1) = 7
//! b[1] = 7*13 = 91
//! a[2] = x(13, 1 - 9) = 4
//! b[2] = 40*13 + 91 = 611
//! S = b[2] = 611
//! ```
//!
//! ## 例 `p1=97` と `p2=101` の場合
//!
//! ```math
//! a[1] = x(101, 7) = 7
//! b[1] = 7*101 = 707
//! a[2] = x(101, 9 - 0) = 9
//! b[2] = 90*101 + 707 = 9797
//! S = b[2] = 9797
//! ```
//!
//! ## 例 `p1=101` と `p2=103` の場合
//!
//! ```math
//! a[1] = x(103, 1) = 7
//! b[1] = 7*103 = 1751
//! a[2] = x(103, 0 - 5) = 5
//! b[2] = 50*103 + 1751 = 6901
//! a[3] = x(103, 1 - 9) = 4
//! b[3] = 400*103 + 6901 = 48101
//! S = b[3] = 48101
//! ```

#![warn(
    bad_style,
    unused,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

use num_integer::Integer;
use prime::PrimeSet;

fn solve() -> String {
    let min_limit = 5;
    let max_limit = 1000000;

    let mut mod_map = vec![vec![0; 10]; 10];
    for &b in &[1, 3, 7, 9] {
        for n in 1..10u64 {
            mod_map[b as usize][((b * n) % 10) as usize] = n;
        }
    }

    let mut sum = 0;
    let ps = PrimeSet::new();
    let pairs = ps
        .iter()
        .zip(ps.iter().skip(1))
        .skip_while(|&(p1, _p2)| p1 < min_limit)
        .take_while(|&(p1, _p2)| p1 <= max_limit);

    for (p1, p2) in pairs {
        if p1 == 3 {
            continue;
        }
        let xmap = &mod_map[(p2 % 10) as usize];
        let mut a;
        let mut b = 0;
        let mut p1_digit = p1;
        let mut coef = 1;
        for _ in 0..(p1.to_string().len() as u64) {
            let (d, m) = p1_digit.div_rem(&10);
            p1_digit = d;
            a = xmap[((10 + m - (b / coef) % 10) % 10) as usize];
            b += a * p2 * coef;
            coef *= 10;
        }
        sum += b;
    }
    sum.to_string()
}

common::problem!("18613426663617118", solve);
