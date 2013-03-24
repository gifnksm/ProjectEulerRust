pub fn solve() -> uint {
    let mut sum = 0;
    for uint::range(0, 1000) |n| {
        if n % 3 == 0 || n % 5 == 0 {
            sum += n;
        }
    }
    return sum;
}
