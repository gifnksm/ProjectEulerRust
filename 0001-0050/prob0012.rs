use euler;

import iter::*;
import euler::prime;

fn gen_triangles(&trigs: [uint]) {
    alt vec::len(trigs) {
      0u { trigs = [1u]; }
      x  { trigs += [trigs[x - 1u] + x + 1u]; }
    }
}

fn num_factors(num: u64, primes: prime::prime) -> u64 {
    let mut prod = 1u;
    prime::factors(num, primes) { |f|
        let (_base, exp): (u64, i64) = f;
        prod *= ((exp + 1) as u64)
    }
    ret prod;
}

fn main() {
    let primes = prime::prime();
    let mut trigs  = [];
    while true {
        gen_triangles(trigs);
        let t = vec::last(trigs);
        let num = num_factors(t, primes);
        if num > 500u {
            io::println(#fmt("%u -> %u", t, num_factors(t, primes)));
            break;
        }
    }
}
