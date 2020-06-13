#![allow(dead_code)]

mod reader;
mod records;

fn main() {
    //let mut rdr = reader::ContextReader::new().set_delimiter(b';').from_path("test.csv").unwrap();
    let mut rdr = reader::ContextReader::new().set_delimiter(b';').from_string("a,b,c,d\ne,f,g,h\n".as_bytes()).unwrap();
    for line in rdr.lines()  {
        println!("{:?}", line);
    }
}

/*
TODO App:
- Testing module
- Delete main


TODO Reader: 
 - Add from_string method
 - Add endline delimiter
 - Parse file with headers
 - Parse row with deserialization
 - Parse row on HashMap (by default)
 - Load file in data structure for filtering
 - Generic method to get lines by filtering (e.g : Return all lines that contains Yes for the column "Notification")

TODO testing:


 TODO Writer:
*/