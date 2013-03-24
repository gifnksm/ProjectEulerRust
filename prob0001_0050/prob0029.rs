use core::hashmap::linear::{ LinearSet };

use common::prime::{ Prime, factors };
use common::problem::{ Problem };

pub static problem: Problem<'static> = Problem {
    id: 29,
    answer: "9183",
    solver: solve
};

fn solve() -> ~str {
    let mut ps  = Prime();
    let mut set = LinearSet::new();

    for uint::range(2, 101) |a| {
        let mut fs = ~[];
        for factors(a, &mut ps) |f| {
            fs += ~[f];
        }
        for uint::range(2, 101) |b| {
            set.insert(fs.map(|f| { (f.first(), f.second() * b) }));
        }
    }

    return set.len().to_str();
}
