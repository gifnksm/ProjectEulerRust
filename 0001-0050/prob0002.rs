use euler;

import euler::calc;

fn main() {
    const MAX: uint = 4000000u;
    let mut sum = 0u;
    for calc::each_fib {|f|
        if f >= MAX     { break; }
        if f % 2u == 0u { sum += f; }
    };
    io::println(#fmt("%u", sum));
}
