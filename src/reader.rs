use std::io::{self, BufReader};
use std::fs::File;
use std::path::Path;
use std::io::{BufRead};

use crate::records::StringRecordIter;
use crate::error::UnauthorizedHeaderError;


#[derive(Debug, Copy, Clone)]
pub struct ContextReader {
    delimiter: char,
    has_headers: bool,
}

impl ContextReader {
    pub fn new() -> ContextReader {
        ContextReader {delimiter: ',', has_headers: false}
    }

    pub fn set_delimiter(&mut self, delimiter: char) -> &mut ContextReader {
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

    pub fn from_read<R: io::Read>(&self, content: R) -> Result<Reader<R>, io::Error> {
        Ok(Reader::new(self, content))
    }
}


#[derive(Debug)]
pub struct Reader<T> {
    context: ContextReader,
    line: u64,
    pub buf: BufReader<T>,
    _headers: Option<Vec<String>> // Move into another sruct
}

impl<T: std::io::Read> Reader<T> {
    pub fn new(context: &ContextReader, read_data: T) -> Reader<T> {
        Reader {context: context.clone(), line:0, buf: BufReader::new(read_data), _headers: None}
    }

    pub fn lines(&mut self) -> StringRecordIter<T> {
        StringRecordIter::new(self)
    }

    pub fn read_line(&mut self) -> Option<String>  {
        // TODO Change it to generic read methods
        let mut line = String::default();
        match self.buf.read_line(&mut line) {
            Ok(0) => None,
            _ => Some(line.clone())
        }
    }

   pub fn headers(&mut self) -> Result<Vec<String>, UnauthorizedHeaderError> {
        if self.context.has_headers == false {
            return Err(UnauthorizedHeaderError);
        }
        if self._headers.is_none() {
            let line = self.read_line().unwrap().clone();
            let s = line.split(self.context.delimiter).map(|s| s.to_string());
            self._headers = Some(s.collect::<Vec<String>>()).clone();
            //self.buf.seek(SeekFrom::Start(0));
        }
        Ok(self._headers.as_ref().unwrap().clone())
    }
}


