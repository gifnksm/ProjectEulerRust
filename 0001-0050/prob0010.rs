use std;
use euler;

import prime = euler::prime;
import euler::prime::{ loopable_prime };

fn main() {
    let sum = 0u;
    let primes = prime::init();
    primes.iterate { |p|
        if p >= 2000000u64 {
            ret false;
        }
        sum += p;
        ret true;
    };

    std::io::println(#fmt("%u", sum));
}
