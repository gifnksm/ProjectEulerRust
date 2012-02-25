use std;

fn main() {
    let sum = 0u;
    uint::range(0u, 1000u) { |n|
        if n % 3u == 0u || n % 5u == 0u {
            sum += n;
        }
    }
    std::io::println(#fmt("%u", sum));
}
