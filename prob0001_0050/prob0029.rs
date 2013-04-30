use core::hashmap::{ HashSet };

use common::extvec;
use common::prime::{ Prime };
use common::problem::{ Problem };

pub static problem: Problem<'static> = Problem {
    id: 29,
    answer: "9183",
    solver: solve
};

fn solve() -> ~str {
    let mut ps  = Prime::new();
    let mut set = HashSet::new();

    for uint::range(2, 101) |a| {
        let fs = extvec::from_iter(ps.factorize(a));
        for uint::range(2, 101) |b| {
            set.insert(fs.map(|&(base, exp)| { (base, (exp as uint) * b) }));
        }
    }

    return set.len().to_str();
}
