fn skip_sep(input: &a/str) -> &a/str {
    let mut itr = input;
    while itr.is_not_empty() {
        let (head, tail) = str::view_shift_char(itr);
        if head != '\n' && head != ',' { return itr; }
        itr = tail;
    }
    return itr;
}

fn read_word(input: &a/str) -> Result<(&a/str, &a/str), ~str> {
    if input.is_empty() { return result::Err(~"string is empty"); }

    let mut (c, itr) = str::view_shift_char(input);
    if c != '\"' { return result::Err(~"string does not start with `\"`"); }

    let mut len = 0;
    loop {
        let (new_c, new_itr) = str::view_shift_char(itr);
        c   = new_c;
        itr = new_itr;
        if c == '\"' { break; }
        len += 1;
    }

    return result::Ok((str::view(input, 1, 1 + len), itr));
}

fn read_whole_word(input: &a/str) -> Result<~[&a/str], ~str> {
    let mut result = ~[];
    let mut itr = input;
    while itr.is_not_empty() {
        match move read_word(itr) {
            result::Ok((name, new_itr)) => {
                result.push(name);
                itr = skip_sep(new_itr);
            }
            result::Err(move msg) => return result::Err(msg)
        }
    }
    return result::Ok(result);
}
