
mod reader;
mod records;

fn main() {
    let mut rdr = reader::ContextReader::new().set_delimiter(b';').from_path("test.csv").unwrap();
    for line in rdr.lines()  {
        println!("{:?}", line);
    }
}

/*


*/