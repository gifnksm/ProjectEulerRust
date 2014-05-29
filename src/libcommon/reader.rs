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

pub struct ReaderSplitIterator<'a, R> {
    reader: &'a mut BufferedReader<R>,
    sep_char: u8,
    sep_flag: bool
}
impl<'a, R: Reader> Iterator<String> for ReaderSplitIterator<'a, R> {
    #[inline]
    fn next(&mut self) -> Option<String> {
        self.reader
            .read_until(self.sep_char)
            .ok()
            .map(|mut bytes| {
                self.sep_flag = bytes.last() == Some(&self.sep_char);
                if self.sep_flag { bytes.pop(); }
                str::from_utf8_owned(bytes).unwrap()
            }).or_else(|| {
                if self.sep_flag {
                    self.sep_flag = false;
                    Some("".to_string())
                } else {
                    None
                }
            })
    }
}

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

pub fn read_whole_word<'a>(input: &'a str) -> Result<~[&'a str], String> {
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
    Ok(result.as_slice().to_owned())
}

#[cfg(test)]
mod tests {
    mod sep_iter {
        use super::super::BufferedReaderUtil;
        use std::io::{BufferedReader, MemReader};

        fn buffered(bytes: &[u8]) -> BufferedReader<MemReader> {
            BufferedReader::new(MemReader::new(Vec::from_slice(bytes)))
        }

        #[test]
        fn exclusive_non_trailing_sep() {
            let mut br = buffered(bytes!("a,bb,ccc"));
            let mut it = br.sep_iter(',' as u8);
            assert_eq!(Some("a".to_string()), it.next());
            assert_eq!(Some("bb".to_string()), it.next());
            assert_eq!(Some("ccc".to_string()), it.next());
            assert_eq!(None, it.next());
        }

        #[test]
        fn exclusive_trailing_sep() {
            let mut br = buffered(bytes!("a,bb,ccc,"));
            let mut it = br.sep_iter(',' as u8);
            assert_eq!(Some("a".to_string()), it.next());
            assert_eq!(Some("bb".to_string()), it.next());
            assert_eq!(Some("ccc".to_string()), it.next());
            assert_eq!(Some("".to_string()), it.next());
            assert_eq!(None, it.next());
            assert_eq!(None, it.next());
        }
    }
}
