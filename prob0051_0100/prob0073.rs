use common::problem::{ Problem };

pub static problem: Problem<'static> = Problem {
    id: 73,
    answer: "7295372",
    solver: solve
};

fn count_between(
    (na, da): (uint, uint), (nb, db): (uint, uint), max_d: uint
) -> uint {
    if da + db > max_d { return 0; }
    count_between((na, da), (na + nb, da + db), max_d) +
        count_between((na + nb, da + db), (nb, db), max_d) + 1
}

fn solve() -> ~str {
    let limit = 12000;
    let cnt = count_between((1, 3), (1, 2), limit);
    return cnt.to_str();
}
