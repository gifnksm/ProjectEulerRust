#![crate_id = "prob0043"]
#![crate_type = "rlib"]

extern crate math;

use std::fmt;
use std::iter::{AdditiveIterator, MultiplicativeIterator, Repeat};
use math::numconv;

pub static EXPECTED_ANSWER: &'static str = "16695334890";

#[deriving(TotalEq)]
struct DigitMap { used: [ bool, ..10] }

impl Eq for DigitMap {
    fn eq(&self, other: &DigitMap) -> bool { self.used == other.used }
}

impl fmt::Show for DigitMap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.used.as_slice().fmt(f)
    }
}

impl DigitMap {
    #[inline]
    fn new() -> DigitMap { DigitMap { used: [ false, ..10 ] } }

    #[inline]
    fn is_used(&self, n: uint) -> bool {
        assert!(n < 10);
        self.used[n]
    }

    #[inline]
    fn join(&self, d: uint) -> Option<DigitMap> {
        assert!(d < 10);

        if self.is_used(d) { return None }

        let mut new_dm = *self;
        new_dm.used[d] = true;
        Some(new_dm)
    }

    #[inline]
    fn join_all<T: Iterator<uint>>(&self, mut ds: T) -> Option<DigitMap> {
        let mut dm = *self;
        for d in ds {
            match dm.join(d) {
                None    => return None,
                Some(x) => dm = x
            }
        }
        Some(dm)
    }
}

fn filled_digits(n: uint, radix: uint, len: uint) -> ~[uint] {
    numconv::to_digits(n, radix).chain(Repeat::new(0u)).take(len).collect()
}

fn create_dm_list(base: uint, radix: uint, len: uint) -> Vec<(~[uint], DigitMap)> {
    assert!(len > 0);
    let max = Repeat::new(radix).take(len).product();
    range(0, max / base)
        .filter_map(|n| {
            let ds = filled_digits(n * base, radix, len);
            match DigitMap::new().join_all(ds.iter().map(|&x| x)) {
                Some(dm) => Some((ds, dm)),
                None     => None
            }
        }).collect()
}

fn update_dm_list(list: &[(~[uint], DigitMap)], base: uint, radix: uint, len: uint)
                  -> Vec<(~[uint], DigitMap)>
{
    assert!(len > 0);
    let ord = Repeat::new(radix).take(len - 1).product();

    let mut result = Vec::new();
    for &(ref ds, ref dm) in list.iter() {
        let lower = numconv::from_digits(ds.slice(ds.len() - (len - 1), ds.len()), radix);
        let it = range(0, radix)
            .filter(|d| (d * ord + lower) % base == 0)
            .filter_map(|d| dm.join(d).map(|dm| (*ds + &[d], dm)));
        result.extend(it);
    }
    result
}

pub fn solve() -> ~str {
    let mut result = create_dm_list(17, 10, 3);
    let base_list = [13u, 11, 7, 5, 3, 2, 1];
    for &base in base_list.iter() {
        result = update_dm_list(result.as_slice(), base, 10, 3);
    }

    result.move_iter()
        .map(|(r, _e)| numconv::from_digits(r, 10))
        .sum()
        .to_str()
}

#[cfg(test)]
mod tests {
    mod digit_map {
        use super::super::DigitMap;

        #[test]
        fn join_with() {
            let dm = DigitMap::new().join(1).unwrap();

            assert_eq!(false, dm.is_used(0));
            assert_eq!(true, dm.is_used(1));
            assert_eq!(false, dm.is_used(2));
            assert_eq!(false, dm.is_used(4));

            assert_eq!(None, dm.join(1));

            let dm = dm.join(4).unwrap();
            assert_eq!(false, dm.is_used(0));
            assert_eq!(true, dm.is_used(1));
            assert_eq!(true, dm.is_used(4));
            assert_eq!(false, dm.is_used(7));
        }

        #[test]
        fn join_all() {
            let dm = DigitMap::new().join_all((~[1u, 2, 3]).move_iter()).unwrap();

            assert_eq!(false, dm.is_used(0));
            assert_eq!(true, dm.is_used(1));
            assert_eq!(true, dm.is_used(2));
            assert_eq!(false, dm.is_used(4));

            assert_eq!(None, DigitMap::new().join_all((~[0u, 0]).move_iter()));
        }

    }

    #[test]
    fn filled_digits() {
        assert_eq!(~[], super::filled_digits(123, 10, 0));
        assert_eq!(~[3], super::filled_digits(123, 10, 1));
        assert_eq!(~[3, 2], super::filled_digits(123, 10, 2));
        assert_eq!(~[3, 2, 1], super::filled_digits(123, 10, 3));
        assert_eq!(~[3, 2, 1, 0], super::filled_digits(123, 10, 4));
    }

    #[test]
    fn create_dm_list() {
        let dm = super::create_dm_list(9, 10, 2);
        assert_eq!(10, dm.len());
        assert_eq!(~[9, 0], *dm.get(0).ref0());
        assert_eq!(~[8, 1], *dm.get(1).ref0());
        assert_eq!(~[7, 2], *dm.get(2).ref0());
        assert_eq!(~[6, 3], *dm.get(3).ref0());
        assert_eq!(~[5, 4], *dm.get(4).ref0());
        assert_eq!(~[4, 5], *dm.get(5).ref0());
        assert_eq!(~[3, 6], *dm.get(6).ref0());
        assert_eq!(~[2, 7], *dm.get(7).ref0());
        assert_eq!(~[1, 8], *dm.get(8).ref0());
        assert_eq!(~[0, 9], *dm.get(9).ref0());

        assert_eq!(0, super::create_dm_list(11, 10, 2).len());
    }

    #[test]
    fn update_dm_list() {
        let dm = super::update_dm_list(super::create_dm_list(9, 10, 2).as_slice(), 5, 10, 2);
        assert_eq!(16, dm.len());
        assert_eq!(~[9, 0, 1], *dm.get(0).ref0());
        assert_eq!(~[9, 0, 2], *dm.get(1).ref0());
        // snip
        assert_eq!(~[9, 0, 8], *dm.get(7).ref0());
        assert_eq!(~[4, 5, 0], *dm.get(8).ref0());
        assert_eq!(~[4, 5, 1], *dm.get(9).ref0());
        assert_eq!(~[4, 5, 2], *dm.get(10).ref0());
        assert_eq!(~[4, 5, 3], *dm.get(11).ref0());
        assert_eq!(~[4, 5, 6], *dm.get(12).ref0());
        assert_eq!(~[4, 5, 7], *dm.get(13).ref0());
        assert_eq!(~[4, 5, 8], *dm.get(14).ref0());
        assert_eq!(~[4, 5, 9], *dm.get(15).ref0());
    }
}
