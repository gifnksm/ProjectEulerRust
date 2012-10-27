extern mod std;
use std::sort::{ merge_sort };

extern mod euler;
use euler::prime::{ Prime };
use euler::calc::{ num_to_digits, permutate_num };

fn main() {
    let ps = Prime();
    let d = 3330;
    for ps.each |p1| {
        if p1 < 1000 { loop; }
        if p1 > 9999 - 2 * d { fail; }
        if p1 == 1487 { loop; }

        let p2 = p1 + d;
        let p3 = p2 + d;
        let sorted = merge_sort(num_to_digits(p1, 10), |a, b| a <= b);
        if merge_sort(num_to_digits(p2, 10), |a, b| a <= b) != sorted { loop }
        if merge_sort(num_to_digits(p3, 10), |a, b| a <= b) != sorted { loop }

        if !ps.is_prime(p2) { loop; }
        if !ps.is_prime(p3) { loop; }
        io::println(fmt!("answer: %u%u%u", p1, p2, p3));
        break;
    }
}
