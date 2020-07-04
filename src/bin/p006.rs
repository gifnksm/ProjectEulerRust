//! [Problem 6](https://projecteuler.net/problem=6) solver.

#![warn(
    bad_style,
    unused,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

fn sum_of_square(n: u32) -> u32 {
    n * (n + 1) * (2 * n + 1) / 6
}
fn sum_of_seq(n: u32) -> u32 {
    n * (n + 1) / 2
}
fn square_of_sum(n: u32) -> u32 {
    let s = sum_of_seq(n);
    s * s
}

fn compute(n: u32) -> u32 {
    square_of_sum(n) - sum_of_square(n)
}

fn solve() -> String {
    compute(100).to_string()
}

common::problem!("25164150", solve);

#[cfg(test)]
mod tests {
    #[test]
    fn sofs_ten() {
        assert_eq!(2640, super::compute(10));
    }
}
