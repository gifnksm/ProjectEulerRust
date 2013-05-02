use core::hashmap::{ HashSet };

use common::extiter::{ ExtIteratorUtil };
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
        let a_factor = ps.factorize(a).to_vec();
        for uint::range(2, 101) |b| {
            let ab_factor = a_factor
                .map(|&(base, exp)| (base, (exp) as uint * b));
            set.insert(ab_factor);
        }
    }

    return set.len().to_str();
}
