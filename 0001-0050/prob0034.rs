fn main() {
    let mut facts: [mut uint * 10] = [ mut 1, 0, 0, 0, 0, 0, 0, 0, 0, 0 ];
    for uint::range(1, facts.len()) |i| {
        facts[i] = facts[i - 1] * i;
    }

    io::println(fmt!("%u", facts[9]));
    let mut answer = 0;
    for uint::range(0, facts[9].to_str().len() * facts[9]) |n| {
        let mut itr = n;
        let mut sum = 0;
        while itr > 0 {
            sum += facts[itr % 10];
            itr /= 10;
        }
        if sum == n {
            io::println(fmt!("%u => %u", n, sum));
            answer += sum;
        }
    }

    io::println(fmt!("answer: %u", answer - 1 - 2));
}
