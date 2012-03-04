use std;

fn find_pyrhagorean(sum: u64) -> [(u64, u64, u64)] {
    let answer = [];
    uint::range(2u64, sum - 2u) { |c|
        uint::range(1u64, (sum - c) / 2u) { |a|
            let b = sum - c - a;
            if a * a + b * b == c * c {
                answer += [(a, b, c)];
            }
        }
    }
    ret answer;
}

fn main() {
    for (a, b, c) in find_pyrhagorean(1000u) {
        std::io::println(#fmt("%u^2 + %u^2 = %u^2", a, b, c));
        std::io::println(#fmt("prod: %u", a * b * c));
    }
}
