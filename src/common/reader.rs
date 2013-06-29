use std::result;

fn skip_sep<'a>(input: &'a str) -> &'a str {
    let mut itr = input;
    while !itr.is_empty() {
        let (head, tail) = itr.slice_shift_char();
        if head != '\n' && head != ',' { return itr; }
        itr = tail;
    }
    return itr;
}

fn read_word<'a>(input: &'a str) -> Result<(&'a str, &'a str), ~str> {
    if input.is_empty() { return result::Err(~"string is empty"); }

    let (c, itr) = input.slice_shift_char();
    let mut c = c;
    let mut itr = itr;
    if c != '\"' { return result::Err(~"string does not start with `\"`"); }

    let mut len = 0;
    loop {
        let (new_c, new_itr) = itr.slice_shift_char();
        c   = new_c;
        itr = new_itr;
        if c == '\"' { break; }
        len += 1;
    }

    return result::Ok((input.slice(1, 1 + len), itr));
}

pub fn read_whole_word<'a>(input: &'a str) -> Result<~[&'a str], ~str> {
    let mut result = ~[];
    let mut itr = input;
    while !itr.is_empty() {
        match read_word(itr) {
            result::Ok((name, new_itr)) => {
                result.push(name);
                itr = skip_sep(new_itr);
            }
            result::Err(msg) => return result::Err(msg)
        }
    }
    return result::Ok(result);
}
