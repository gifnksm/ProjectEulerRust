//! [Problem 135](https://projecteuler.net/problem=135) solver.
//!
//! # 解析
//!
//! `x`, `y`, `z` は等差数列なので、等差 `k` を用いて以下のように表せる。
//!
//! ```math
//! x = 2k + z
//! y = k + z
//! ```
//!
//! すなわち、
//!
//! ```math
//!   x^2 - y^2 - z^2
//! = (z+2k)^2 - (z+k)^2 -z^2
//! = 3k^2 + 2kz - z^2
//! = (3k - z)(k + z)
//! ```
//!
//! となる。 `n > 0` なので、`3k > z` である。
//!
//! ```math
//! p := 3k - z
//! q := k + z
//! ```
//!
//! とおくと、以下を得る。
//!
//! ```math
//! n = pq
//! k = ( q + p) / 4
//! z = (3q - p) / 4
//! ```
//!
//! `n > 0` なので、`3q > p` である。
//! さらに、`k`, `z` は整数なので、
//!
//! ```math
//!  q + p ≡ 0 (mod 4)
//! 3q - p ≡ 0 (mod 4)
//! ```
//!
//! である。 `q`, `p` の4を法にした剰余をそれぞれ `r`, `s` とおくと、
//! 以下が成立する。
//!
//! * `r = 0` の場合、`s = 0`
//! * `r = 1` の場合、`s = 3`
//! * `r = 2` の場合、`s = 2`
//! * `r = 3` の場合、`s = 1`
//!
//! これらの関係性より、
//! 3以下の正の整数 `r` と任意の正の整数 `a`, `b` を用いて、
//! 以下のように書ける。
//!
//! ```
//! q = 4a + r
//! p = 4b - r
//! ```
//!
//! # 解法
//!
//! `q` と `p` の数値を変化させ、
//! 各 `n = qp < 1000000` となる `q`, `p` の数を数える。

#![warn(
    bad_style,
    unused,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

fn num_solutions(limit: usize) -> Vec<u32> {
    let mut cnt = vec![0; limit];
    for q in 1..limit {
        let r = (4 - (q % 4)) % 4;
        if q * r >= limit {
            continue;
        }
        for p in (r..(q * 3)).step_by(4) {
            let n = q * p;
            if n >= limit {
                break;
            }
            cnt[n] += 1;
        }
    }
    cnt
}

fn solve() -> String {
    let limit = 1000000;
    let cnt = 10;
    num_solutions(limit)
        .iter()
        .filter(|&&n| n == cnt)
        .count()
        .to_string()
}

common::problem!("4989", solve);

#[cfg(test)]
mod tests {
    #[test]
    fn first_sol() {
        let pos = super::num_solutions(2000)
            .iter()
            .position(|&n| n == 10)
            .unwrap();
        assert_eq!(1155, pos);
    }
}
