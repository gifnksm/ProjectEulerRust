extern mod euler;

use iter::*;
use euler::prime;
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
        let (base, exp) = *tp;
        if exp.repr > 0 {
            result *= pow(base, exp.repr as u64);
        } else {
            result /= pow(base, (-exp.repr) as u64);
        }
    }
    return result;
}

fn main() {
    let primes = prime::Prime();
    let mut numer_facts = ~[];
    for u64::range(21u64, 40u64 + 1u64) |num| {
        let mut list = ~[];
        for prime::factors(num, &primes) |f| { list += [ f ] }
        numer_facts += [ list ];
    }
    let numer = mergei(numer_facts);
    let mut denom_facts = ~[];
    for u64::range(1u64, 20u64 + 1u64) |num| {
        let mut list = ~[];
        for prime::factors(num, &primes) |f| {
            let (b, e) = f;
            list += [ (b, Sum(-e.repr)) ];
        }
        denom_facts += [ list ];
    }
    let denom = mergei(denom_facts);
    io::println(#fmt("%u", fact_to_uint(mergei([numer, denom])) as uint));
}
