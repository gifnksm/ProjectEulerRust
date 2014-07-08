#![crate_name = "prob0085"]
#![crate_type = "rlib"]

pub static EXPECTED_ANSWER: &'static str = "2772";

// x by y => C = (1 + 2 + .. + x) * (1 + 2 + .. + y) recutangulars
//             = (x (1 + x) / 2) * (y (1 + y)) / 2
//             = xy (1 + x)(1 + y) / 4
fn count_rect((x, y): (uint, uint)) -> uint {
    x * y * (1 + x) * (1 + y) / 4
}

fn uint_diff(a: uint, target: uint) -> uint {
    if a > target { a - target } else { target - a }
}

fn min_idx(i_a: (uint, uint), i_b: (uint, uint), target: uint) -> (uint, uint) {
    let val_a = count_rect(i_a);
    let val_b = count_rect(i_b);

    let diff_a = uint_diff(val_a, target);
    let diff_b = uint_diff(val_b, target);
    if diff_a < diff_b {
        return i_a;
    }
    return i_b;
}

pub fn solve() -> String {
    let target = 2000000;

    let mut x = 1;
    let mut y = 1;

    let mut nearest = (0, 0);

    while count_rect((x, y)) < target { x += 1; y += 1; }
    while count_rect((x - 1, y)) >= target { x -= 1; }
    nearest = min_idx(nearest, (x, y), target);
    if y > 1 { nearest = min_idx(nearest, (x, y - 1), target); }
    y += 1;

    while x > 1 {
        let old_x = x;
        while count_rect((x - 1, y)) >= target { x -= 1; }
        if x != old_x {
            nearest = min_idx(nearest, (x, y), target);
            if y > 1 { nearest = min_idx(nearest, (x, y - 1), target); }
        }
        y += 1;
    }

    while count_rect((x, y + 1)) < target { y += 1; }
    nearest = min_idx(nearest, (x, y), target);

    let (x, y) = nearest;
    return (x * y).to_str();
}
