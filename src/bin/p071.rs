//! [Problem 71](https://projecteuler.net/problem=71) solver.

#![warn(
    bad_style,
    unused,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

fn compute(limit: u64) -> u64 {
    let mut max_n = 0;
    let mut max_d = 1;
    for d in (limit - 7..limit).rev() {
        let n = if 3 * d % 7 == 0 {
            3 * d / 7 - 1
        } else {
            3 * d / 7
        };
        if n * max_d > max_n * d {
            max_n = n;
            max_d = d;
        }
    }
    max_n
}

fn solve() -> String {
    compute(1000000).to_string()
}

common::problem!("428570", solve);

#[cfg(test)]
mod tests {
    #[test]
    fn eight() {
        assert_eq!(2, super::compute(8));
    }
}
