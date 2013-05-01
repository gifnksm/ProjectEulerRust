use core::iterator::{ IteratorUtil };

use common::prime::{ Prime, factors_to_uint };
use common::monoid::{ Max, MergeMultiMonoidIterator, Wrap };
use common::problem::{ Problem };

pub static problem: Problem<'static> = Problem {
    id: 5,
    answer: "232792560",
    solver: solve
};

fn solve() -> ~str {
    let mut ps = Prime::new();

    let fs = do vec::from_fn(20) |i| { ps.factorize(i + 1) };
    let it = MergeMultiMonoidIterator::new(
        fs.map(|&x| x.transform(|(base, exp)| (base, Max(exp))))
    ).transform(|(base, m)| (base, m.unwrap()));
    return factors_to_uint(it).to_str();
}
