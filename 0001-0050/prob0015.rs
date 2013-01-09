extern mod euler;

use euler::prime;
use euler::monoid::*;

impl int : Add<int,int> {
    pure fn add(&self, other: &int) -> int { int::add(*self, *other) }
}

fn pow(base: uint, exp: uint) -> uint {
    let mut result = 1;
    let mut itr = exp;
    let mut pow = base;
    while itr > 0 {
        if itr & 0x1 == 0x1 {
            result *= pow;
        }
        itr >>= 1;
        pow *= pow;
    }
    return result;
}

fn fact_to_uint(fs: &[(uint, int)]) -> uint {
    let mut result = 1;
    for fs.each() |tp| {
        let (base, exp) = *tp;
        if exp > 0 {
            result *= pow(base, exp as uint);
        } else {
            result /= pow(base, (-exp) as uint);
        }
    }
    return result;
}

fn main() {
    let primes = prime::Prime();
    let mut numer_facts = ~[];
    for uint::range(21, 40 + 1) |num| {
        let mut list = ~[];
        for prime::factors(num, &primes) |f| { list += [ f ]; }
        numer_facts.push(move list);
    }
    let numer = mergei_as(numer_facts, |i| Sum(i as int));

    let mut denom_facts = ~[];
    for uint::range(1, 20 + 1) |num| {
        let mut list = ~[];
        for prime::factors(num, &primes) |f| { list += [ f ]; }
        denom_facts.push(move list);
    }
    let denom = mergei_as(denom_facts, |i| Sum(-(i as int)));

    io::println(fmt!("%u", fact_to_uint(merge_as(numer, denom, Sum))));
}
