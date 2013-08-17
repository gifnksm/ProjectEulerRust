#[link(name = "prob0052", vers = "0.0")];
#[crate_type = "lib"];

extern mod math;

use math::numconv;

pub static EXPECTED_ANSWER: &'static str = "142857";

pub fn solve() -> ~str {
    let mut n = 0;
    let mut order = 0;
    let mut limit = 0;

    loop {
        n += 1;
        if n > limit {
            // 6倍しても桁が変わらない数だけ調べる
            n = order;
            order *= 10;
            limit = (order - 1) / 6;
            loop;
        }

        let ds = numconv::to_digit_histogram(n);

        // n * 5 の時に、必ず 0 または 5 は含むため
        if ds[0] == 0 && ds[5] == 0 { loop; }

        // n * 2, n * 4 の時に、必ず偶数は含むため
        if ds[0] == 0 && ds[2] == 0 && ds[4] == 0 && ds[6] == 0 && ds[8] == 0 {
            loop;
        }

        if ds != numconv::to_digit_histogram(n * 2) { loop; }
        if ds != numconv::to_digit_histogram(n * 3) { loop; }
        if ds != numconv::to_digit_histogram(n * 4) { loop; }
        if ds != numconv::to_digit_histogram(n * 5) { loop; }
        if ds != numconv::to_digit_histogram(n * 6) { loop; }

        return n.to_str();
    }
}

