extern mod std;
use std::sort::{ merge_sort };
use std::map::{ HashMap, set_add };

extern mod euler;
use euler::calc::{ num_to_digits };

// possible num of digits combinations
// 1 x 1 = 7 : NG 10 * 10
// 1 x 2 = 6 : NG 10 * 100
// 1 x 3 = 5 : NG 10 * 1000 = 10000
// 1 x 4 = 4 : OK
// 2 x 2 = 5 : NG 100 * 100 = 10000
// 2 x 3 = 4 : OK
// 3 x 3 = 3 : NG

pure fn fill_zero(v: &[uint], n: uint) -> ~[uint] {
    assert n >= v.len();
    vec::from_elem(n - v.len(), 0) + v
}

pure fn permutate_num(digits: &[uint], len: uint, min: uint, max: uint,
                      f: fn(uint, &[uint])->bool) {
    let min_vec = fill_zero(num_to_digits(min), len);
    let max_vec = fill_zero(num_to_digits(max), len);
    perm_sub(digits, len, to_some(min_vec), to_some(max_vec), f);

    pure fn to_some(v: &a/[uint]) -> Option<&a/[uint]> { Some(v) }
    
    pure fn perm_sub(digits: &[uint], len: uint, min: Option<&[uint]>, max: Option<&[uint]>,
                     f: fn(uint, &[uint])->bool) {
        if len == 0 {
            f(0, digits);
            return;
        }

        let unit = {
            let mut tmp = 1;
            for (len-1).times { tmp *= 10 }
            tmp
        };

        let buf = vec::to_mut(vec::from_elem(digits.len() - 1, 0));
        
        for digits.eachi |i, np| {
            let n = *np;

            let min_vec = match min {
                Some(v) if n <  v[0] => loop,
                Some(v) if n == v[0] => Some(vec::view(v, 1, v.len())),
                _ => None
            };
            let max_vec = match max {
                Some(v) if n >  v[0] => loop,
                Some(v) if n == v[0] => Some(vec::view(v, 1, v.len())),
                _ => None
            };

            for uint::range(0, i)         |j| { buf[j] = digits[j]; }
            for uint::range(i, buf.len()) |j| { buf[j] = digits[j + 1]; }
            for perm_sub(buf, len - 1, min_vec, max_vec) |num, ds| {
                if !f(num + n * unit, ds) { return; }
            }
        }
    }
}

fn main() {
    let digits = &[1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut answer = HashMap::<uint, ()>();

    // 1 x 4 = 4
    // a b = c
    // 1 < a < 10
    // 1000 < b < 10000
    // 1000 < c = ab < 10000 => 1000 / a < b < 10000 / a
    //  => 1000 < b < 10000 / a
    for permutate_num(digits, 1, 0, 9) |a, ds| {
        for permutate_num(ds, 4, 1000, 9999 / a) |b, ds| {
            let c = a * b;
            let c_digits = merge_sort(|a, b| a <= b, num_to_digits(c));
            if vec::eq(c_digits, ds) { set_add(answer, c); }
        }
    }

    // 2 x 3 = 4
    // a b = c
    // 10   < a < 100
    // 100  < b < 1000
    // 1000 < c = ab < 10000 => 1000 / a < b < 10000 / a
    // => 100 < b < 10000 / a
    for permutate_num(digits, 2, 10, 99) |a, ds| {
        for permutate_num(ds, 3, 100, 9999 / a) |b, ds| {
            let c = a * b;
            let c_digits = merge_sort(|a, b| a <= b, num_to_digits(c));
            if vec::eq(c_digits, ds) { set_add(answer, c); }
        }
    }

    let mut sum = 0;
    for answer.each_key |c| {
        sum += c;
    }
    io::println(fmt!("%u", sum));
}
