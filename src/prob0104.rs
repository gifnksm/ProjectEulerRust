#[crate_id = "prob0104"];
#[crate_type = "rlib"];

extern crate num;

use std::iter::Unfold;
use num::Integer;

pub static EXPECTED_ANSWER: &'static str = "329468";

fn is_pandigit(n: u64) -> bool {
    let mut hist = [false, .. 10];
    let mut cnt = 0;
    let mut itr = n;
    while itr > 0 {
        let (d, r) = itr.div_rem(&10);
        if r == 0 || hist[r] { return false; }
        hist[r] = true;
        itr = d;
        cnt += 1;
    }
    return cnt == 9;
}

pub fn solve() -> ~str {
    let base = from_str::<u64>("1" + "0".repeat(9)).unwrap();

    let phi = (1.0 + (5.0f64).sqrt()) / 2.0;
    let next_fib_first10 = |st: &mut (u64, uint)| {
        let (n, cnt) = (st.val0(), st.val1());
        let next = match cnt {
            0 => 1,
            1 => 1,
            2 => 2,
            3 => 3,
            4 => 5,
            _ => {
                let mut f = ((n as f64) * phi + 0.5) as u64;
                while f > base * base { f /= 10; }
                f
            }
        };
        *st = (next, cnt + 1);
        let mut curr = n;
        while curr > base { curr /= 10; }
        Some(curr)
    };

    let next_fib_last10 = |st: &mut (u64, u64)| {
        let (n0, n1) = *st;
        let next = (n0 + n1) % base;
        *st = (n1, next);
        Some(n0)
    };

    let first = Unfold::new((0, 0), next_fib_first10);
    let last  = Unfold::new((0, 1), next_fib_last10);
    let mut it = first.zip(last).enumerate().filter(|&(_, (f, l))| is_pandigit(f) && is_pandigit(l));
    let (k, _) = it.next().unwrap();
    return k.to_str();
}
