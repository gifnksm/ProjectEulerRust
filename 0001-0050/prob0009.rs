extern mod euler;
use arith = euler::arith;

fn find_pyrhagorean(sum: uint) -> ~[(uint, uint, uint)] {
    let mut answer = ~[];
    for uint::range(2, sum - 2) |c| {
        for uint::range(1, uint::min((sum - c) / 2, arith::isqrt(c*c / 2))) |a| {
            let b = sum - c - a;
            if a * a + b * b == c * c {
                answer += [(a, b, c)];
            }
        }
    }
    return answer;
}

fn main() {
    for find_pyrhagorean(1000).each() |tp| {
        let (a, b, c) = *tp;
        io::println(fmt!("%u^2 + %u^2 = %u^2", a, b, c));
        io::println(fmt!("prod: %u", a * b * c));
    }
}
