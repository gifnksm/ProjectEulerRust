use std;

fn to_palindromic(n: u64, dup_flag: bool) -> u64 {
    let cs = str::chars(u64::to_str(n, 10u));
    let s = str::from_chars(
        if dup_flag { cs + vec::tail(vec::reversed(cs)) } else { cs + vec::reversed(cs) }
    );
    alt u64::from_str(s, 10u) {
      none    { fail }
      some(x) { ret x }
    }
}

fn div_ceil(a: u64, b: u64) -> u64 {
    let d = a / b;
    ret if a % b != 0u64 { d + 1u64 } else { d };
}

fn dividable_pairs(num: u64, min: u64, max: u64) -> [(u64, u64)] {
    let div = u64::max(div_ceil(num, max), min);
    let result = [];
    while div * div <= num {
        if num % div == 0u64 {
            result += [(div, num / div)];
        }
        div += 1u64;
    }
    ret result;
}

fn main() {
    let dup_flag = false;
    while true {
        let seed = 999u64;
        while (seed >= 100u64) {
            let num = to_palindromic(seed, dup_flag);
            let pairs = dividable_pairs(num, 100u64, 999u64);
            if vec::is_not_empty(pairs) {
                std::io::print(#fmt("%u", num));
                for (d1, d2) in pairs {
                    std::io::print(#fmt(" = %u * %u", d1, d2));
                }
                std::io::print("\n");
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
