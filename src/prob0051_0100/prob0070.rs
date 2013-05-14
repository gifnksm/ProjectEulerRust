use common::prime;
use common::calc::{ digit_histogram };
use common::problem::{ Problem };

pub static problem: Problem<'static> = Problem {
    id: 70,
    answer: "8319823",
    solver: solve
};

fn solve() -> ~str {
    let limit = 10000000;

    // n / phi(n) = 1 / \Pi_{k=1}^d (1 - 1/p_k)
    // => p^k / phi(p^k) = p / phi(p)
    // p is greater then n / phi(p) is less
    // 
    // phi(p) = p - 1 (if p is prime) => phi(p) is not permutation of p
    // phi(p1 * p2) = (p1 - 1) * (p2 - 1)

    let mut min_n   = 0;
    let mut min_n_phi = float::infinity;
    for prime::each |p1| {
        if p1 * p1 > limit { break; }
        for prime::each |p2| {
            if p2 < p1 { loop; }

            let n = p1 * p2;
            if n > limit { break; }

            let phi = (p1 - 1) * (p2 - 1);
            let ds1 = digit_histogram(n);
            let ds2 = digit_histogram(phi);
            if ds1 != ds2 { loop; }

            let n_phi = (n as float) / (phi as float);
            if n_phi < min_n_phi {
                min_n_phi = n_phi;
                min_n     = n;
            }
        }
    }

    return min_n.to_str();
}
