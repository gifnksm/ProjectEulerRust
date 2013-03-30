use common::calc::{ digit_histogram };
use common::problem::{ Problem };

pub static problem: Problem<'static> = Problem {
    id: 52,
    answer: "142857",
    solver: solve
};

fn solve() -> ~str {
    let mut n = 0;
    let mut order = 0;
    let mut limit = 0;

    loop {
        n += 1;
        if n > limit {
            // 6倍しても桁が変わらない数だけ調べる
            n = order;
            order *= 10;
            limit = (order - 1) / 6;
            loop;
        }

        let ds = digit_histogram(n);

        // n * 5 の時に、必ず 0 または 5 は含むため
        if ds[0] == 0 && ds[5] == 0 { loop; }

        // n * 2, n * 4 の時に、必ず偶数は含むため
        if ds[0] == 0 && ds[2] == 0 && ds[4] == 0 && ds[6] == 0 && ds[8] == 0 {
            loop;
        }

        if ds != digit_histogram(n * 2) { loop; }
        if ds != digit_histogram(n * 3) { loop; }
        if ds != digit_histogram(n * 4) { loop; }
        if ds != digit_histogram(n * 5) { loop; }
        if ds != digit_histogram(n * 6) { loop; }

        return n.to_str();
    }
}

