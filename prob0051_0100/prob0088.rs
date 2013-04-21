use core::hashmap::{ HashSet };
use common::problem::{ Problem };

pub static problem: Problem<'static> = Problem {
    id: 88,
    answer: "7587457",
    solver: solve
};

fn each_sum_product(start: uint, end: uint, f: &fn(uint, uint, uint) -> bool) {
    for sub(start, end, 0, 1, 0) |sum, prod, len| {
        if !f(sum, prod, len) { return; }
    }

    fn sub(start: uint, end: uint, sum: uint, prod: uint, len: uint,
           f: &fn(uint, uint, uint) -> bool) {
        for uint::range(start, end / prod + 1) |n| {
            if len > 0 {
                if !f(sum + n, prod * n, len + 1) { return; }
            }

            for sub(n, end, sum + n, prod * n, len + 1) |sum1, prod1, len1| {
                if !f(sum1, prod1, len1) { return; }
            }
        }
    }
}

fn solve() -> ~str {
    let limit = 12000;

    let start = 2;
    let mut end = 4;
    let mut cnt = limit - 1;
    let mut nums = vec::from_elem(limit + 1, uint::max_value);

    while cnt > 0 {
        for each_sum_product(start, end) |sum, prod, len| {
            let k = prod - sum + len;
            if k <= limit && prod < nums[k] {
                if nums[k] == uint::max_value { cnt -= 1; }
                nums[k] = prod;
            }
        }
        end *= 2;
    }

    let mut set = HashSet::new();
    for nums.each |&n| {
        if n != uint::max_value { set.insert(n); }
    }

    let mut sum = 0;
    for set.each |&n| { sum += n; }
    return sum.to_str();
}
