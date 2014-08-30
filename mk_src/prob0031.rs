#![crate_name = "prob0031"]
#![crate_type = "rlib"]

pub static EXPECTED_ANSWER: &'static str = "73682";

fn count_ways(sum: uint, coins: &[uint]) -> uint {
    if coins.len() == 1 { return 1 }

    let mut ans = 0;
    for n in range(0, sum / coins[0] + 1) {
        let d = sum - n * coins[0];
        ans += count_ways(d, coins.slice(1, coins.len()));
    }
    return ans;
}

pub fn solve() -> String {
    let coins = [ 200, 100, 50, 20, 10, 5, 2, 1 ];
    return count_ways(200, coins).to_string();
}
