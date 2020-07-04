//! [Problem 92](https://projecteuler.net/problem=92) solver.

#![warn(
    bad_style,
    unused,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

use integer::Integer;

fn square_digit_sum(n: u32) -> u32 {
    n.into_digits(10).map(|x| x * x).sum()
}

fn is_reach_89(n: u32, map: &mut [Option<bool>]) -> bool {
    if (n as usize) >= map.len() {
        return is_reach_89(square_digit_sum(n), map);
    }

    if let Some(b) = map[n as usize] {
        return b;
    }

    let result = is_reach_89(square_digit_sum(n), map);
    map[n as usize] = Some(result);
    result
}

fn create_map(limit: u32) -> Vec<Option<bool>> {
    let mut map = vec![None; (limit - 1).to_string().len() * 81 + 1];
    map[0] = Some(false);
    map[1] = Some(false);
    map[89] = Some(true);
    map
}

fn solve() -> String {
    let limit = 10000000;
    let mut map = create_map(limit);
    (1..limit)
        .filter(|&n| is_reach_89(n, &mut map))
        .count()
        .to_string()
}

common::problem!("8581146", solve);

#[cfg(test)]
mod tests {
    #[test]
    fn is_reach_89() {
        let mut map = super::create_map(100);
        assert!(!super::is_reach_89(44, &mut map));
        assert!(super::is_reach_89(85, &mut map));
    }
}
