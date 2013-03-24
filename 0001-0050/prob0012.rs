use core::util::{ unreachable };

use common::prime::{ Prime, num_of_divisors };
use common::calc::{ each_triangles };

pub fn solve() -> uint {
    let mut primes = Prime();
    for each_triangles |t| {
        let num = num_of_divisors(t, &mut primes);
        if num > 500 {
            return t;
        }
    }

    unreachable();
}
