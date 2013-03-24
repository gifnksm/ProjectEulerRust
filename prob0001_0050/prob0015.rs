use common::prime::{ Prime, Factors, factors_to_uint };
use common::monoid::{ Sum, merge_as, mergei_as };
use common::problem::{ Problem };

pub static problem: Problem<'static> = Problem {
    id: 15,
    answer: "137846528820",
    solver: solve
};

fn solve() -> ~str {
    let mut primes = Prime::new();

    let mut numer_facts = ~[];
    for uint::range(21, 40 + 1) |num| {
        numer_facts.push(iter::to_vec(&Factors::new(num, &mut primes)));
    }
    let numer = mergei_as(numer_facts, |i| Sum(i as int));

    let mut denom_facts = ~[];
    for uint::range(1, 20 + 1) |num| {
        denom_facts.push(iter::to_vec(&Factors::new(num, &mut primes)));
    }
    let denom = mergei_as(denom_facts, |i| Sum(-(i as int)));

    return factors_to_uint(&merge_as(numer, denom, Sum)).to_str();
}
