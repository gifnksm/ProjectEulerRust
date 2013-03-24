use core::util::{ unreachable };
use euler::prime::{ Prime };

pub fn solve() -> uint {
    let mut num = 600851475143;
    let mut ps = Prime();
    for ps.each |p| {
        while num % p == 0 {
            num /= p;
        }
        if num == 1 {
            return p;
        }
    }

    unreachable();
}
