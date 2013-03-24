// p(n) = n^2 + an + b is prime for n = 0 .. N
// p(0) = b         => b must be prime
// p(1) = 1 + a + b => a > -(1+b)
// p(2) = 4 + 2a + b

use euler::prime::{ Prime };

fn get_len(a: int, b: int, ps: &mut Prime) -> uint {
    let mut nu = 0;
    loop {
        let n = nu as int;
        let mut val = n * n + a * n + b;
        for ps.each |p| {
            if (p as int) == val {
                nu += 1;
                break;
            }
            if (p as int) > val {
                return nu;
            }
        }
    }
}

pub fn solve() -> int {
    let mut ps = Prime();
    let mut ans_a = 0;
    let mut ans_b = 0;
    let mut ans_len = 0;
    let mut i = 0;
    loop {
        let mut bu = ps.get_at(i);
        if bu >= 1000 { break; }
        let b = bu as int;
        for int::range(-b, 1000) |a| {
            let len = get_len(a, b, &mut ps);
            if len > ans_len {
                ans_len = len;
                ans_a = a;
                ans_b = b;
            }
        }
        i += 1;
    }
    // io::println(fmt!("n^2 + %dn + %d => %u len", ans_a, ans_b, ans_len));
    // io::println(fmt!("a * b = %d", ans_a * ans_b));
    return ans_a * ans_b;
}
