#[crate_type = "rlib"];

extern mod math;

use std::iter::MultiplicativeIterator;
use math::numconv;

pub static EXPECTED_ANSWER: &'static str = "210";

// d_(a*10 + b)
//    0 1 2 3 4 5 6 7 8 9
// 0  - 1 2 3 4 5 6 7 8 9
// 1  1 0 1 1 1 2 1 3 1 4
// 2  1 5 1 6 1 7 1 8 1 9
// 3  2 0 2 1 2 2 2 3 2 4
// 4  2 5 2 6 2 7 2 8 2 9
// 5  3 0 3 1 3 2 3 3 3 4
//
// [1,   9]   => 9
// [10,  99]  => 90
// [100, 999] => 900
//
// num => idx
// 1 <= n <= 9       => i = n
// 10 <= n <= 99     => i = 2 * (n - 10) + 10 = 2n - 10
// 100 <= n <= 999   => i = 3 * (n - 100) + 2 * 100 - 10 = 3n - 110
// 1000 <= n <= 9999 => i = 4 * (n - 1000) + 3 * 1000 - 110 = 4n - 1110

struct Area {
    num_digit: uint,
    min_val: uint,
    max_val: uint,
    min_idx: uint,
    max_idx: uint
}

impl Area {
    #[inline]
    pub fn new() -> Area {
        Area {
            num_digit: 0,
            min_val: 0, max_val: 0,
            min_idx: 0, max_idx: 0
        }
    }

    #[inline]
    pub fn contain_index(&self, idx: uint) -> bool {
        self.min_idx <= idx && idx <= self.max_idx
    }

    #[inline]
    pub fn next(&self) -> Area {
        let num_digit = self.num_digit + 1;
        let min_val   = self.max_val + 1;
        let min_idx   = self.max_idx + 1;
        Area {
            num_digit: num_digit,
            min_val: min_val,
            max_val: min_val * 10 - 1,
            min_idx: min_idx,
            max_idx: min_idx + min_val * 9 * num_digit - 1
        }
    }

    #[inline]
    pub fn get_nth_digit(&self, n: uint) -> uint {
        let (d, r) = (n - self.min_idx).div_rem(&self.num_digit);
        numconv::to_digits(self.min_val + d, 10).invert().nth(r).unwrap()
    }
}

struct AreaSeq<'a> {
    seq: &'a mut ~[Area],
    idx: uint
}

impl<'a> AreaSeq<'a> {
    #[inline]
    fn new(seq: &'a mut ~[Area]) -> AreaSeq<'a> {
        AreaSeq { seq: seq, idx: 0 }
    }
}

impl<'a> Iterator<Area> for AreaSeq<'a> {
    #[inline]
    fn next(&mut self) -> Option<Area> {
        self.idx += 1;
        self.seq.reserve(self.idx + 1);
        while self.idx >= self.seq.len() {
            let next = self.seq.last().next();
            self.seq.push(next)
        }
        Some(self.seq[self.idx])
    }
}

pub fn solve() -> ~str {
    let idx = &[ 1u, 10, 100, 1000, 10000, 100000, 1000000 ];

    let mut area_seq = ~[ Area::new() ];
    idx.iter()
        .map(|&n| {
            AreaSeq::new(&mut area_seq)
                .find(|area| area.contain_index(n))
                .unwrap()
                .get_nth_digit(n)
        }).product()
        .to_str()
}
