extern mod euler;

use euler::prime;
use euler::monoid::Sum;

fn each_triangles(f: fn(uint) -> bool) {
    let mut idx = 0u;
    let mut t   = 1u;
    loop {
        if !f(t) {
            break;
        }
        idx += 1u;
        t   += idx + 1u;
    }
}

fn main() {
    let primes = prime::Prime();
    for each_triangles |t| {
        let num = prime::num_of_divisors(t as u64, &primes);
        if num > 500u64 {
            io::println(#fmt("%u -> %u", t, num as uint));
            break;
        }
    }
}
