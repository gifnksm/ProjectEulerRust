#![crate_id = "prob0095"]
#![crate_type = "rlib"]

use std::iter;

pub static EXPECTED_ANSWER: &'static str = "14316";

#[inline(always)]
fn get_chain_len(mut n: uint, len_map: &mut [Option<uint>], div_map: &[uint]) -> uint {
    match len_map[n] {
        Some(x) => { return x; }
        None => {}
    }

    let mut itr_map = vec!(n);
    loop {
        n = div_map[n];

        if n >= len_map.len() {
            for &n in itr_map.iter() { len_map[n] = Some(0); }
            return 0;
        }

        match itr_map.as_slice().position_elem(&n) {
            Some(idx) => {
                let len = itr_map.len() - idx;
                for (i, &m) in itr_map.iter().enumerate() {
                    len_map[m] = Some(if i < idx { 0 } else { len });
                }
                return if idx != 0 { 0 } else { len };
            }
            None => { itr_map.push(n); }
        }
    }
}

pub fn solve() -> StrBuf {
    let limit = 1000000;
    let mut len_map = Vec::from_elem(limit + 1, None);
    let mut div_map = Vec::from_elem(limit + 1, 1u);
    *div_map.get_mut(0) = 0;
    *div_map.get_mut(1) = 0;

    for f in range(2, limit / 2) {
        for n in iter::range_step(2 * f, limit, f) {
            *div_map.get_mut(n) += f;
        }
    }

    let (n, _div) = range(1, len_map.len())
        .map(|n| (n, get_chain_len(n, len_map.as_mut_slice(), div_map.as_mut_slice())))
        .max_by(|&(_n, div)| div)
        .unwrap();

    return n.to_str();
}
