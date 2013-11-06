use std::result;

use std::io::buffered::BufferedReader;
use std::str;

trait BufferedReaderUtil<R> {
    fn line_iter<'a>(&'a mut self) -> ReaderLineIterator<'a, R>;
    fn sep_iter<'a>(&'a mut self, c: u8) -> ReaderSplitIterator<'a, R>;
}
impl<R> BufferedReaderUtil<R> for BufferedReader<R> {
    fn line_iter<'a>(&'a mut self) -> ReaderLineIterator<'a, R> {
        ReaderLineIterator { reader: self }
    }
    fn sep_iter<'a>(&'a mut self, c: u8) -> ReaderSplitIterator<'a, R> {
        ReaderSplitIterator { reader: self, c: c }
    }
}

struct ReaderLineIterator<'self, R> {
    priv reader: &'self mut BufferedReader<R>
}
impl<'self, R: Reader> Iterator<~str> for ReaderLineIterator<'self, R> {
    fn next(&mut self) -> Option<~str> {
        self.reader.read_line()
    }
}

struct ReaderSplitIterator<'self, R> {
    priv reader: &'self mut BufferedReader<R>,
    priv c: u8
}
impl<'self, R: Reader> Iterator<~str> for ReaderSplitIterator<'self, R> {
    fn next(&mut self) -> Option<~str> {
        if self.reader.eof() {
            None
        } else {
            self.reader.read_until(self.c).map(str::from_utf8_owned)
        }
    }
}

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
