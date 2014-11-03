#![warn(unused, bad_style,
        unused_qualifications, unused_typecasts, unused_results)]

#![feature(slicing_syntax)]

extern crate common;
extern crate integer;

use std::iter::{mod, AdditiveIterator, MultiplicativeIterator, Repeat};
use common::Solver;
use integer::Integer;

const RADIX: uint = 10;

struct Pandigimal {
    used: [ bool, .. RADIX ],
    num:  [ uint, .. RADIX ],
    len:  uint
}

impl Pandigimal {
    fn new() -> Pandigimal {
        Pandigimal {
            used: [ false, .. RADIX ],
            num:  [ 0, .. RADIX ],
            len:  0
        }
    }

    fn from_uint(n: uint, len: uint) -> Option<Pandigimal> {
        let it = n.into_digits(RADIX).chain(Repeat::new(0));
        Pandigimal::new().join_all(it.take(len))
    }

    fn to_uint(&self) -> uint {
        Integer::from_digits(self.num().iter().map(|&x| x), RADIX)
    }

    fn is_used(&self, n: uint) -> bool {
        assert!(n < RADIX);
        self.used[n]
    }

    fn num<'a>(&'a self) -> &'a [uint] { self.num[.. self.len] }

    fn join(&self, d: uint) -> Option<Pandigimal> {
        assert!(d < RADIX);
        if self.is_used(d) { return None }

        let mut new_pd = *self;
        new_pd.used[d] = true;
        new_pd.num[new_pd.len] = d;
        new_pd.len += 1;
        Some(new_pd)
    }

    fn join_all<T: Iterator<uint>>(&self, mut ds: T) -> Option<Pandigimal> {
        let mut pd = *self;
        for d in ds {
            match pd.join(d) {
                None    => return None,
                Some(x) => pd = x
            }
        }
        Some(pd)
    }
}

fn create_pandigimal_list(base: uint, len: uint) -> Vec<Pandigimal> {
    assert!(len > 0);
    let max = Repeat::new(RADIX).take(len).product() - 1;
    iter::range_inclusive(0, max / base)
        .filter_map(|n| Pandigimal::from_uint(base * n, len))
        .collect()
}

fn update_pandigimal_list(list: Vec<Pandigimal>, base: uint, len: uint) -> Vec<Pandigimal> {
    assert!(len > 0);
    let ord = Repeat::new(RADIX).take(len - 1).product();

    let mut result = Vec::new();
    for pd in list.iter() {
        let num = pd.num();
        let ds = num[num.len() - (len - 1) ..];
        let lower = Integer::from_digits(ds.iter().map(|&x| x), RADIX);
        let it = range(0, RADIX)
            .filter(|&d| !pd.is_used(d))
            .filter(|&d| (d * ord + lower) % base == 0)
            .map(|d| pd.join(d).unwrap());
        result.extend(it);
    }
    result
}

fn solve() -> String {
    let mut result = create_pandigimal_list(17, 3);
    for &base in [13, 11, 7, 5, 3, 2, 1].iter() {
        result = update_pandigimal_list(result, base, 3);
    }

    result
        .iter()
        .map(|pd| pd.to_uint())
        .sum()
        .to_string()
}

fn main() { Solver::new("16695334890", solve).run(); }

#[cfg(test)]
mod tests {

    mod pandigimal {
        use super::super::Pandigimal;

        #[test]
        fn from_uint() {
            let pd = Pandigimal::from_uint(123, 3).unwrap();
            assert_eq!(false, pd.is_used(0));
            assert_eq!(true, pd.is_used(1));
            assert_eq!(true, pd.is_used(2));
            assert_eq!(true, pd.is_used(3));
            assert_eq!(false, pd.is_used(4));
            assert_eq!([3, 2, 1][], pd.num())

            let pd = Pandigimal::from_uint(123, 4).unwrap();
            assert_eq!(true, pd.is_used(0));
            assert_eq!(true, pd.is_used(1));
            assert_eq!(true, pd.is_used(2));
            assert_eq!(true, pd.is_used(3));
            assert_eq!(false, pd.is_used(4));
            assert_eq!([3, 2, 1, 0][], pd.num());

            let pd = Pandigimal::from_uint(123, 2).unwrap();
            assert_eq!(false, pd.is_used(0));
            assert_eq!(false, pd.is_used(1));
            assert_eq!(true, pd.is_used(2));
            assert_eq!(true, pd.is_used(3));
            assert_eq!(false, pd.is_used(4));
            assert_eq!([3, 2][], pd.num());

            assert!(Pandigimal::from_uint(11, 2).is_none());
        }

        #[test]
        fn join() {
            let pd = Pandigimal::from_uint(123, 3).unwrap();

            let pd2 = pd.join(5).unwrap();
            assert_eq!(false, pd2.is_used(0));
            assert_eq!(true, pd2.is_used(1));
            assert_eq!(true, pd2.is_used(2));
            assert_eq!(true, pd2.is_used(3));
            assert_eq!(false, pd2.is_used(4));
            assert_eq!(true, pd2.is_used(5));
            assert_eq!([3, 2, 1, 5][], pd2.num());

            assert!(pd.join(1).is_none());
        }
    }

    #[test]
    fn create_pandigimal_list() {
        assert_eq!(0, super::create_pandigimal_list(11, 2).len());

        let pds = super::create_pandigimal_list(11, 3);
        assert_eq!([2, 3, 1][], pds[0].num());
        assert_eq!([3, 4, 1][], pds[1].num());
        assert_eq!([4, 5, 1][], pds[2].num());
        assert_eq!([5, 6, 1][], pds[3].num());
        assert_eq!([6, 7, 1][], pds[4].num());
        assert_eq!([7, 8, 1][], pds[5].num());
        assert_eq!([8, 9, 1][], pds[6].num());
        assert_eq!([9, 0, 2][], pds[7].num());
        assert_eq!([1, 3, 2][], pds[8].num());

        let pds = super::create_pandigimal_list(9, 2);
        assert_eq!(10, pds.len());
        assert_eq!([9, 0][], pds[0].num());
        assert_eq!([8, 1][], pds[1].num());
        assert_eq!([7, 2][], pds[2].num());
        assert_eq!([6, 3][], pds[3].num());
        assert_eq!([5, 4][], pds[4].num());
        assert_eq!([4, 5][], pds[5].num());
        assert_eq!([3, 6][], pds[6].num());
        assert_eq!([2, 7][], pds[7].num());
        assert_eq!([1, 8][], pds[8].num());
        assert_eq!([0, 9][], pds[9].num());
    }

    #[test]
    fn update_pandigimal_list() {
        let pd = super::create_pandigimal_list(9, 2);
        let pd = super::update_pandigimal_list(pd, 5, 2);
        assert_eq!(16, pd.len());
        assert_eq!([9, 0, 1][], pd[0].num());
        assert_eq!([9, 0, 2][], pd[1].num());
        // snip
        assert_eq!([9, 0, 8][], pd[7].num());
        assert_eq!([4, 5, 0][], pd[8].num());
        assert_eq!([4, 5, 1][], pd[9].num());
        assert_eq!([4, 5, 2][], pd[10].num());
        assert_eq!([4, 5, 3][], pd[11].num());
        assert_eq!([4, 5, 6][], pd[12].num());
        assert_eq!([4, 5, 7][], pd[13].num());
        assert_eq!([4, 5, 8][], pd[14].num());
        assert_eq!([4, 5, 9][], pd[15].num());
    }
}
