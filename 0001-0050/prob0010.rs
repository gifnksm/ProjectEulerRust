extern mod euler;

use prime = euler::prime;

fn main() {
    let mut sum = 0;
    for prime::Prime().each |p| {
        if p >= 2000000 {
            break;
        }
        sum += p;
    }
    io::println(fmt!("%u", sum));
}
