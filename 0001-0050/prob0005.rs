extern mod euler;

use euler::prime;
use euler::util;
use euler::monoid::*;

fn pow(base: u64, exp: u64) -> u64 {
    let mut result = 1u64;
    let mut itr = exp;
    let mut pow = base;
    while itr > 0u64 {
        if itr & 0x1u64 == 0x1u64 {
            result *= pow;
        }
        itr >>= 1u64;
        pow *= pow;
    }
    return result;
}

fn fact_to_uint(fs: &[(u64, Sum<i64>)]) -> u64 {
    let mut result = 1;
    for fs.each() |tp| {
        let (base, exp) = tp;
        if *exp > 0 {
            result *= pow(base, *exp as u64);
        } else {
            result /= pow(base, (-*exp) as u64);
        }
    }
    return result;
}

fn main() {
    let primes = prime::Prime();
    let mut factors = ~[];
    for u64::range(1u64, 20u64 + 1u64) |n| {
        let mut list = ~[];
        for prime::factors(n, primes) |f| { list += [ f ]; }
        factors += [ list ];
    };
    io::println(u64::str(fact_to_uint(mergei(factors))));
}
