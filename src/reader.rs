use std::io::{self, BufReader};
use std::fs::File;
use std::path::Path;

use crate::records::StringRecordIter;

#[derive(Debug, Copy, Clone)]
pub struct ContextReader {
    delimiter: u8,
    has_headers: bool,
}

impl ContextReader {
    pub fn new() -> ContextReader {
        ContextReader {delimiter: b',', has_headers: true}
    }

    pub fn set_delimiter(&mut self, delimiter: u8) -> &mut ContextReader {
        self.delimiter = delimiter;
        self
    }

    pub fn set_headers(&mut self, value: bool) -> &mut ContextReader {
        self.has_headers = value;
        self
    }
    
    pub fn from_path<P: AsRef<Path>>(&self, path: P) -> Result<Reader<File>, io::Error> {
        Ok(Reader::new(self, File::open(path)?))
    }

    pub fn from_string<R: io::Read>(&self, content: R) -> Result<Reader<R>, io::Error> {
        Ok(Reader::new(self, content))
    }
}


#[derive(Debug)]
pub struct Reader<T> {
    context: ContextReader,
    line: u64,
    pub buf: BufReader<T>
}

impl<T: std::io::Read> Reader<T> {
    pub fn new(context: &ContextReader, read_data: T) -> Reader<T> {
        Reader {context: context.clone(), line:0, buf: BufReader::new(read_data)}
    }

    pub fn lines(&mut self) -> StringRecordIter<T> {
        StringRecordIter::new(self)
    }
}

