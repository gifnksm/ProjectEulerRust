use common::calc::{ each_triangles };
use common::reader::{ read_whole_word };
use common::problem::{ Problem };

pub static problem: Problem<'static> = Problem {
    id: 42,
    answer: "162",
    solver: solve
};

fn word_value(word: &str) -> uint {
    let mut value = 0;
    for word.each |b| {
        value += (b - ('A' as u8) + 1) as uint;
    }
    return value;
}

fn solve() -> ~str {
    let result = do io::read_whole_file_str(&Path("files/words.txt")).chain |input| {
        do read_whole_word(input).map |words| { words.map(|w| word_value(*w)) }
    };
    match result {
        result::Err(msg) => {
            fail!(msg)
        }
        result::Ok(values) => {
            let mut flag = vec::from_elem(values.max() + 1, false);
            for each_triangles |t| {
                if t >= flag.len() { break; }
                flag[t] = true;
            }

            let mut cnt = 0u;
            for values.each |v| {
                if flag[*v] { cnt += 1; }
            }
            return cnt.to_str();
        }
    }
}
