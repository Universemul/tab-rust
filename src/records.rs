use crate::reader::CsvReader;

pub struct RecordIter<'a> {
    record: &'a mut CsvReader,
}

impl<'a> RecordIter<'a> {
    pub fn new(read_data: &'a mut CsvReader) -> RecordIter<'a> {
        RecordIter {record: read_data}
    }
}

impl<'a> Iterator for RecordIter<'a> {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        self.record.read_line()
    }
}