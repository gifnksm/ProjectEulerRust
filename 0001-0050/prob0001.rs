import iter::*;

fn main() {
    let it  = bind uint::range(0u, 1000u, _);
    let sum = iter::foldl(it, 0u) { |accum, n|
        ret accum + if n % 3u == 0u || n % 5u == 0u { n } else { 0u };
    };
    io::println(#fmt("%u", sum));
}
