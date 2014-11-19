fn skip_sep<'a>(mut input: &'a str) -> &'a str {
    loop {
        match input.slice_shift_char() {
            Some(('\n', tail)) | Some((',', tail)) => input = tail,
            _ => return input
        }
    }
}

fn read_word<'a>(input: &'a str) -> Result<(&'a str, &'a str), String> {
    if input.is_empty() { return Err("string is empty".to_string()); }

    let (_, itr) = match input.slice_shift_char() {
        Some((c, itr)) if c == '\"' => (c, itr),
        _ => { return Err("string does not start with `\"`".to_string()) }
    };

    let mut itr = itr;
    let mut len = 0;
    loop {
        if let Some((head, tail)) = itr.slice_shift_char() {
            itr = tail;
            if head == '\"' { break; }
            len += 1;
        } else {
            return Err("string does not contains double `\"`".to_string())
        }
    }

    Ok((input[1 .. 1 + len], itr))
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
