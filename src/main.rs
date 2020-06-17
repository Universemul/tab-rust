#![allow(dead_code)]

mod reader;
mod records;
mod error;

fn main() {
    //let mut rdr = reader::ContextReader::new().set_delimiter(b';').from_path("test.csv").unwrap();
    let mut rdr = reader::ContextReader::new().set_delimiter(';').set_headers(true).from_read("a,b,c,d\ne,f,g,h\n".as_bytes()).unwrap();
    let headers = rdr.headers().unwrap();
    println!("{:?}", headers);
    for line in rdr.lines()  {
        println!("{:?}", line);
    }
}

/*
TODO App:
- Testing module
- Delete main


TODO Reader: 
 - Parse headers
 - Parse row with deserialization
 - Parse row on HashMap (by default)
 - Load file in data structure for filtering
 - Generic method to get lines by filtering (e.g : Return all lines that contains Yes for the column "Notification")

TODO testing:


 TODO Writer:
*/