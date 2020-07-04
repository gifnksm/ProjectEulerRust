//! [Problem 137](https://projecteuler.net/problem=137) solver.
//!
//! ```math
//! A_F(x)     = x F_1 + x^2 F_2 + x^3 F_3 + \dots
//! x A_F(x)   = x^2 F_1 + x^3 F_2 + x^4 F_3 + \dots
//! x^2 A_F(x) = x^3 F_1 + x^4 F_2 + x^5 F_3 + \dots
//! (1 - x - x^2) A_F(x) = x F_1 + x^2 (F_2 - F_1) + x^3 (F_3 - F_2 - F_1) + \dots
//! ```
//!
//! `F_k = F_{k-1} + F_k`, `F_1 = F_2 = 1` より、
//!
//! ```math
//! (1 - x - x^2) A_F(x) = x
//! ```
//!
//! `A_F(x)` は正の整数なので `n := A_F(x) > 0` とおくと、
//! 以下の二次方程式を得る。
//!
//! ```math
//! n x^2 + (n + 1) x - n = 0
//! ```
//!
//! この方程式が有理数解をもつのは、判別式 `D` が平方数の場合であり、
//! ある整数 `m` を用いると以下のように表せる場合である。
//!
//! ```math
//! D = (n+1)^2 + 4n^2 = m^2
//! (5n+1)^2 - 5m^2 = -4
//! ```
//!
//! これは Pell 方程式であり、解を列挙すれば良い。
//!
//! `p := 5n + 1`, `q := m` とおくと、
//! `p_0 = 1`, `q_0 = 1` より、
//! ```math
//! p_{k+1} = \frac{3p_k + 5q_k}{2}
//! q_{k+1} = \frac{p_k + 3q_k}{2}
//! ```
//!
//! となり、これが一般解である。

#![warn(
    bad_style,
    unused,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

fn compute(i: usize) -> u64 {
    itertools::unfold((1, 1), |state| {
        let next = ((3 * state.0 + 5 * state.1) / 2, (state.0 + 3 * state.1) / 2);
        *state = next;
        Some(next)
    })
    .filter_map(|(p, q)| if p % 5 == 1 { Some((p / 5, q)) } else { None })
    .nth(i)
    .unwrap()
    .0
}

fn solve() -> String {
    compute(14).to_string()
}

common::problem!("1120149658760", solve);

#[cfg(test)]
mod tests {
    #[test]
    fn tenth_sol() {
        assert_eq!(74049690, super::compute(9));
    }
}
