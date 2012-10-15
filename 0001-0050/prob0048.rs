fn pow_mod(base: uint, exponent: uint, modulo: uint) -> uint {
    if base == 0 { return 0; }
    let mut acc = 1;
    for exponent.times {
        acc = (acc * base) % modulo;
    }
    return acc;
}

fn main() {
    let modulo  = 100_0000_0000;
    let mut sum = 0;
    for uint::range(1, 1000 + 1) |n| {
        sum = (sum + pow_mod(n, n, modulo)) % modulo;
    }
    io::println(fmt!("%u", sum));
}
