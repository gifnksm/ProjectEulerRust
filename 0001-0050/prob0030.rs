extern mod std;
use std::sort::{ quick_sort };

extern mod euler;
use euler::calc::{ combinate_overlap };

// 9^5     = 59049
// 9999    => 9^5 * 4 = 236196
// 99999   => 9^5 * 5 = 295245
// 999999  => 9^5 * 6 = 354294
// 9999999 => 9^5 * 7 = 413343

// 1-6 digits numbers meet conditions
fn main() {
    let pows = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9].map(|n| int::pow(*n, 5) as uint);
    let nums = [mut 0, 0, 0, 0, 0, 0, 0];
    let mut sum = 0;

    for combinate_overlap(~[0, 1, 2, 3, 4, 5, 6, 7, 8, 9], 7) |comb| {
        let mut num = 0;
        for comb.each |n| { num += pows[*n]; }

        let mut itr = num;
        for uint::range(0, nums.len()) |i| {
            nums[i] = itr % 10;
            itr /= 10;
        }
        quick_sort(nums, |a, b| a < b);
        if vec::eq(nums, comb) {
            io::println(fmt!("%u", num));
            sum += num;
        }
    }

    io::println(fmt!("answer: %u", sum - 1)); // remove 1
}
