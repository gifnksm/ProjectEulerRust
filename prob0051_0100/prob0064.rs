use core::hashmap::linear::{ LinearSet };

use common::arith::{ isqrt };
use common::calc::{ get_gcd };
use common::problem::{ Problem };

pub static problem: Problem<'static> = Problem {
    id: 64,
    answer: "1322",
    solver: solve
};

fn calc_a(n: int, (p, q, r): (int, int, int)) -> int {
    // a := |f_n(p, q, r)|
    // a <= f_n(p, q, r) < a + 1
    // r a - q <= p sqrt(n) < r (a + 1) - pq
    // (ar - q)^2 <= np^2 < ((a+1)r - q)^2
    fn arq2(a: int, r: int, q: int) -> int {
        let s = a * r - q;
        return s * s;
    }

    let np2 = n * p * p;

    let sqn = isqrt(n as uint) as int;
    let estim_a = (p * sqn + q) / r;
    let mut a = estim_a;
    assert!(arq2(a, r, q) <= np2);
    while arq2(a + 1, r, q) <= np2 {
        a = a + 1;
    }
    assert!(arq2(a, r, q) <= np2);
    return a;
}

// f_n (p, q, r) := (p sqrt(n) + q)/ r
//                = a + (p sqrt(n) + q - ar) / r
// b := q - ar
//                = a + (p sqrt(n) + b) / r
//                = a + (1 / (r / (p sqrt(n) + b)))
//                = a + (1 / (rp sqrt(n) - rb) / (np^2 - b^2))
// (p, q, r) := (rp / m, -rb / m, (np^2 - b^2) / m)
fn each_a(n: int, f: &fn(int, (int, int, int)) -> bool) {
    let mut (p, q, r) = (1, 0, 1);
    loop {
        let a = calc_a(n, (p, q, r));
        if a * a == n || p == 0 {
            p = 0; q = 0; r = 1;
        } else {
            let b = q - a * r;
            let (p2, q2, r2) = (r*p, -r*b, n*p*p - b*b);
            let m = get_gcd(get_gcd(p2 as uint, q2 as uint), r2 as uint) as int;
            p = p2 / m;
            q = q2 / m;
            r = r2 / m;
        }
        if !f(a, (p, q, r)) { break; }
    }
}

// (p sqrt(n) + q) / r
// = a + (p sqrt(n) + j) / r
// = a + 1 / (r / (p sqrt(n) + j))
// = a + 1 / r ((p sqrt(n) - j) / (np^2 - j^2))
// => r(p sqrt(n) - j) / (np^2 - j^2)
fn solve() -> ~str {
    let mut cnt = 0u;
    for int::range(1, 10001) |n| {
        let mut an = ~[];
        let mut set = LinearSet::new();
        for each_a(n) |a, pqr| {
            if set.contains(&(a, pqr)) {
                // 平方数の period は 0
                let period = if a == 0 { 0 } else { set.len() - 1 };
                if period % 2 == 1 { cnt += 1; }
                break;
            }
            set.insert((a, pqr));
            an.push(a);
        }
    }
    return cnt.to_str();
}

