#![crate_id = "prob0036"]
#![crate_type = "rlib"]

extern crate math;

use math::numconv;

pub static EXPECTED_ANSWER: &'static str = "872187";

pub fn solve() -> StrBuf {
    let order_array = &[ 1u, 10, 100, 1000, 1000, 10000 ];
    let mut sum = 0;
    for i in range(0, order_array.len() - 1) {
        let tf = [true, false];
        for &b in tf.iter() {
            let (start, end) = (order_array[i], order_array[i + 1]);
            for n in range(start, end) {
                let n = numconv::to_palindromic(n, 10, b);
                if n >= 1000000 { break; }
                if numconv::is_palindromic(n, 2) {
                    sum += n;
                }
            }
        }
    }

    return sum.to_str();
}

