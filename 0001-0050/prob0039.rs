use core::hashmap::linear::{ LinearMap };

extern mod euler;
use euler::arith::{ isqrt };
use euler::calc::{ get_gcd };

fn main() {
    // a + b + c = 2m(m + n) <= L
    // 1 <= n <= L / 2m - m
    // if n == 1, a + b + c = 2m^2 + 2m <= L
    // m <= (sqrt(1 + L) - 1)/2
    let limit = 1000;
    let mut map   = LinearMap::new::<uint, uint>();

    for uint::range(1, (isqrt(1 + limit) - 1) / 2) |m| {
        for uint::range(1, uint::min(1 + limit / (2 * m) - m, m)) |n| {
            if (m - n) % 2 == 0 { loop; }
            if get_gcd(m, n) != 1 { loop; }
            let (a, b, c) = (m * m - n * n, 2 * m * n, m * m + n * n);
            let s = a + b + c;
            for uint::range(1, limit / s + 1) |k| {
                let new_val = map.find(&(k * s)).map_default(1, |&v| v + 1);
                map.insert(k * s, new_val);
            }
        }
    }

    let mut max_key = 0;
    let mut max_val = 0;
    for map.each |&(&k, &v)| {
        if max_val < v {
            max_key = k;
            max_val = v;
        }
    }

    io::println(fmt!("answer: %?", max_key));
}

