use std::fs::File;
use std::io::Read;

mod day_one;
mod day_two;
mod day_three;

pub fn read_from_file(input: &str) -> String {
    let mut content = String::new();
    let mut file = File::open(input).unwrap();
    file.read_to_string(&mut content).unwrap();
    content
}
