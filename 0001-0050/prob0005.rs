use std;
use euler;

import prime = euler::prime;
import euler::prime::{ loopable_prime, iterable_factors };

fn merge_fact(fs1: [(u64, u64)], fs2: [(u64, u64)]) -> [(u64, u64)] {
    let result = [];
    let i1 = 0u, i2 = 0u;
    let len1 = vec::len(fs1), len2 = vec::len(fs2);
    while (i1 < len1 && i2 < len2) {
        let (base1, exp1) = fs1[i1];
        let (base2, exp2) = fs2[i2];
        if (base1 < base2) {
            result += [(base1, exp1)];
            i1 += 1u64;
        } else if (base1 > base2) {
            result += [(base2, exp2)];
            i2 += 1u64;
        } else {
            result += [(base1, uint::max(exp1, exp2))];
            i1 += 1u64;
            i2 += 1u64;
        }
    }
    if i1 < len1 {
        result += vec::slice(fs1, i1, len1);
    }
    if i2 < len2 {
        result += vec::slice(fs2, i2, len2);
    }
    ret result;
}

fn merge_facti(fss: [[(u64, u64)]]) -> [(u64, u64)] {
    ret alt vec::len(fss) {
      0u64 { [] }
      1u64 { fss[0] }
      l    {
        let pre  = merge_facti(vec::slice(fss, 0u64, l / 2u64));
        let post = merge_facti(vec::slice(fss, l / 2u64, l));
        merge_fact(pre, post)
      }
    }
}

fn pow(base: u64, exp: u64) -> u64 {
    let result = 1u64;
    let itr = exp;
    let pow = base;
    while itr > 0u64 {
        if itr & 0x1u64 == 0x1u64 {
            result *= pow;
        }
        itr >>= 1u64;
        pow *= pow;
    }
    ret result;
}

fn fact_to_uint(fs: [(u64, u64)]) -> u64 {
    let result = 1u64;
    for (base, exp) in fs {
        result *= pow(base, exp);
    }
    ret result;
}

fn main() {
    let primes = prime::init();
    let factors = vec::map(vec::enum_uints(1u64, 20u64)) { |num|
        iter::to_list(prime::factors(num, primes))
    };
    std::io::println(#fmt("%u", fact_to_uint(merge_facti(factors))));
}