extern mod euler;

use euler::prime;
use euler::monoid::Sum;

fn each_triangles(f: fn(uint) -> bool) {
    let mut idx = 0;
    let mut t   = 1;
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
        let num = prime::num_of_divisors(t, &primes);
        if num > 500 {
            io::println(#fmt("%u -> %u", t, num));
            break;
        }
    }
}
