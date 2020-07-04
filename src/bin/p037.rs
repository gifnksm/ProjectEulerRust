//! [Problem 37](https://projecteuler.net/problem=37) solver.

#![warn(
    bad_style,
    unused,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

use integer::Integer;
use prime::PrimeSet;

fn is_r2l(ps: &PrimeSet, n: u64) -> bool {
    let mut itr = n / 10;
    while itr > 0 {
        if !ps.contains(itr) {
            return false;
        }
        itr /= 10;
    }
    true
}

fn compute() -> u64 {
    let ps = PrimeSet::new();
    let mut l2r_mat = vec![vec![2, 3, 5, 7]];
    let mut order = 10;

    loop {
        let mut result = vec![];
        for &p in l2r_mat.last().unwrap() {
            // 2 can only be appeared as the most left digits
            if p.into_digits(10).next_back() == Some(2) {
                continue;
            }

            let ds = [1, 2, 3, 5, 7, 9];
            for &d in &ds {
                let n = order * d + p;
                if ps.contains(n) {
                    result.push(n);
                }
            }
        }

        if result.is_empty() {
            break;
        }
        l2r_mat.push(result);
        order *= 10;
    }

    l2r_mat
        .into_iter()
        .flat_map(|l2r| l2r.into_iter())
        .filter(|&n| n >= 10)
        .filter(|&n| is_r2l(&ps, n))
        .sum()
}

fn solve() -> String {
    compute().to_string()
}

common::problem!("748317", solve);

#[cfg(test)]
mod tests {
    use prime::PrimeSet;

    #[test]
    fn is_r2l() {
        let ps = PrimeSet::new();
        assert_eq!(true, super::is_r2l(&ps, 3797));
        assert_eq!(false, super::is_r2l(&ps, 151));
    }
}
