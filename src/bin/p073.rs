//! [Problem 73](https://projecteuler.net/problem=73) solver.

#![warn(
    bad_style,
    unused,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

fn count_between(a: (u32, u32), b: (u32, u32), max_denom: u32) -> u32 {
    if a.1 + b.1 > max_denom {
        return 0;
    }
    let mid = (a.0 + b.0, a.1 + b.1);
    count_between(a, mid, max_denom) + count_between(mid, b, max_denom) + 1
}

fn solve() -> String {
    count_between((1, 3), (1, 2), 12000).to_string()
}

common::problem!("7295372", solve);

#[cfg(test)]
mod tests {
    #[test]
    fn eight() {
        assert_eq!(3, super::count_between((1, 3), (1, 2), 8));
    }
}
