#![allow(dead_code)]

mod reader;
mod records;
mod utils;
mod error;

use crate::reader::ToCsv;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Record {
    city: String,
    region: String,
    country: String,
    population: String,
}

fn dump(arr: &[i32]) {
    println!("arr is {:?}", arr);
}

fn main() {
    //let mut rdr = reader::CsvReader::from().set_delimiter(',');
    let mut rdr = std::fs::File::open("test.csv").unwrap().parse_csv(';', true);
    println!("{:?}", rdr.headers().unwrap());
    //for line in rdr.lines()  {
    //    println!("{:?}", line);
    //}
    //println!("{}", rdr.current_position())
    let arr = [10, 20, 30];
    for i in arr.iter() {
        println!("{}", i);
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