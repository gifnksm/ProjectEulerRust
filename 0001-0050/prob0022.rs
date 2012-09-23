export mod std;

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

fn main() {
    match io::read_whole_file_str(&Path("files/names.txt")) {
        result::Err(msg) => {
            (io::stderr() as io::WriterUtil).write_str(fmt!("%?\n", msg));
            return;
        }
        result::Ok(input) => {
            let names = read_whole_name(input);
            std::sort::merge_sort(str::le, names);
            io::println(fmt!("%?", read_whole_name(input)));
        }
    }
}