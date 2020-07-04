//! [Problem 120](https://projecteuler.net/problem=120) solver.

#![warn(
    bad_style,
    unused,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

use num_integer::Integer;

// f(a, n) := (a-1)^n + (a+1)^n
//
// f(a, 1) = 2a
// f(a, 2) = 2(a^2 + 1)
// f(a, 3) = 2(a^3 + 3a)
// f(a, 4) = 2(a^4 + 6a^2 + 1)
// f(a, 5) = 2(a^5 + 10a^3 + 5a)
// f(1, 6) = 2(a^6 + 15a^4 + 15a^2 + 1)
//
// f(a, n) =
//   if n is even:
//     2(a^n + nC2 a^(n-2) + ... + 1)
//   else
//     2(a^n + nC2 a^(n-2) + ... + nC(n-1) a)
//
// f(a, 2k + 1) ≡ 1   mod a^2
// f(a, 2k)     ≡ 4ka mod a^2
//
// => rmax = 2 nmax a < a^2
//    nmax = max[k; k < a/ 2]
//
// nmax =
//   if a is even:
//     (a - 2) / 2
//   else
//     (a - 1) / 2
//
// rmax =
//   if a is even:
//     a (a - 2)
//   else:
//     a (a - 1)

fn rmax(a: u32) -> u32 {
    if a.is_even() {
        a * (a - 2)
    } else {
        a * (a - 1)
    }
}

fn solve() -> String {
    (3..1001).map(rmax).sum::<u32>().to_string()
}

common::problem!("333082500", solve);

#[cfg(test)]
mod tests {
    #[test]
    fn rmax() {
        assert_eq!(42, super::rmax(7));
    }
}
