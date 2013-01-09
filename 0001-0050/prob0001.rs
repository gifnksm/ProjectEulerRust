use iter::*;

fn main() {
    let mut sum = 0u;
    for uint::range(0u, 1000u) |n| {
        if n % 3u == 0u || n % 5u == 0u { sum += n; }
    }
    io::println(fmt!("%u", sum));
}
