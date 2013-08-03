#[link(name = "prob0037", vers = "0.0")];
#[crate_type = "lib"];

extern mod common;

use std::vec;
use common::calc::{num_to_digits};
use common::prime;

pub static EXPECTED_ANSWER: &'static str = "748317";

fn is_r2l(n: uint) -> bool {
    let mut itr = n / 10;
    while itr > 0 {
        if !prime::contains(itr) { return false; }
        itr /= 10;
    }
    return true;
}

pub fn solve() -> ~str {
    let mut l2r_mat = ~[ ~[ 2u, 3, 5, 7 ] ];
    let mut order = 10;

    loop {
        let mut result = ~[];
        foreach &p in l2r_mat.last().iter() {
            // 2 can obly be appeared as the mnost left digits
            if num_to_digits(p, 10)[0] == 2 { loop; }

            let ds = [ 1u, 2, 3, 5, 7, 9 ];
            foreach &d in ds.iter() {
                let n = order * d + p;
                if prime::contains(n) { result.push(n); }
            }
        }

        if result.is_empty() { break; }
        l2r_mat.push(result);
        order *= 10;
    }

    let l2r = vec::concat(l2r_mat);
    let mut sum = 0;
    foreach n in  l2r.iter() {
        if *n < 10 { loop; }
        if is_r2l(*n) { sum += *n; }
    }

    return sum.to_str();
}
