use euler::prime::{ Prime, factors };

fn num_factors(n: uint, ps: &mut Prime) -> uint {
    let mut cnt = 0;
    for factors(n, ps) |_f| { cnt += 1; }
    return cnt;
}

pub fn solve() -> uint {
    let mut ps = Prime();
    let mut cnt = 0;
    let len = 4;
    let num_factor = 4;
    let mut n = 1;
    loop {
        if num_factors(n, &mut ps) == num_factor {
            cnt += 1;
        } else {
            cnt = 0;
        }
        if cnt == len {
            return n + 1 - len;
        }
        n += 1;
    }
}
