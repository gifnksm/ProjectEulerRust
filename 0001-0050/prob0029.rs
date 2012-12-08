extern mod std;
use std::map::{ HashMap, Set, set_add };

extern mod euler;
use euler::prime::{ Prime, factors };

fn main() {
    let ps  = Prime();
    let set = HashMap();

    for uint::range(2, 101) |a| {
        let mut fs = ~[];
        for factors(a, &ps) |f| {
            fs += ~[f];
        }
        for uint::range(2, 101) |b| {
            set_add(set, fs.map(|f| { (f.first(), f.second() * b) }));
        }
    }
    io::println(fmt!("%u", set.size()));
}
