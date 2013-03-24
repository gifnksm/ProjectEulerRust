use common::calc::{ each_fib };

pub fn solve() -> uint {
    let max = 4000000;
    let mut sum = 0;
    for each_fib |f: &uint| {
        if *f >= max { break; }
        if *f % 2 == 0 { sum += *f; }
    }
    return sum;
}
