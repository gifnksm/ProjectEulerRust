fn to_palindromic(n: u64, dup_flag: bool) -> u64 {
    let cs = str::chars(u64::to_str(n, 10u));
    let s = str::from_chars(
        if dup_flag { cs + vec::tail(vec::reversed(cs)) } else { cs + vec::reversed(cs) }
    );
    match u64::from_str(s) {
      None    => fail,
      Some(x) => x
    }
}

mod my_u64 {
    pure fn div_ceil(x: u64, y: u64) -> u64 {
        let div = x / y;
        if x % y == 0u64 { return div;}
        else { return div + 1u64; }
    }
}

fn dividable_pairs(num: u64, min: u64, max: u64, f: fn(u64, u64) -> bool) {
    let mut div = u64::max(my_u64::div_ceil(num, max), min);
    while div * div <= num {
        if num % div == 0u64 {
            if !f(div, num / div) { break; }
        }
        div += 1u64;
    }
}

fn main() {
    let mut dup_flag = false;
    loop {
        let mut seed = 999u64;
        while (seed >= 100u64) {
            let num = to_palindromic(seed, dup_flag);
            let mut exist_flag = false;
            for dividable_pairs(num, 100u64, 999u64) |d1, d2| {
                if exist_flag { io::print(#fmt("%u", num as uint)); }
                exist_flag = true;
                io::print(#fmt(" = %u * %u", d1 as uint, d2 as uint));
            }
            if exist_flag { io::println(""); }
            seed -= 1u64;
        }
        if (!dup_flag) {
            dup_flag = true;
        } else {
            break
        }
    }
}
