fn get_cycle_len(n: uint) -> uint {
    if n == 1 { return 1; }
    let mut buf = vec::to_mut(vec::from_elem(n, None));
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

fn main() {
    let mut longest = { num: 0, len: 0};
    for uint::range(2, 1000) |n| {
        let len = get_cycle_len(n);
        if longest.len < len {
            longest.num = n;
            longest.len = len;
        }
    }
    io::println(fmt!("%u => %u", longest.num, longest.len));
}