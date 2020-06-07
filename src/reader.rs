use std::io::{self, BufReader};
use std::fs::File;
use std::path::Path;

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
    
    pub fn from_path<P: AsRef<Path>>(&self, path: P) -> Result<Reader<File>, io::Error> {
        Ok(Reader::new(self, File::open(path)?))
    }
}
#[derive(Debug)]
pub struct Reader<R> {
    context: ContextReader,
    line: u64,
    data: BufReader<R>
}

impl<R: io::Read> Reader<R> {
    pub fn new(context: &ContextReader, read_data: R) -> Reader<R> {
        Reader {context: *context, line:0, data: BufReader::new(read_data)}
        //TODO: Clone ContextReader to avoid bad ownership
    }
}

