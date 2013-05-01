use common::prime::{ Prime, sum_of_proper_divisors };
use common::problem::{ Problem };

pub static problem: Problem<'static> = Problem {
    id: 23,
    answer: "4179871",
    solver: solve
};

fn is_abundant(n: uint, ps: &mut Prime) -> bool {
    let sum = sum_of_proper_divisors(n, ps);
    return sum > n;
}

fn solve() -> ~str {
    let max_num = 28123;
    let mut ps = Prime::new();

    let abundant = do vec::build_sized(max_num + 1) |push| {
        for uint::range(2, max_num + 1) |n| {
            if is_abundant(n, &mut ps) { push(n); }
        }
    };

    let mut sum_of_sum_abundant = 0;
    let mut is_sum_abundant = vec::from_elem(max_num + 1, false);
    for abundant.eachi |i, &a| {
        for abundant.tailn(i).each |&b| {
            let s = a + b;
            if s > max_num { break; }
            if !is_sum_abundant[s] { sum_of_sum_abundant += s; }
            is_sum_abundant[s] = true;
        }
    }

    let sum_of_all_int = (1 + max_num) * max_num / 2;

    return (sum_of_all_int - sum_of_sum_abundant).to_str();
}
