use common::calc::{ num_to_digits };

fn is_palindromic(n: uint, radix: uint) -> bool {
    let digits = num_to_digits(n, radix);
    for uint::range(0, digits.len() / 2) |i| {
        if digits[i] != digits[digits.len() - 1 - i] { return false;}
    }
    return true;
}

fn to_palindromic(n: uint, radix: uint, is_odd: bool) -> uint{
    let digits = num_to_digits(n, radix);
    let mut num = 0;
    for digits.each |d| { num = num * radix + *d; }
    let start = if is_odd { 1 } else { 0 };
    for uint::range(start, digits.len()) |i| {
        num = num * radix + digits[digits.len() - 1 - i];
    }
    return num;
}

pub fn solve() -> ~str {
    let order_array = &[ 1, 10, 100, 1000, 1000, 10000 ];
    let mut sum = 0;
    for uint::range(0, order_array.len() - 1) |i| {
        for [true, false].each |b| {
            let (start, end) = (order_array[i], order_array[i + 1]);
            for uint::range(start, end) |n| {
                let n = to_palindromic(n, 10, *b);
                if n >= 1000000 { break; }
                if is_palindromic(n, 2) {
                    sum += n;
                }
            }
        }
    }

    return sum.to_str();
}

