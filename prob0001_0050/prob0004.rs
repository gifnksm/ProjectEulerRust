use common::problem::{ Problem };

pub static problem: Problem<'static> = Problem {
    id: 4,
    answer: "906609",
    solver: solve
};

fn to_palindromic(n: uint, dup_flag: bool) -> uint {
    let cs = str::chars(n.to_str());
    let rv = vec::reversed(cs);
    let s = str::from_chars(
        if dup_flag {
            cs + vec::tail(rv).to_vec()
        } else {
            cs + rv
        }
    );
    match uint::from_str(s) {
      None    => fail!(),
      Some(x) => x
    }
}

fn dividable_pairs(num: uint, min: uint, max: uint, f: &fn(uint, uint) -> bool) {
    let mut div = uint::max(uint::div_ceil(num, max), min);
    while div * div <= num && div <= max {
        if num % div == 0 {
            if !f(div, num / div) { break; }
        }
        div += 1;
    }
}

fn solve() -> ~str {
    for [false, true].each |&dup_flag| {
        for uint::range_rev(999, 99) |seed| {
            let num = to_palindromic(seed, dup_flag);
            for dividable_pairs(num, 100, 999) |d1, d2| {
                return (d1 * d2).to_str();
            }
        }
    }

    fail!();
}

