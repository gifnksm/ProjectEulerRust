use common::problem::{ Problem };

pub static problem: Problem<'static> = Problem {
    id: 99,
    answer: "709",
    solver: solve
};

fn solve() -> ~str {
    let result = io::file_reader(&Path("files/base_exp.txt"))
        .map(|input| {
            let mut line_idx = 1u;
            let mut max = 0f;
            let mut max_idx = 1;
            for input.each_line |line| {
                for str::find_char(line, ',').each |&idx| {
                    let base = float::from_str(line.slice(0, idx)).get();
                    let exp  = float::from_str(line.slice(idx + 1, line.len())).get();
                    let ln = exp * base.ln();
                    if ln > max {
                        max = ln;
                        max_idx = line_idx;
                    }
                    line_idx += 1;
                }
            }
            max_idx
        });

    match result {
        Err(msg) => fail!(msg),
        Ok(value) => return value.to_str()
    }
}
