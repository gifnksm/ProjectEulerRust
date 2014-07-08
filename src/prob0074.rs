#![crate_name = "prob0074"]
#![crate_type = "rlib"]


use std::collections::HashMap;

pub static EXPECTED_ANSWER: &'static str = "402";

#[deriving(Clone)]
enum Length {
    Loop(uint), Chain(uint), Unknown
}

#[inline(always)]
fn fact_sum(mut n: uint, fs: &[uint, ..10]) -> uint {
    if n == 0 { return 1; }

    let mut sum = 0;
    while n > 0 {
        sum += fs[n % 10];
        n /= 10;
    }
    return sum;
}

#[inline(always)]
fn get_chain_len(
    n: uint, map: &mut[Length], fs: &[uint, ..10]
) -> uint {
    let mut chain_map = HashMap::<uint, uint>::new();
    let mut idx = n;
    let mut chain_len = 0;
    let mut loop_len  = 0;

    loop {
        match map[idx] {
            Loop(c)  => { loop_len += c;  break; }
            Chain(c) => { chain_len += c; break; }
            Unknown  => {
                match chain_map.find(&idx) {
                    Some(&chain_idx) => {
                        loop_len  = chain_len - chain_idx;
                        chain_len = chain_idx;
                        break;
                    }
                    None => {
                        chain_map.insert(idx, chain_len);
                        idx = fact_sum(idx, fs);
                        chain_len += 1;
                    }
                }
            }
        }
    }

    for (&key, &idx) in chain_map.iter() {
        if idx >= chain_len {
            map[key] = Loop(loop_len);
        } else {
            map[key] = Chain(loop_len + chain_len - idx);
        }
    }

    return chain_len + loop_len;
}

pub fn solve() -> String {
    let limit = 1000000;
    let factorial = {
        let mut val = [1, ..10];
        for i in range(1u, 10) {
            val[i] = val[i - 1] * i;
        }
        val
    };

    let mut map = Vec::from_elem(factorial[9] * 6 + 1, Unknown);
    let mut cnt = 0u;
    for n in range(1u, limit + 1) {
        let len = get_chain_len(n, map.as_mut_slice(), &factorial);
        if len == 60 { cnt += 1; }
    }
    return cnt.to_str();
}
