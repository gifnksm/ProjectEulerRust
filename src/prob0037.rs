#![crate_name = "prob0037"]
#![crate_type = "rlib"]

extern crate math;

use std::iter::AdditiveIterator;
use math::numconv;
use math::prime::Prime;

pub static EXPECTED_ANSWER: &'static str = "748317";

fn is_r2l(prime: &Prime, n: uint) -> bool {
    let mut itr = n / 10;
    while itr > 0 {
        if !prime.contains(itr) { return false }
        itr /= 10;
    }
    true
}

pub fn solve() -> String {
    let prime = Prime::new();
    let mut l2r_mat = vec!(vec!(2u, 3, 5, 7));
    let mut order = 10;

    loop {
        let mut result = Vec::new();
        for &p in l2r_mat.last().unwrap().iter() {
            // 2 can only be appeared as the most left digits
            if numconv::to_digits(p, 10).next_back() == Some(2) { continue }

            let ds = [ 1u, 2, 3, 5, 7, 9 ];
            for &d in ds.iter() {
                let n = order * d + p;
                if prime.contains(n) { result.push(n); }
            }
        }

        if result.is_empty() { break }
        l2r_mat.push(result);
        order *= 10;
    }

    l2r_mat
        .move_iter()
        .flat_map(|l2r| l2r.move_iter())
        .filter(|&n| n>= 10)
        .filter(|&n| is_r2l(&prime, n))
        .sum()
        .to_str()
}
