//! [Problem 31](https://projecteuler.net/problem=31) solver.

#![warn(
    bad_style,
    unused,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

fn count_ways(sum: u32, coins: &[u32]) -> u32 {
    if coins.len() == 1 {
        return 1;
    }

    let mut ans = 0;
    for n in 0..(sum / coins[0] + 1) {
        let d = sum - n * coins[0];
        ans += count_ways(d, &coins[1..]);
    }
    ans
}

fn compute(sum: u32) -> u32 {
    let coins = &[200, 100, 50, 20, 10, 5, 2, 1];
    count_ways(sum, coins)
}

fn solve() -> String {
    compute(200).to_string()
}

common::problem!("73682", solve);

#[cfg(test)]
mod tests {
    #[test]
    fn four() {
        assert_eq!(3, super::compute(4));
    }
}
