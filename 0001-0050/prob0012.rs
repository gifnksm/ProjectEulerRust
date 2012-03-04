use std;
use euler;

import prime = euler::prime;
import euler::prime::{ loopable_prime };

fn gen_triangles(&trigs: [uint]) {
    alt vec::len(trigs) {
      0u { trigs = [1u]; }
      x  { trigs += [trigs[x - 1u] + x + 1u]; }
    }
}

fn div_mult(&num: u64, f: u64) -> u64 {
    let exp = 0u64;
    while (num % f == 0u64) {
        exp += 1u64;
        num /= f;
    }
    ret exp;
}

fn factorize(num: u64, &primes: prime::prime) -> [(u64, u64)] {
    let itr = num;
    let result = [];

    primes.loop { |p|
        let exp = div_mult(itr, p);
        if exp > 0u64 {
            result += [(p, exp)];
        }
        ret itr != 1u;
    }

    ret result;
}

fn num_factors(num: u64, &primes: prime::prime) -> u64 {
    let facts = factorize(num, primes);
    ret vec::foldl(1u, facts) { |prod, tuple|
        let (_base, exp) = tuple;
        prod * (exp + 1u)
    };
}

fn main() {
    let primes = prime::init();
    let trigs  = [];
    while true {
        gen_triangles(trigs);
        let t = vec::last_total(trigs);
        let num = num_factors(t, primes);
        if num > 500u {
            std::io::println(#fmt("%u -> %u", t, num_factors(t, primes)));
            break;
        }
    }
}
