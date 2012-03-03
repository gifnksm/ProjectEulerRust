use std;

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

fn main() {
    let sum = 0u;

    let primes = [];
    while true {
        gen_prime(primes);
        let p = vec::last_total(primes);
        if p >= 2000000u64 {
            break;
        }
        sum += p;
    }

    std::io::println(#fmt("%u", sum));
}
