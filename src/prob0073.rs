#![crate_id = "prob0073"]
#![crate_type = "rlib"]

pub static EXPECTED_ANSWER: &'static str = "7295372";

fn count_between(
    (na, da): (uint, uint), (nb, db): (uint, uint), max_d: uint
) -> uint {
    if da + db > max_d { return 0; }
    count_between((na, da), (na + nb, da + db), max_d) +
        count_between((na + nb, da + db), (nb, db), max_d) + 1
}

pub fn solve() -> String {
    let limit = 12000;
    let cnt = count_between((1, 3), (1, 2), limit);
    return cnt.to_str();
}
