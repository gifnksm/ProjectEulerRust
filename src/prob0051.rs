#[link(name = "prob0051", vers = "0.0")];
#[crate_type = "lib"];

extern mod common;

use std::util;
use common::prime;
use common::calc;

pub static EXPECTED_ANSWER: &'static str = "121313";

pub fn solve() -> ~str {
    let num_family = 8;

    for p in prime::iter() {
        let ds = calc::num_to_digits(p, 10);
        let hs = calc::digit_histogram(p);
        for (i, &cnt) in hs.iter().enumerate() {
            // 同じ文字が2つ以上登場する数値だけを対象にする
            if cnt <= 1 { loop; }

            // i が 2 以上の場合、d >= i の数が 8 個以上にならないため
            // (i を d で置き換えた場合に、8種類の数字が作れない)
            if 9 - i < num_family { loop; }

            let mut cnt = 1;
            for j in range(i + 1, 10) {
                let buf = ds.map(|&d| if d == i { j } else { d });
                if prime::contains(calc::digits_to_num(buf, 10)) {
                    cnt += 1;
                }
            }
            if cnt >= num_family {
                return p.to_str();
            }
        }
    }

    util::unreachable();
}

