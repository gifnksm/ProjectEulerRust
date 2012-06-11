fn to_palindromic(n: u64, dup_flag: bool) -> u64 {
    let cs = str::chars(u64::to_str(n, 10u));
    let s = str::from_chars(
        if dup_flag { cs + vec::tail(vec::reversed(cs)) } else { cs + vec::reversed(cs) }
    );
    alt u64::from_str(s) {
      none    { fail }
      some(x) { ret x }
    }
}

mod my_u64 {
    pure fn div_ceil(x: u64, y: u64) -> u64 {
        let div = u64::div(x, y);
        if x % y == 0u64 { ret div;}
        else { ret div + 1u64; }
    }
}

fn dividable_pairs(num: u64, min: u64, max: u64) -> [(u64, u64)] {
    let mut div = u64::max(my_u64::div_ceil(num, max), min);
    let mut result = [];
    while div * div <= num {
        if num % div == 0u64 {
            result += [(div, num / div)];
        }
        div += 1u64;
    }
    ret result;
}

fn main() {
    let mut dup_flag = false;
    loop {
        let mut seed = 999u64;
        while (seed >= 100u64) {
            let num = to_palindromic(seed, dup_flag);
            let pairs = dividable_pairs(num, 100u64, 999u64);
            if vec::is_not_empty(pairs) {
                io::print(u64::to_str(num, 10u));
                for pairs.each() { |tp|
                    let (d1, d2) = tp;
                    io::print(#fmt(" = %u * %u", d1 as uint, d2 as uint));
                }
                io::print("\n");
            }
            seed -= 1u64;
        }
        if (!dup_flag) {
            dup_flag = true;
        } else {
            break
        }
    }
}
