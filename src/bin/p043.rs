//! [Problem 43](https://projecteuler.net/problem=43) solver.

#![warn(
    bad_style,
    unused,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

use integer::Integer;
use std::iter;

const RADIX: u32 = 10;

#[derive(Copy, Clone)]
struct Pandigimal {
    used: [bool; RADIX as usize],
    num: [u32; RADIX as usize],
    len: usize,
}

impl Pandigimal {
    fn new() -> Pandigimal {
        Pandigimal {
            used: [false; RADIX as usize],
            num: [0; RADIX as usize],
            len: 0,
        }
    }

    fn from_u64(n: u32, len: usize) -> Option<Pandigimal> {
        let it = n.into_digits(RADIX).chain(iter::repeat(0));
        Pandigimal::new().join_all(it.map(|x| x).take(len))
    }

    fn to_u64(&self) -> u64 {
        Integer::from_digits(self.num().iter().map(|&x| x as u64), RADIX as u64)
    }

    fn is_used(&self, n: u32) -> bool {
        assert!(n < RADIX);
        self.used[n as usize]
    }

    fn num(&self) -> &[u32] {
        &self.num[..self.len]
    }

    fn join(&self, d: u32) -> Option<Pandigimal> {
        assert!(d < RADIX);
        if self.is_used(d) {
            return None;
        }

        let mut new_pd = *self;
        new_pd.used[d as usize] = true;
        new_pd.num[new_pd.len] = d;
        new_pd.len += 1;
        Some(new_pd)
    }

    fn join_all<T: Iterator<Item = u32>>(&self, ds: T) -> Option<Pandigimal> {
        let mut pd = *self;
        for d in ds {
            match pd.join(d) {
                None => return None,
                Some(x) => pd = x,
            }
        }
        Some(pd)
    }
}

fn create_pandigimal_list(base: u32, len: usize) -> Vec<Pandigimal> {
    assert!(len > 0);
    let max = iter::repeat(RADIX).take(len).product::<u32>() - 1;
    (0..(max / base + 1))
        .filter_map(|n| Pandigimal::from_u64(base * n, len))
        .collect()
}

fn update_pandigimal_list(list: Vec<Pandigimal>, base: u64, len: usize) -> Vec<Pandigimal> {
    assert!(len > 0);
    let ord = iter::repeat(RADIX as u64).take(len - 1).product::<u64>();

    let mut result = vec![];
    for pd in &list {
        let num = pd.num();
        let ds = &num[num.len() - (len - 1)..];
        let lower = Integer::from_digits(ds.iter().map(|&x| x as u64), RADIX as u64);
        let it = (0..RADIX)
            .filter(|&d| !pd.is_used(d))
            .filter(|&d| ((d as u64) * ord + lower) % base == 0)
            .map(|d| pd.join(d).unwrap());
        result.extend(it);
    }
    result
}

fn solve() -> String {
    let mut result = create_pandigimal_list(17, 3);
    for &base in &[13, 11, 7, 5, 3, 2, 1] {
        result = update_pandigimal_list(result, base, 3);
    }

    result.iter().map(|pd| pd.to_u64()).sum::<u64>().to_string()
}

common::problem!("16695334890", solve);

#[cfg(test)]
mod tests {

    mod pandigimal {
        use super::super::Pandigimal;

        #[test]
        fn from_u64() {
            let pd = Pandigimal::from_u64(123, 3).unwrap();
            assert_eq!(false, pd.is_used(0));
            assert_eq!(true, pd.is_used(1));
            assert_eq!(true, pd.is_used(2));
            assert_eq!(true, pd.is_used(3));
            assert_eq!(false, pd.is_used(4));
            assert_eq!(&[3, 2, 1], &pd.num());

            let pd = Pandigimal::from_u64(123, 4).unwrap();
            assert_eq!(true, pd.is_used(0));
            assert_eq!(true, pd.is_used(1));
            assert_eq!(true, pd.is_used(2));
            assert_eq!(true, pd.is_used(3));
            assert_eq!(false, pd.is_used(4));
            assert_eq!(&[3, 2, 1, 0], &pd.num());

            let pd = Pandigimal::from_u64(123, 2).unwrap();
            assert_eq!(false, pd.is_used(0));
            assert_eq!(false, pd.is_used(1));
            assert_eq!(true, pd.is_used(2));
            assert_eq!(true, pd.is_used(3));
            assert_eq!(false, pd.is_used(4));
            assert_eq!(&[3, 2], &pd.num());

            assert!(Pandigimal::from_u64(11, 2).is_none());
        }

        #[test]
        fn join() {
            let pd = Pandigimal::from_u64(123, 3).unwrap();

            let pd2 = pd.join(5).unwrap();
            assert_eq!(false, pd2.is_used(0));
            assert_eq!(true, pd2.is_used(1));
            assert_eq!(true, pd2.is_used(2));
            assert_eq!(true, pd2.is_used(3));
            assert_eq!(false, pd2.is_used(4));
            assert_eq!(true, pd2.is_used(5));
            assert_eq!(&[3, 2, 1, 5], &pd2.num());

            assert!(pd.join(1).is_none());
        }
    }

    #[test]
    fn create_pandigimal_list() {
        assert_eq!(0, super::create_pandigimal_list(11, 2).len());

        let pds = super::create_pandigimal_list(11, 3);
        assert_eq!(&[2, 3, 1], &pds[0].num());
        assert_eq!(&[3, 4, 1], &pds[1].num());
        assert_eq!(&[4, 5, 1], &pds[2].num());
        assert_eq!(&[5, 6, 1], &pds[3].num());
        assert_eq!(&[6, 7, 1], &pds[4].num());
        assert_eq!(&[7, 8, 1], &pds[5].num());
        assert_eq!(&[8, 9, 1], &pds[6].num());
        assert_eq!(&[9, 0, 2], &pds[7].num());
        assert_eq!(&[1, 3, 2], &pds[8].num());

        let pds = super::create_pandigimal_list(9, 2);
        assert_eq!(10, pds.len());
        assert_eq!(&[9, 0], &pds[0].num());
        assert_eq!(&[8, 1], &pds[1].num());
        assert_eq!(&[7, 2], &pds[2].num());
        assert_eq!(&[6, 3], &pds[3].num());
        assert_eq!(&[5, 4], &pds[4].num());
        assert_eq!(&[4, 5], &pds[5].num());
        assert_eq!(&[3, 6], &pds[6].num());
        assert_eq!(&[2, 7], &pds[7].num());
        assert_eq!(&[1, 8], &pds[8].num());
        assert_eq!(&[0, 9], &pds[9].num());
    }

    #[test]
    fn update_pandigimal_list() {
        let pd = super::create_pandigimal_list(9, 2);
        let pd = super::update_pandigimal_list(pd, 5, 2);
        assert_eq!(16, pd.len());
        assert_eq!(&[9, 0, 1], &pd[0].num());
        assert_eq!(&[9, 0, 2], &pd[1].num());
        // snip
        assert_eq!(&[9, 0, 8], &pd[7].num());
        assert_eq!(&[4, 5, 0], &pd[8].num());
        assert_eq!(&[4, 5, 1], &pd[9].num());
        assert_eq!(&[4, 5, 2], &pd[10].num());
        assert_eq!(&[4, 5, 3], &pd[11].num());
        assert_eq!(&[4, 5, 6], &pd[12].num());
        assert_eq!(&[4, 5, 7], &pd[13].num());
        assert_eq!(&[4, 5, 8], &pd[14].num());
        assert_eq!(&[4, 5, 9], &pd[15].num());
    }
}
