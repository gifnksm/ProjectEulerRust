extern mod euler;

use euler::prime;

fn main() {
    let mut num = 600851475143;
    let mut ps = prime::Prime();
    for ps.each |p| {
        while num % p == 0 {
            num /= p;
        }
        if num == 1 {
            io::println(p.to_str());
            break;
        }
    }
}
