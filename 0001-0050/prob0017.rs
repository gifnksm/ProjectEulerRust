fn to_word_under10(n: uint) -> ~str {
    return match n {
        0 => ~"zero",
        1 => ~"one",
        2 => ~"two",
        3 => ~"three",
        4 => ~"four",
        5 => ~"five",
        6 => ~"six",
        7 => ~"seven",
        8 => ~"eight",
        9 => ~"nine",
        _ => fail
    };
}

fn to_word_under20(n: uint) -> ~str {
    assert n < 20;
    if n < 10 { return to_word_under10(n); }
    return match n {
        10 => ~"ten",
        11 => ~"eleven",
        12 => ~"twelve",
        13 => ~"thirteen",
        14 => ~"fourteen",
        15 => ~"fifteen",
        16 => ~"sixteen",
        17 => ~"seventeen",
        18 => ~"eighteen",
        19 => ~"nineteen",
        _  => fail
    };
}

fn to_word_under100(n: uint) -> ~str {
    assert n < 100;
    if n < 20 { return to_word_under20(n); }

    let prefix = match n / 10 {
        0 | 1 => fail,
        2 => ~"twenty",
        3 => ~"thirty",
        4 => ~"forty",
        5 => ~"fifty",
        6 => ~"sixty",
        7 => ~"seventy",
        8 => ~"eighty",
        9 => ~"ninety",
        _ => fail
    };
    if n % 10 != 0 {
        return prefix + "-" + to_word_under10(n % 10);
    } else {
        return move prefix;
    }
}

fn to_word_under1000(n: uint) -> ~str {
    assert n < 1000;
    if n < 100 { return to_word_under100(n); }

    let prefix = to_word_under10(n / 100) + " hundred";
    if n % 100 != 0 {
        return prefix + " and " + to_word_under100(n % 100);
    } else {
        return move prefix;
    }
}

fn to_word(n: uint) -> ~str {
    assert n <= 1000;
    if n < 1000 { return to_word_under1000(n); }
    return ~"one thousand";
}

fn main() {
    let mut sum = 0;
    for uint::range(0, 1000) |i| {
        let n = i + 1;
        io::println(fmt!("%4u => %s", n, to_word(n)));
        for str::each(to_word(n)) |b| {
            if b != ('-' as u8) && b != (' ' as u8) {
                sum += 1;
            }
        }
    }
    io::println(fmt!("%u", sum));
}