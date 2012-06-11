use euler;

import prime = euler::prime;

fn main() {
    let mut sum = 0u64;
    for prime::prime().each {|p|
        if p >= 2000000u64 {
            break;
        }
        sum += p;
    };
    io::println(#fmt("%u", sum as uint));
}
