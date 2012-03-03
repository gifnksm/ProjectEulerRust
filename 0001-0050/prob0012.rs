use std;

fn gen_triangles(&trigs: [uint]) {
    alt vec::len(trigs) {
      0u { trigs = [1u]; }
      x  { trigs += [trigs[x - 1u] + x + 1u]; }
    }
}

fn gen_prime(&primes: [u64]) {
    let num = alt vec::last(primes) {
      none       { primes =  [2u64]; ret }
      some(2u64) { primes += [3u64]; ret }
      some(x)    { x + 2u64 }
    };

    while true {
        for p in primes {
            if p * p > num {
                primes += [num];
                ret;
            }
            if num % p == 0u64 {
                break;
            }
        }
        num += 2u64;
    }
    fail;
}

fn div_mult(&num: u64, f: u64) -> u64 {
    let exp = 0u64;
    while (num % f == 0u64) {
        exp += 1u64;
        num /= f;
    }
    ret exp;
}

fn factorize(num: u64, &primes: [u64]) -> [(u64, u64)] {
    let itr = num;
    let result = [];

    for p in primes {
        let exp = div_mult(itr, p);
        if exp > 0u64 {
            result += [(p, exp)];
        }
    }

    while itr != 1u64 {
        gen_prime(primes);
        let p = vec::last_total(primes);
        let exp = div_mult(itr, p);
        if exp > 0u64 {
            result += [(p, exp)];
        }
    }

    ret result;
}

fn num_factors(num: u64, &primes: [u64]) -> u64 {
    let facts = factorize(num, primes);
    ret vec::foldl(1u, facts) { |prod, tuple|
        let (_base, exp) = tuple;
        prod * (exp + 1u)
    };
}

fn main() {
    let trigs  = [];
    let primes = [];
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
