
mod reader;

fn main() {
    let mut ctx = reader::ContextReader::new();
    let rdr = ctx.set_delimiter(b';').from_path("test.csv").unwrap();
    println!("{:?}", rdr);
}
