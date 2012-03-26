use euler;

import iter::*;
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
            util::eq((base1, int::max(exp1, exp2)))
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
    for (base, exp) in fs {
        if exp > 0 {
            result *= pow(base, exp as u64);
        } else {
            result /= pow(base, (-exp) as u64);
        }
    }
    ret result;
}

fn main() {
    let mut primes = prime::init();
    let range  = bind uint::range(1u, 20u + 1u, _);
    let factors = iter::foldl(range, []) { |accum, num|
        let mut list = [];
        prime::factors(num, primes) { |f|
            list += [ f ];
        }
        accum + [ list ]
    };
    io::println(#fmt("%u", fact_to_uint(merge_facti(factors))));
}