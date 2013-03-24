use common::problem::{ Problem };

pub static problem: Problem<'static> = Problem {
    id: 26,
    answer: "983",
    solver: solve
};

fn get_cycle_len(n: uint) -> uint {
    if n == 1 { return 1; }
    let mut buf = vec::from_elem(n, None);
    let mut rem = 1;
    let mut idx = 1;
    loop {
        let new_rem = rem % n;
        match buf[new_rem] {
            Some(i) => { return idx - i; }
            None    => { buf[new_rem] = Some(idx); }
        }
        idx += 1;
        rem = new_rem * 10;
    }
}

fn solve() -> ~str {
    let mut longest_num = 0;
    let mut longest_len = 0;
    for uint::range(2, 1000) |n| {
        let len = get_cycle_len(n);
        if longest_len < len {
            longest_num = n;
            longest_len = len;
        }
    }
    return longest_num.to_str();
}
