use std::rt::io::Reader;
use std::rt::io::buffered::BufferedReader;

trait BufferedReaderUtil<R> {
    fn line_iter<'a>(&'a mut self) -> ReaderLineIterator<'a, R>;
}
impl<R> BufferedReaderUtil<R> for BufferedReader<R> {
    fn line_iter<'a>(&'a mut self) -> ReaderLineIterator<'a, R> {
        ReaderLineIterator { reader: self }
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
