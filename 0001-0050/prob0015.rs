extern mod euler;

use iter::*;
use euler::prime;
use euler::monoid::*;

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

fn fact_to_uint(fs: &[(uint, Sum<int>)]) -> uint {
    let mut result = 1;
    for fs.each() |tp| {
        let (base, exp) = *tp;
        if exp.repr > 0 {
            result *= pow(base, exp.repr as uint);
        } else {
            result /= pow(base, (-exp.repr) as uint);
        }
    }
    return result;
}

fn main() {
    let primes = prime::Prime();
    let mut numer_facts = ~[];
    for uint::range(21, 40 + 1) |num| {
        let mut list = ~[];
        for prime::factors(num, &primes) |f| {
            let (b, e) = f;
            list += [ (b, Sum(e as int)) ];
        }
        numer_facts += [ list ];
    }
    let numer = mergei(numer_facts);
    let mut denom_facts = ~[];
    for uint::range(1, 20 + 1) |num| {
        let mut list = ~[];
        for prime::factors(num, &primes) |f| {
            let (b, e) = f;
            list += [ (b, Sum(-(e as int))) ];
        }
        denom_facts += [ list ];
    }
    let denom = mergei(denom_facts);
    io::println(#fmt("%u", fact_to_uint(mergei([numer, denom]))));
}
