#[crate_id = "prob0043"];
#[crate_type = "rlib"];

extern crate math;

use std::slice;
use std::iter::AdditiveIterator;
use math::numconv;

pub static EXPECTED_ANSWER: &'static str = "16695334890";

struct DigitMap { used: [ bool, ..10] }

fn DigitMap() -> DigitMap {
    DigitMap { used: [ false, ..10 ] }
}

impl DigitMap {
    fn is_used(&self, n: uint) -> bool {
        assert!(n < 10);
        self.used[n]
    }

    fn get_used(&self, ds: &[uint]) -> Option<DigitMap> {
        let mut used: [bool, ..10] = [false, ..10];
        for &d in ds.iter() {
            assert!(d < 10);
            if used[d] || self.is_used(d) { return None; }
            used[d] = true;
        }

        Some(DigitMap { used: [
            self.used[0] || used[0],
            self.used[1] || used[1],
            self.used[2] || used[2],
            self.used[3] || used[3],
            self.used[4] || used[4],
            self.used[5] || used[5],
            self.used[6] || used[6],
            self.used[7] || used[7],
            self.used[8] || used[8],
            self.used[9] || used[9]
        ] })
    }
}

fn fill_vec<T: Clone>(v: ~[T], len: uint, init: T) -> ~[T] {
    assert!(v.len() <= len);
    if v.len() == len { return v; }
    v + slice::from_elem(len - v.len(), init)
}

pub fn solve() -> ~str {
    let mut result = slice::build(None, |push| {
        let dm   = DigitMap();
        let base = 17;
        for n in range(0u, 1000 / base) {
            let new_ds = fill_vec(numconv::to_digits(n * base, 10).collect::<~[uint]>(), 3, 0);
            match dm.get_used(new_ds) {
                None         => continue,
                Some(new_dm) => push((new_ds, new_dm))
            }
        }
    });

    let base_list = [13u, 11, 7, 5, 3, 2, 1];
    for &base in base_list.iter() {
        result = result.flat_map(|&(ref ds, ref dm)| {
            slice::build(None, |push| {
                let lower = numconv::from_digits(ds.slice(ds.len() - 2, ds.len()), 10);
                for d in range(0u, 10) {
                    if (d * 100 + lower) % base != 0 { continue }
                    match dm.get_used([d]) {
                        None         => continue,
                        Some(new_dm) => push((*ds + &[d], new_dm))
                    }
                }
            })
        });
    }

    result.move_iter()
        .map(|(r, _e)| numconv::from_digits(r, 10))
        .sum()
        .to_str()
}
