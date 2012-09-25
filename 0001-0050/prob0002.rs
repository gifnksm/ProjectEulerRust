extern mod euler;

use euler::calc;

fn main() {
    let MAX = 4000000;
    let mut sum = 0;
    for calc::each_fib |f| {
        if f >= MAX     { break; }
        if f % 2 == 0 { sum += f; }
    }
    io::println(sum.to_str());
}
