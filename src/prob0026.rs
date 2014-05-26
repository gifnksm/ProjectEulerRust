#![crate_id = "prob0026"]
#![crate_type = "rlib"]

pub static EXPECTED_ANSWER: &'static str = "983";

fn get_cycle_len(n: uint) -> uint {
    if n == 1 { return 1 }
    let mut buf = Vec::from_elem(n, None);
    let mut rem = 1;
    let mut idx = 1;
    loop {
        let new_rem = rem % n;
        match *buf.get(new_rem) {
            Some(i) => { return idx - i }
            None    => { *buf.get_mut(new_rem) = Some(idx); }
        }
        idx += 1;
        rem = new_rem * 10;
    }
}

pub fn solve() -> String {
    return range(2u, 1000)
        .max_by(|&n| get_cycle_len(n))
        .unwrap()
        .to_str();
}
