use std::io::BufferedReader;
use std::str;

pub trait BufferedReaderUtil<R> {
    fn sep_iter<'a>(&'a mut self, c: u8) -> ReaderSplitIterator<'a, R>;
}
impl<R> BufferedReaderUtil<R> for BufferedReader<R> {
    #[inline]
    fn sep_iter<'a>(&'a mut self, c: u8) -> ReaderSplitIterator<'a, R> {
        ReaderSplitIterator { reader: self, sep_char: c, sep_flag: false }
    }
}

struct ReaderSplitIterator<'a, R> {
    priv reader: &'a mut BufferedReader<R>,
    priv sep_char: u8,
    priv sep_flag: bool
}
impl<'a, R: Reader> Iterator<~str> for ReaderSplitIterator<'a, R> {
    #[inline]
    fn next(&mut self) -> Option<~str> {
        self.reader
            .read_until(self.sep_char)
            .map(|mut bytes| {
                self.sep_flag = bytes.last() == Some(&self.sep_char);
                if self.sep_flag { bytes.pop(); }
                str::from_utf8_owned(bytes).unwrap()
            }).or_else(|| {
                if self.sep_flag {
                    self.sep_flag = false;
                    Some(~"")
                } else {
                    None
                }
            })
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
    if input.is_empty() { return Err(~"string is empty"); }

    let (c, itr) = input.slice_shift_char();
    let mut c = c;
    let mut itr = itr;
    if c != '\"' { return Err(~"string does not start with `\"`"); }

    let mut len = 0;
    loop {
        let (new_c, new_itr) = itr.slice_shift_char();
        c   = new_c;
        itr = new_itr;
        if c == '\"' { break; }
        len += 1;
    }

    return Ok((input.slice(1, 1 + len), itr));
}

pub fn read_whole_word<'a>(input: &'a str) -> Result<~[&'a str], ~str> {
    let mut result = ~[];
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
    return Ok(result);
}

#[cfg(test)]
mod test {
    mod sep_iter {
        use super::super::BufferedReaderUtil;
        use std::io::{BufferedReader, MemReader};

        fn buffered(bytes: ~[u8]) -> BufferedReader<MemReader> {
            BufferedReader::new(MemReader::new(bytes))
        }

        #[test]
        fn exclusive_non_trailing_sep() {
            let mut br = buffered(bytes!("a,bb,ccc").to_owned());
            let mut it = br.sep_iter(',' as u8);
            assert_eq!(Some(~"a"), it.next());
            assert_eq!(Some(~"bb"), it.next());
            assert_eq!(Some(~"ccc"), it.next());
            assert_eq!(None, it.next());
        }

        #[test]
        fn exclusive_trailing_sep() {
            let mut br = buffered(bytes!("a,bb,ccc,").to_owned());
            let mut it = br.sep_iter(',' as u8);
            assert_eq!(Some(~"a"), it.next());
            assert_eq!(Some(~"bb"), it.next());
            assert_eq!(Some(~"ccc"), it.next());
            assert_eq!(Some(~""), it.next());
            assert_eq!(None, it.next());
            assert_eq!(None, it.next());
        }
    }
}
