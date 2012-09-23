fn is_leap_year(y: uint) -> bool {
    if y % 400 == 0 { return true; }
    if y % 100 == 0 { return false; }
    if y % 4   == 0 { return true; }
    return false;
}

fn day_of_year(y: uint) -> uint {
    if is_leap_year(y) { 366 } else { 365 }
}

fn day_of_month(y: uint) -> [uint * 12] {
    [ 31, // Jan
     if is_leap_year(y) { 29 } else { 28 }, // Feb
     31, // Mar
     30, // Apr
     31, // May
     30, // Jun
     31, // Jul
     31, // Aug
     30, // Sep
     31, // Oct
     30, // Nov
     31  // Dec
    ] 
}

fn append_day(y: uint, offset: uint, result: &[mut uint * 7]) -> uint {
    let mut day = offset;
    for day_of_month(y).each |n| {
        result[day] += 1;
        day = (day + *n) % 7;
    }
    return day;
}

fn main() {
    let result = [mut 0, 0, 0, 0, 0, 0, 0];
    let mut day = 1; // Monday
    day = (day + day_of_year(1900)) % 7;
    for uint::range(1901, 2000 + 1) |y| {
        day = append_day(y, day, &result);
        io::println(fmt!("%? %?", result, day));
    }
    io::println(fmt!("%u", result[0]));
}