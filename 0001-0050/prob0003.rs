extern mod euler;

use euler::prime;

fn main() {
    let mut num = 600851475143;
    for prime::Prime().each |p| {
        while num % p == 0 {
            num /= p;
        }
        if num == 1 {
            io::println(p.to_str());
            break;
        }
    }
}
