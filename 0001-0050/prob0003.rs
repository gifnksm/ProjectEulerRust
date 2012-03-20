use euler;

import prime = euler::prime;
import euler::prime::{ loopable_prime };

fn main() {
    let num = 600851475143u64;
    let primes = prime::init();
    primes.iterate { |p|
        while num % p == 0u64 {
            num /= p;
        }
        if num == 1u {
            io::println(#fmt("%u", p));
            ret false;
        }
        ret true;
    };
}
