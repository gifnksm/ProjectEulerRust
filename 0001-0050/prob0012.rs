extern mod euler;

use euler::prime;
use euler::calc::{ each_triangles };

fn main() {
    let mut primes = prime::Prime();
    for each_triangles |t| {
        let num = prime::num_of_divisors(t, &mut primes);
        if num > 500 {
            io::println(fmt!("%u -> %u", t, num));
            break;
        }
    }
}
