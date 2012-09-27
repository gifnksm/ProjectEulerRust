// p(n) = n^2 + an + b is prime for n = 0 .. N
// p(0) = b         => b must be prime
// p(1) = 1 + a + b => a > -(1+b)
// p(2) = 4 + 2a + b

extern mod euler;
use euler::prime::{ Prime };

fn get_len(a: int, b: int, ps: &Prime) -> uint {
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

fn main() {
    let ps = Prime();
    let mut ans = { len: 0, a: 0, b: 0 };
    for ps.each |bu| {
        if bu >= 1000 { break; }
        let b = bu as int;
        for int::range(-b, 1000) |a| {
            let len = get_len(a, b, &ps);
            if len > ans.len {
                ans.len = len;
                ans.a = a;
                ans.b = b;
            }
        }
    }
    io::println(fmt!("n^2 + %dn + %d => %u len", ans.a, ans.b, ans.len));
    io::println(fmt!("a * b = %d", ans.a * ans.b));
}
