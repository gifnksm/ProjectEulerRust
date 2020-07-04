//! [Problem 19](https://projecteuler.net/problem=19) solver.

#![warn(
    bad_style,
    unused,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

fn is_leap_year(y: u32) -> bool {
    if y % 400 == 0 {
        return true;
    }
    if y % 100 == 0 {
        return false;
    }
    if y % 4 == 0 {
        return true;
    }
    false
}

fn day_of_year(y: u32) -> u32 {
    if is_leap_year(y) {
        366
    } else {
        365
    }
}

fn day_of_month(y: u32) -> [u32; 12] {
    [
        31,                                    // Jan
        if is_leap_year(y) { 29 } else { 28 }, // Feb
        31,                                    // Mar
        30,                                    // Apr
        31,                                    // May
        30,                                    // Jun
        31,                                    // Jul
        31,                                    // Aug
        30,                                    // Sep
        31,                                    // Oct
        30,                                    // Nov
        31,                                    /* Dec */
    ]
}

fn append_day(y: u32, offset: u32, result: &mut [u32; 7]) -> u32 {
    let mut day = offset;
    let dom = day_of_month(y);
    for n in &dom {
        result[day as usize] += 1;
        day = (day + *n) % 7;
    }
    day
}

fn compute() -> u32 {
    let mut result = [0; 7];
    let mut day = 1; // Monday
    day = (day + day_of_year(1900)) % 7;
    for y in 1901u32..(2000 + 1) {
        day = append_day(y, day, &mut result);
    }
    result[0]
}

fn solve() -> String {
    compute().to_string()
}

common::problem!("171", solve);
