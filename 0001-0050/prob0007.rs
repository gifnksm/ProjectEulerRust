extern mod euler;

use euler::prime;

fn main() {
    let mut ps = prime::Prime();
    io::println(fmt!("%?", ps.get_at(10000u)));
}
