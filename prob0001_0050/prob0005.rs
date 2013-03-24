use common::prime;
use common::monoid::{ mergei_as, Max };
use common::problem::{ Problem };

pub static problem: Problem<'static> = Problem {
    id: 5,
    answer: "232792560",
    solver: solve
};

fn pow(base: uint, exp: uint) -> uint {
    let mut result = 1;
    let mut itr = exp;
    let mut pow = base;
    while itr > 0 {
        if itr & 0x1 == 0x1 {
            result *= pow;
        }
        itr >>= 1;
        pow *= pow;
    }
    return result;
}

fn fact_to_uint(fs: &[(uint, uint)]) -> uint {
    let mut result = 1;
    for fs.each() |tp| {
        let (base, exp) = *tp;
        result *= pow(base, exp);
    }
    return result;
}

fn solve() -> ~str {
    let mut primes = prime::Prime();
    let mut factors = ~[];
    for uint::range(1, 20 + 1) |n| {
        let mut list = ~[];
        for prime::factors(n, &mut primes) |f| { list += [ f ]; }
        factors.push(list);
    };

    return fact_to_uint(mergei_as(factors, Max)).to_str();
}
