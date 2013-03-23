extern mod euler;

use euler::prime::{ Prime, sum_of_proper_divisors };

fn main() {
    let mut p = Prime();
    let elms = vec::from_fn(10000, |n| sum_of_proper_divisors(n, &mut p));
    let mut amicables = ~[];
    for elms.eachi |n, sum| {
        if *sum >= n { loop }
        if *sum < elms.len() && elms[*sum] == n {
            amicables += [(*sum, n)];
        }
    }

    let mut sum = 0;
    for amicables.each |pair| {
        let (a, b) = *pair;
        sum += a + b;
    }
    io::println(fmt!("%?", sum));
}
