use std::io::{self, BufReader};
use std::io::BufRead;

use crate::reader::Reader;

pub struct StringRecordIter<'a, T: 'a> {
    record: &'a mut Reader<T>,

}

impl<'a, T: std::io::Read> StringRecordIter<'a, T> {
    pub fn new(read_data: &'a mut Reader<T>) -> StringRecordIter<'a, T> {
        StringRecordIter {record: read_data}
    }
}

impl<'a, T: io::Read> Iterator for StringRecordIter<'a, T> {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        let mut line = String::default();
        match self.record.buf.read_line(&mut line) {
            Ok(0) => None,
            _ => Some(line.clone())
        }
    }
}