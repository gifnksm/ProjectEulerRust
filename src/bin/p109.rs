//! [Problem 109](https://projecteuler.net/problem=109) solver.

#![warn(
    bad_style,
    unused,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

use polynomial::Polynomial;

fn count_way(score: u32) -> u32 {
    let mut single = vec![0u32; 26];
    let mut double = vec![0; 51];
    let mut triple = vec![0; 61];
    let mut dup = vec![0; 121];
    for i in 1..21 {
        single[i] = 1;
        double[i * 2] = 1;
        triple[i * 3] = 1;
        dup[i * 2] += 1;
        dup[i * 4] += 1;
        dup[i * 6] += 1;
    }
    single[25] = 1;
    double[50] = 1;
    dup[50] += 1;
    dup[100] += 1;

    let single = Polynomial::new(single);
    let double = Polynomial::new(double);
    let triple = Polynomial::new(triple);
    let dup = Polynomial::new(dup);

    let p_all = &single + &double + &triple;
    let p1 = double.clone();
    let p2 = &double * &p_all;
    let p3 = &double
        * Polynomial::new(
            (&p_all * &p_all + &dup)
                .data()
                .iter()
                .map(|&n| n / 2)
                .collect(),
        );
    let total = p1 + p2 + p3;
    total.data().iter().take(score as usize).sum()
}

fn solve() -> String {
    count_way(100).to_string()
}

common::problem!("38182", solve);

#[cfg(test)]
mod tests {
    #[test]
    fn example() {
        assert_eq!(11, super::count_way(6));
        assert_eq!(42336, super::count_way(171));
    }
}
