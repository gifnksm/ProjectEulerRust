use core::util::{ unreachable };

use std::sort::{ merge_sort };

use common::prime::{ Prime };
use common::calc::{ num_to_digits, permutate_num };
use common::problem::{ Problem };

pub static problem: Problem<'static> = Problem {
    id: 49,
    answer: "296962999629",
    solver: solve
};

fn solve() -> ~str {
    let d = 3330;

    let mut ps = Prime::new();
    for ps.each_borrow |p1, ps| {
        if p1 < 1000 { loop; }
        if p1 > 9999 - 2 * d { fail!(); }
        if p1 == 1487 { loop; }

        let p2 = p1 + d;
        let p3 = p2 + d;
        let sorted = merge_sort(num_to_digits(p1, 10), |a, b| a <= b);
        if merge_sort(num_to_digits(p2, 10), |a, b| a <= b) != sorted {
            loop;
        }
        if merge_sort(num_to_digits(p3, 10), |a, b| a <= b) != sorted {
            loop;
        }

        if !ps.is_prime(p2) { loop; }
        if !ps.is_prime(p3) { loop; }
        return fmt!("%u%u%u", p1, p2, p3);
    }

    unreachable();
}
