extern mod std;

fn skip_sep(input: &a/str) -> &a/str {
    let mut itr = input;
    while itr.is_not_empty() && (itr[0] == '\n' as u8 || itr[0] == ',' as u8) {
        itr = str::view_shift_char(itr).second();
    }
    return itr;
}

fn read_name(input: &a/str) -> Option<(&a/str, &a/str)> {
    let mut (c, itr) = str::view_shift_char(input);
    if c != '\"' { return None; }

    let mut len = 0;
    loop {
        let (new_c, new_itr) = str::view_shift_char(itr);
        c = new_c;
        itr = new_itr;
        if c == '\"' { break; }
        len += 1;
    }

    return Some((str::view(input, 1, 1 + len), itr));
}

fn read_whole_name(input: &a/str) -> Option<~[&a/str]> {
    let mut result = ~[];
    let mut itr = input;
    while itr.is_not_empty() {
        match read_name(itr) {
            Some((name, new_itr)) => {
                result += [name];
                itr = skip_sep(new_itr);
            }
            None => return None
        }
    }
    return Some(result);
}

fn get_score(pair: &(uint, &str)) -> uint {
    let (n, s) = *pair;
    n * str::as_bytes_slice(s).map(|c| *c - ('A' as u8) + 1).foldl(0 as uint, |s, e| *s + *e as uint)
}

fn main() {
    match io::read_whole_file_str(&Path("files/names.txt")) {
        result::Err(msg) => {
            (io::stderr() as io::WriterUtil).write_str(fmt!("%?\n", msg));
            return;
        }
        result::Ok(input) => {
            match read_whole_name(input) {
                None => {
                    (io::stderr() as io::WriterUtil).write_str("Error!\n");
                    return;
                }
                Some(names) => {
                    let sorted = std::sort::merge_sort(|a, b| a < b, names).mapi(|i, s| (i + 1, *s));
                    let mut total_score = 0;
                    for sorted.each |s| {
                        total_score += get_score(s);
                    }
                    io::println(fmt!("%u", total_score));
                }
            }
        }
    }
}