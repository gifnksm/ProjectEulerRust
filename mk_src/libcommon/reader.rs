fn skip_sep<'a>(mut input: &'a str) -> &'a str {
    loop {
        let (head, tail) = input.slice_shift_char();
        match head {
            Some('\n') | Some(',') => input = tail,
            _ => return input
        }
    }
}

fn read_word<'a>(input: &'a str) -> Result<(&'a str, &'a str), String> {
    if input.is_empty() { return Err("string is empty".to_string()); }

    let (c, itr) = input.slice_shift_char();
    if c != Some('\"') { return Err("string does not start with `\"`".to_string()) }

    let mut itr = itr;
    let mut len = 0;
    loop {
        let (head, tail) = itr.slice_shift_char();
        itr = tail;
        if head == Some('\"') { break; }
        if head == None { return Err("string does not contains double `\"`".to_string()) }
        len += 1;
    }

    Ok((input.slice(1, 1 + len), itr))
}

pub fn read_whole_word<'a>(input: &'a str) -> Result<Vec<&'a str>, String> {
    let mut result = Vec::new();
    let mut itr = input;
    while !itr.is_empty() {
        match read_word(itr) {
            Ok((name, new_itr)) => {
                result.push(name);
                itr = skip_sep(new_itr);
            }
            Err(msg) => return Err(msg)
        }
    }
    Ok(result)
}
