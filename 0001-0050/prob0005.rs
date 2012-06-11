use euler;

import euler::prime;
import euler::util;

fn merge_facti(fss: [[(u64, i64)]]) -> [(u64, i64)] {
    ret util::mergei(fss) { |f1, f2|
        let (base1, exp1): (u64, i64) = f1;
        let (base2, exp2) = f2;
        if base1 < base2 {
            util::lt
        } else if base1 > base2 {
            util::gt
        } else {
            util::eq((base1, i64::max(exp1, exp2)))
        }
    };
}

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
    ret result;
}

fn fact_to_uint(fs: [(u64, i64)]) -> u64 {
    let mut result = 1u64;
    for fs.each() { |tp|
        let (base, exp) = tp;
        if exp > 0i64 {
            result *= pow(base, exp as u64);
        } else {
            result /= pow(base, (-exp) as u64);
        }
    }
    ret result;
}

fn main() {
    let primes = prime::prime();
    let mut factors = [];
    for u64::range(1u64, 20u64 + 1u64) {|n|
        let mut list = [];
        for prime::factors(n, primes) {|f| list += [ f ]; }
        factors += [ list ];
    };
    io::println(u64::str(fact_to_uint(merge_facti(factors))));
}