use core::util::{ unreachable };

use std::sort::{ quick_sort };

use common::calc::{ permutate_num, num_to_digits };

pub fn solve() -> ~str {
    for permutate_num(~[9, 8, 7, 6, 5, 4, 3, 2, 1], 4, 0, 9999) |num, rest| {
        let mut ds = num_to_digits(num * 2, 10);
        quick_sort(ds, |a, b| a >= b);

        if vec::eq(ds, rest) {
            return fmt!("%u%u", num, num * 2);
        }
    }

    unreachable();
}
