use std;
use euler;

import prime = euler::prime;
import euler::prime::{ loopable_prime, iterable_factors };

fn gen_triangles(&trigs: [uint]) {
    alt vec::len(trigs) {
      0u { trigs = [1u]; }
      x  { trigs += [trigs[x - 1u] + x + 1u]; }
    }
}

fn num_factors(num: u64, &primes: prime::prime) -> u64 {
    ret iter::foldl(prime::factors(num, primes), 1u) { |prod, tuple|
        let (_base, exp): (u64, i64) = tuple;
        ret prod * ((exp + 1) as u64);
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
