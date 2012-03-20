pure fn fib(prev: uint, cur: uint) -> (uint, uint) {
    ret (cur, prev + cur);
}

fn main() {
    const MAX: uint = 4000000u;
    let (prev, cur) = (1u, 1u);
    let sum = 0u;
    while cur < MAX {
        if (cur % 2u == 0u) {
            sum += cur;
        }
        let (prev2, cur2) = fib(prev, cur);
        prev = prev2;
        cur = cur2;
    }
    io::println(#fmt("%u", sum));
}