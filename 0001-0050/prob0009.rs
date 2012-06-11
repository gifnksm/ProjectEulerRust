use euler;
import calc = euler::calc;

fn find_pyrhagorean(sum: u64) -> [(u64, u64, u64)] {
    let mut answer = [];
    for u64::range(2u64, sum - 2u64) { |c|
        for u64::range(1u64, u64::min((sum - c) / 2u64, calc::isqrt(c*c / 2u64))) { |a|
            let b = sum - c - a;
            if a * a + b * b == c * c {
                answer += [(a, b, c)];
            }
        }
    }
    ret answer;
}

fn main() {
    for find_pyrhagorean(1000u64).each() { |tp|
        let (a, b, c) = tp;
        io::println(#fmt("%u^2 + %u^2 = %u^2", a as uint, b as uint, c as uint));
        io::println(#fmt("prod: %u", a * b * c as uint));
    }
}
