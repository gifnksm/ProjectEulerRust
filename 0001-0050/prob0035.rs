extern mod euler;
use euler::prime::{ Prime };
use euler::calc::{ num_to_digits };

fn is_circular_prime(n: uint, ps: &Prime) -> bool {
    let buf = num_to_digits(n, 10);

    for uint::range(1, buf.len()) |i| {
        let mut num = 0;
        for uint::range(0, buf.len()) |j| {
            num = num * 10 + (buf[(i + j) % buf.len()] as uint);
        }
        if !ps.is_prime(num) { return false; }
    }
    return true;
}

fn main() {
    let ps = Prime();
    let mut cnt = 0;
    for ps.each() |p| {
        if p >= 1000000 { break; }
        if is_circular_prime(p, &ps) {
            io::println(fmt!("%u", p));
            cnt += 1;
        }
    }
    io::println(fmt!("answer: %u", cnt));
}
