//! [Problem 28](https://projecteuler.net/problem=28) solver.

#![warn(
    bad_style,
    unused,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

// 43 44 45 46 47 48 49
// 42 21 22 23 24 25 26
// 41 20  7  8  9 10 27
// 40 19  6  1  2 11 28
// 39 18  5  4  3 12 29
// 38 17 16 15 14 13 30
// 37 36 35 34 33 32 31

// n = square size (1 by 1 => n = 1, 3 by 3 => n = 9)
// Upper right => 1, 9, 25, 49
//             => ur[n] := n^2
// Upper left  => 1, 7, 21, 43 = 1 - 0, 9 - 2, 25 - 4, 49 - 6
//             => ul[n] := ur[n] - (n - 1) = n^2 - n + 1
// Lower left  => 1, 5, 17, 37 = 1 - 0, 7 - 2, 21 - 4, 43 - 6
//             => ll[n] := ul[n] - (n - 1) = n^2 - 2n + 2
// Lower right => 1, 3, 13, 31 = 1 - 0, 5 - 2, 17 - 4, 37 - 6
//             => lr[n] := ll[n] - (n - 1) = n^2 - 3n + 3

// sum[n] = ur[n] + ul[n] + ll[n] + lr[n] = 4n^2 - 6n + 6

// n = 2k - 1
// sum[n] = 16k^2 - 28k + 16

// N := (n + 1) / 2
// ans[n] = \sum_{k=1}^{N} sum[k] - 3
//        = 1/6 (4n^3 + 3n^2 + 8n - 9)
fn compute(n: u32) -> u32 {
    (4 * n * n * n + 3 * n * n + 8 * n - 9) / 6
}

fn solve() -> String {
    compute(1001).to_string()
}

common::problem!("669171001", solve);

#[cfg(test)]
mod tests {
    #[test]
    fn five() {
        assert_eq!(101, super::compute(5));
    }
}
