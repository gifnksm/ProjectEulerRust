fn main() {
    let mut i = 1u;
    for uint::range(0u, 63u) { |_n|
        let sum = vec::foldl(0u, vec::filter_map(str::chars(#fmt("%u", i))) { |c|
            uint::from_str(str::from_char(c))
        }) { |s, n| s + n };
        io::println(#fmt("%20u => %3u", i, sum));
        i *= 2u64;
    };
}

