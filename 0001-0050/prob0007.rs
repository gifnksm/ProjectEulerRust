use euler;

import prime = euler::prime;

fn main() {
    let idx = 10000u64;
    let mut primes = prime::init();
    io::println(#fmt("%u", prime::get_at(primes, idx)));
}
