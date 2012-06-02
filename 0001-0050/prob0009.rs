use euler;
import calc = euler::calc;

fn find_pyrhagorean(sum: u64) -> [(u64, u64, u64)] {
    let mut answer = [];
    for uint::range(2u64, sum - 2u) { |c|
        for uint::range(1u64, uint::min((sum - c) / 2u, calc::isqrt(c*c / 2u))) { |a|
            let b = sum - c - a;
            if a * a + b * b == c * c {
                answer += [(a, b, c)];
            }
        }
    }
    ret answer;
}

fn main() {
    for find_pyrhagorean(1000u).each() { |tp|
        let (a, b, c) = tp;
        io::println(#fmt("%u^2 + %u^2 = %u^2", a, b, c));
        io::println(#fmt("prod: %u", a * b * c));
    }
}
