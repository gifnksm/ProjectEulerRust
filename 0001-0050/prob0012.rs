use euler;

import euler::prime;

fn each_triangles(f: fn(&&uint) -> bool) {
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

fn num_factors(num: u64, primes: prime::prime) -> u64 {
    let mut prod = 1u64;
    for prime::factors(num, primes) {|f|
        let (_base, exp): (u64, i64) = f;
        prod *= ((exp + 1i64) as u64);
    }
    ret prod;
}

fn main() {
    let primes = prime::prime();
    for each_triangles {|t|
        let num = num_factors(t as u64, primes);
        if num > 500u64 {
            io::println(#fmt("%u -> %u", t, num_factors(t as u64, primes) as uint));
            break;
        }
    }
}
