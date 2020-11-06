use std::io::BufReader;
use std::io::{BufRead};
use std::io::{Seek, SeekFrom};

use crate::records::RecordIter;
use crate::utils::Position;
use crate::error::UnauthorizedHeaderError;


#[derive(Debug)]
pub struct CsvReader {
    pub buffer: BufReader<std::fs::File>,
    pub delimiter: char,
    pub has_headers: bool,
    pub in_memory: bool,
    position: Position,
    _headers: Option<Vec<String>>
}

pub trait ToCsv {
     fn parse_csv(self, delimiter: char, has_headers: bool) -> CsvReader;
}


impl ToCsv for std::fs::File{
    fn parse_csv(self, delimiter: char, has_headers: bool) -> CsvReader{
        let mut rdr = CsvReader{
            delimiter: delimiter, 
            has_headers: has_headers, 
            _headers: None, 
            buffer: BufReader::new(self), 
            in_memory: false,
            position: Position::new(),
        };
        if has_headers == true {
            rdr.set_headers(None);
        }
        rdr
    }
}

impl CsvReader {

    pub fn lines(&mut self) -> RecordIter {
        RecordIter::new(self)
    }

    pub fn set_headers(&mut self, line: Option<String>) -> &mut Self {
        if self._headers.is_none() {
            let l = match line {
                None => {
                    self.position.set_line(self.position.line() - 1);
                    self.read_line().unwrap().clone()
                },
                Some(e) => e
            };
            let s = l.split(self.delimiter).map(|s| s.to_string());
            self._headers = Some(s.collect::<Vec<String>>()).clone();
        }
        self
    }

    pub fn headers(&mut self) -> Result<Vec<String>, UnauthorizedHeaderError> {
        if self.has_headers == false {
            return Err(UnauthorizedHeaderError);
        }
        if self._headers.is_none() {
            let line = self.read_line().unwrap().clone();
            self.set_headers(Some(line));
        }
        Ok(self._headers.as_ref().unwrap().clone())
    }

    pub fn read_line(&mut self) -> Option<String>  {
        let mut line = String::default();
        match self.buffer.read_line(&mut line) {
            Ok(0) => {
                None
            },
            _ => {
                self.position.set_line(self.position.line() + 1);
                Some(line.trim().to_string())
            }
        }
    }

    pub fn current_position(self) -> u64{
        self.position.line()
    }

    /*pub fn reset(&mut self) -> &mut Self {
        self.position.reset();
        self.buffer.seek(SeekFrom::Start(0));
        self
    }
    */
}