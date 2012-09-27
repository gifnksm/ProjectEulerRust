extern mod std;

use std::sort::{ quick_sort };

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

    for uint::range(0, 10) |d0| {
        let p0 = pows[d0];
        let s0 = p0;
        for uint::range(d0, 10) |d1| {
            let p1 = pows[d1];
            let s1 = s0 + p1;
            for uint::range(d1, 10) |d2| {
                let p2 = pows[d2];
                let s2 = s1 + p2;
                for uint::range(d2, 10) |d3| {
                    let p3 = pows[d3];
                    let s3 = s2 + p3;
                    for uint::range(d3, 10) |d4| {
                        let p4 = pows[d4];
                        let s4 = s3 + p4;
                        for uint::range(d4, 10) |d5| {
                            let p5 = pows[d5];
                            let s5 = s4 + p5;
                            for uint::range(d5, 10) |d6| {
                                let p6 = pows[d6];
                                let s6 = s5 + p6;
                                let mut itr = s6;
                                for uint::range(0, nums.len()) |i| {
                                    nums[i] = itr % 10;
                                    itr /= 10;
                                }
                                quick_sort(|a, b| a < b, nums);
                                if nums[0] == d0 &&
                                    nums[1] == d1 &&
                                    nums[2] == d2 &&
                                    nums[3] == d3 &&
                                    nums[4] == d4 &&
                                    nums[5] == d5 &&
                                    nums[6] == d6 {
                                    io::println(fmt!("%u", s6));
                                    sum += s6;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    io::println(fmt!("answer: %u", sum - 1)); // remove 1
}
