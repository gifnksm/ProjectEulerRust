use to_bytes::{ IterBytes, Cb };

extern mod std;
use std::map::{ HashMap, set_add};

extern mod euler;
use euler::prime::{ Prime, factors };

impl (uint, uint) : IterBytes {
    #[inline(always)]
    pure fn iter_bytes(++lsb0: bool, f: Cb) {
        if lsb0 {
            self.first().iter_bytes(lsb0, f);
            self.second().iter_bytes(lsb0, f);
        } else {
            self.second().iter_bytes(lsb0, f);
            self.first().iter_bytes(lsb0, f);
        }
    }
}

fn main() {
    let ps  = Prime();
    let set = HashMap::<~[(uint, uint)], ()>();

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
