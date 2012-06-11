use euler;

import euler::calc;

fn main() {
    const MAX: u64 = 4000000u64;
    let mut sum = 0u64;
    for calc::each_fib {|f|
        if f >= MAX     { break; }
        if f % 2u64 == 0u64 { sum += f; }
    };
    io::println(u64::str(sum));
}
