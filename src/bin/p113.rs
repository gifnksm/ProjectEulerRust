//! [Problem 113](https://projecteuler.net/problem=113) solver.

#![warn(
    bad_style,
    unused,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

fn num_increasing(len: usize) -> u64 {
    let mut buf = vec![[0; 10]; len];
    for b in &mut buf[0] {
        *b = 1;
    }

    for i in 1..len {
        let mut s = 0;
        for d in (0..buf[i].len()).rev() {
            s += buf[i - 1][d];
            buf[i][d] = s;
        }
    }

    let sum: u64 = (0..buf[len - 1].len()).map(|d| buf[len - 1][d]).sum();
    sum - 1 // all zero
}

fn num_decreasing(len: usize) -> u64 {
    let mut buf = vec![[0; 11]; len];
    for b in &mut buf[0] {
        *b = 1;
    }

    for i in 1..len {
        let mut s = 0;
        for d in 0..buf[i].len() {
            s += buf[i - 1][d];
            buf[i][d] = s;
        }
    }

    let sum: u64 = (0..buf[len - 1].len()).map(|d| buf[len - 1][d]).sum();

    sum - (len as u64) - 1 // all zero
}

fn num_nonbouncy(len: usize) -> u64 {
    let num_incr = num_increasing(len);
    let num_decr = num_decreasing(len);
    let num_incr_and_decr = 9 * len as u64;
    num_incr + num_decr - num_incr_and_decr
}

fn solve() -> String {
    num_nonbouncy(100).to_string()
}

common::problem!("51161058134250", solve);

#[cfg(test)]
mod tests {
    #[test]
    fn test_nonbouncy() {
        assert_eq!(12951, super::num_nonbouncy(6));
        assert_eq!(277032, super::num_nonbouncy(10));
    }
}
