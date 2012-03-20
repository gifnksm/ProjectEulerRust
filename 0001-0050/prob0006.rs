fn sum_of_square(n: u64) -> u64 {
    ret n * (n + 1u64) * (2u64 * n + 1u64) / 6u64;
}

fn sum_of_seq(n: u64) -> u64 {
    ret n * (n + 1u64) / 2u64;
}

fn square_of_sum(n: u64) -> u64 {
    let s = sum_of_seq(n);
    ret s * s;
}

fn main() {
    let sq_of_sum = square_of_sum(100u64);
    let sum_of_sq = sum_of_square(100u64);
    io::println(#fmt("%u - %u = %u", sq_of_sum, sum_of_sq, sq_of_sum - sum_of_sq));
}
