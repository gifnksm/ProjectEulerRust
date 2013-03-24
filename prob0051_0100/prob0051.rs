use core::util::{ unreachable };

use common::prime::{ Prime };
use common::calc::{ num_to_digits, digits_to_num, digit_histogram };
use common::problem::{ Problem };

pub static problem: Problem<'static> = Problem {
    id: 51,
    answer: "121313",
    solver: solve
};

fn solve() -> ~str {
    let num_family = 8;

    let mut ps = Prime::new();
    for ps.each_borrow |p, ps| {
        let ds = num_to_digits(p, 10);
        let hs = digit_histogram(p);
        for hs.eachi |i, &cnt| {
            // 同じ文字が2つ以上登場する数値だけを対象にする
            if cnt <= 1 { loop; }

            // i が 2 以上の場合、d >= i の数が 8 個以上にならないため
            // (i を d で置き換えた場合に、8種類の数字が作れない)
            if 9 - i < num_family { loop; }

            let mut cnt = 1;
            for uint::range(i + 1, 10) |j| {
                let buf = ds.map(|&d| if d == i { j } else { d });
                if ps.is_prime(digits_to_num(buf, 10)) {
                    cnt += 1;
                }
            }
            if cnt >= num_family {
                return p.to_str();
            }
        }
    }

    unreachable();
}

