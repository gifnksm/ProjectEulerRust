use core::io::{WriterUtil};

extern mod euler;
use euler::calc::{ each_triangles };
use euler::reader::{ read_whole_word };

fn word_value(word: &str) -> uint {
    let mut value = 0;
    for word.each |b| {
        value += (b - ('A' as u8) + 1) as uint;
    }
    return value;
}

fn main() {
    let result = do io::read_whole_file_str(&Path("files/words.txt")).chain |input| {
        do read_whole_word(input).map |words| { words.map(|w| word_value(*w)) }
    };
    match result {
        result::Err(msg) => {
            io::stderr().write_str(fmt!("%s\n", msg));
            fail!()
        }
        result::Ok(values) => {
            let mut flag = vec::from_elem(values.max() + 1, false);
            for each_triangles |t| {
                if t >= flag.len() { break; }
                flag[t] = true;
            }

            let mut cnt = 0;
            for values.each |v| {
                if flag[*v] { cnt += 1; }
            }
            io::println(fmt!("answer: %u", cnt));
        }
    }
}
