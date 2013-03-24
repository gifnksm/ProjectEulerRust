use std::sort::{ merge_sort };

use common::prime::{ Prime };
use common::calc::{ num_to_digits, permutate_num };

pub fn solve() -> uint {
    let d = 3330;

    let mut ps = Prime();
    let mut i = 0;
    loop {
        let p1 = ps.get_at(i);
        if p1 < 1000 { i += 1; loop; }
        if p1 > 9999 - 2 * d { fail!(); }
        if p1 == 1487 { i += 1; loop; }

        let p2 = p1 + d;
        let p3 = p2 + d;
        let sorted = merge_sort(num_to_digits(p1, 10), |a, b| a <= b);
        if merge_sort(num_to_digits(p2, 10), |a, b| a <= b) != sorted {
            i += 1;
            loop;
        }
        if merge_sort(num_to_digits(p3, 10), |a, b| a <= b) != sorted {
            i += 1;
            loop;
        }

        if !ps.is_prime(p2) { i += 1; loop; }
        if !ps.is_prime(p3) { i += 1; loop; }
        return uint::from_str(fmt!("%u%u%u", p1, p2, p3)).get();
    }
}
