//! [Problem 26](https://projecteuler.net/problem=26) solver.

#![warn(
    bad_style,
    unused,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

fn get_cycle_len(n: u32) -> u32 {
    if n == 1 {
        return 1;
    }
    let mut buf = vec![None; n as usize];
    let mut rem = 1;
    let mut idx = 1u32;
    loop {
        let new_rem = rem % n;
        match buf[new_rem as usize] {
            Some(i) => {
                return idx - i;
            }
            None => {
                buf[new_rem as usize] = Some(idx);
            }
        }
        idx += 1;
        rem = new_rem * 10;
    }
}

fn compute(limit: u32) -> u32 {
    (2..limit).max_by_key(|&n| get_cycle_len(n)).unwrap()
}

fn solve() -> String {
    compute(1000).to_string()
}

common::problem!("983", solve);

#[cfg(test)]
mod tests {
    #[test]
    fn ten() {
        assert_eq!(7, super::compute(10));
    }
}
