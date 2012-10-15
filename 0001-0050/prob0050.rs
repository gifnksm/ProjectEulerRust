extern mod euler;
use euler::prime::{ Prime };

fn get_longer(p: uint, min_len: uint, ps: &Prime) -> Option<uint> {
    fn get_prime(n: int, ps: &Prime)-> uint {
        if n < 0 { 0 } else { ps.get_at(n as uint) }
    }

    let max_avg = if min_len == 0 { p } else { p / min_len };

    let mut start_idx = 0;
    let mut end_idx   = 0;
    let mut start     = ps.get_at(0);
    let mut sum       = ps.get_at(0);
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
            sum += get_prime(end_idx, ps);
            loop;
        }

        if sum > p {
            sum -= start;
            start_idx += 1;
            start = get_prime(start_idx, ps);
            loop;
        }
    }
}

fn main() {
    let limit = 1000000;
    let ps = Prime();

    let mut len = 0;
    let mut num = 0;
    for ps.each |p| {
        if p > limit { break; }
        match get_longer(p, len, &ps) {
            Some(l) => {
                io::println(fmt!("%u => %u", p, l));
                len = l;
                num = p;
            }
            None => {}
        }
    }
    io::println(fmt!("%u: %u", num, len));
    io::println(fmt!("answer: %u", num));
}
