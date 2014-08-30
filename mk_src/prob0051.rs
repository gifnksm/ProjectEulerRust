#![crate_name = "prob0051"]
#![crate_type = "rlib"]

extern crate math;

use math::numconv;
use math::prime::Prime;

pub static EXPECTED_ANSWER: &'static str = "121313";

pub fn solve() -> String {
    let num_family = 8;
    let prime = Prime::new();

    for p in prime.iter() {
        let ds = numconv::to_digits(p, 10);
        let hs = numconv::to_digit_histogram(p);

        for (d_src, &cnt) in hs.iter().enumerate() {
            // 同じ文字が2つ以上登場する数値だけを対象にする
            if cnt <= 1 { continue }

            // d_src が 2 以上の場合、d >= d_src の数が 8 個以上にならないため
            // (d_src を d_dst で置き換えた場合に、8種類の数字が作れない)
            if 9 - d_src < num_family { continue }

            let mut cnt = 1;
            for d_dst in range(d_src + 1, 10) {
                let buf = ds.map(|d| if d == d_src { d_dst } else { d }).collect::<Vec<uint>>();
                if prime.contains(numconv::from_digits(buf.as_slice(), 10)) {
                    cnt += 1;
                }
            }

            if cnt >= num_family {
                return p.to_string();
            }
        }
    }

    unreachable!();
}

