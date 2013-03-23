extern mod euler;
use euler::prime::{ Prime };
use euler::arith::{ isqrt };

fn is_goldbach(n: uint, ps: &mut Prime) -> bool {
    for uint::range(1, isqrt(n / 2) + 1) |s| {
        let sq = s * s * 2;
        if sq > n { return false; }
        if ps.is_prime(n - sq) { return true; }
    }
    return false;
}

fn main() {
    let mut ps = Prime();
    let mut n = 1;
    loop {
        n += 2;
        if ps.is_prime(n) { loop; }
        if !is_goldbach(n, &mut ps) {
            io::println(fmt!("%u is not goldbach number", n));
            break
        }
    }
    io::println(fmt!("answer: %u", n));
}
