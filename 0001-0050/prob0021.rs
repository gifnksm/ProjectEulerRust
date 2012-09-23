extern mod euler;

use euler::prime::{ Prime, sum_of_proper_divisors };

fn main() {
    let p = Prime();
    let elms = vec::from_fn(10000, |n| sum_of_proper_divisors(n as u64, &p));
    let mut amicables = ~[];
    for elms.eachi |i, sum| {
        let n = i as u64;
        if sum >= n { loop }
        if sum < (elms.len() as u64) && elms[sum] == n {
            amicables += [(sum, n)];
        }
    }

    let mut sum = 0;
    for amicables.each |pair| {
        let (a, b) = *pair;
        sum += a + b;
    }
    io::println(fmt!("%?", sum));
}
