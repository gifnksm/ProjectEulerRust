extern mod euler;

use euler::prime;

fn main() {
    io::println(fmt!("%?", prime::Prime().get_at(10000u)));
}
