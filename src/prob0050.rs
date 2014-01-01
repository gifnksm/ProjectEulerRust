#[crate_type = "rlib"];

extern mod math;

use math::prime::Prime;

pub static EXPECTED_ANSWER: &'static str = "997651";

fn get_longer(prime: &Prime, p: uint, min_len: uint) -> Option<uint> {
    let max_avg = if min_len == 0 { p } else { p / min_len };

    let mut start_idx = 0;
    let mut end_idx   = 0;
    let mut start     = prime.nth(0);
    let mut sum       = prime.nth(0);
    loop {
        let len = (end_idx - start_idx + 1) as uint;
        if sum / len > max_avg { return None; }
        if sum == p {
            if len <= min_len {
                return None;
            } else {
                return Some(len);
            }
        }

        if sum < p {
            end_idx += 1;
            if end_idx >= 0 { sum += prime.nth(end_idx as uint); }
            continue
        }

        if sum > p {
            sum -= start;
            start_idx += 1;
            if start_idx < 0 {
                start = 0;
            } else {
                start = prime.nth(start_idx as uint)
            }
            continue
        }
    }
}

pub fn solve() -> ~str {
    let limit = 1000000;

    let prime = Prime::new();
    let mut it = prime.iter().take_while(|&p| p <= limit);

    let mut len = 0;
    let mut num = 0;
    for p in it {
        match get_longer(&prime, p, len) {
            Some(l) => {
                len = l;
                num = p;
            }
            None => {}
        }
    }
    return num.to_str();
}
