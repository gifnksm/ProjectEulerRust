extern mod euler;
use euler::calc::{ permutate_num };
use euler::prime::{ Prime };

fn main() {
    let ps = Prime();
    for permutate_num(&[7, 6, 5, 4, 3, 2, 1], 7, 0, 9999999) |num, _rest| {
        if ps.is_prime(num) {
            io::println(fmt!("answer: %u", num));
            break;
        }
    }
}
